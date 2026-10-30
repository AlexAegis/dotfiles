use std::{
    os::unix::net::UnixStream,
    panic,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use wayland_protocols::xdg::shell::server::{
    xdg_surface::{self, XdgSurface},
    xdg_toplevel::{self, XdgToplevel},
    xdg_wm_base::{self, XdgWmBase},
};
use wayland_server::{
    Client, DataInit, Dispatch, Display, DisplayHandle, GlobalDispatch, New, Resource,
    backend::{ClientData, ClientId, DisconnectReason, GlobalId},
    protocol::{
        wl_compositor::{self, WlCompositor},
        wl_data_device_manager::WlDataDeviceManager,
        wl_output::WlOutput,
        wl_seat::WlSeat,
        wl_shm::WlShm,
        wl_surface::WlSurface,
    },
};

const POLL_INTERVAL: Duration = Duration::from_millis(1);
const WAIT_LIMIT: Duration = Duration::from_secs(5);

pub struct FakeCompositor {
    handle: DisplayHandle,
    received: Arc<Mutex<ReceivedRequests>>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

#[derive(Clone, Default)]
pub struct ReceivedRequests {
    pub titles: Vec<String>,
    pub app_ids: Vec<String>,
    pub disconnected_clients: usize,
}

impl FakeCompositor {
    pub fn start() -> Self {
        let display = Display::<CompositorState>::new().expect("a display");
        let handle = display.handle();
        handle.create_global::<CompositorState, WlCompositor, ()>(6, ());
        handle.create_global::<CompositorState, WlDataDeviceManager, ()>(3, ());
        handle.create_global::<CompositorState, WlShm, ()>(1, ());
        handle.create_global::<CompositorState, WlSeat, ()>(7, ());
        handle.create_global::<CompositorState, XdgWmBase, ()>(6, ());
        let received = Arc::new(Mutex::new(ReceivedRequests::default()));
        let state = CompositorState {
            received: received.clone(),
        };
        let stop = Arc::new(AtomicBool::new(false));
        let thread = thread::spawn({
            let stop = stop.clone();
            move || serve(display, state, &stop)
        });
        Self {
            handle,
            received,
            stop,
            thread: Some(thread),
        }
    }

    pub fn connect(&self) -> UnixStream {
        let (proxy_end, compositor_end) = UnixStream::pair().expect("a socket pair");
        self.handle
            .clone()
            .insert_client(
                compositor_end,
                Arc::new(TestClient {
                    received: self.received.clone(),
                }),
            )
            .expect("an inserted client");
        proxy_end
    }

    pub fn add_clipboard_global(&self) -> GlobalId {
        self.handle
            .create_global::<CompositorState, WlDataDeviceManager, ()>(3, ())
    }

    pub fn add_output_global(&self) -> GlobalId {
        self.handle
            .create_global::<CompositorState, WlOutput, ()>(4, ())
    }

    pub fn remove_global(&self, global: GlobalId) {
        self.handle.remove_global::<CompositorState>(global);
    }

    pub fn received(&self) -> ReceivedRequests {
        self.received.lock().expect("the received requests").clone()
    }

    pub fn received_after_disconnect(&self) -> ReceivedRequests {
        let deadline = Instant::now() + WAIT_LIMIT;
        loop {
            let received = self.received();
            if received.disconnected_clients > 0 {
                return received;
            }
            assert!(
                Instant::now() < deadline,
                "no client disconnected within {WAIT_LIMIT:?}"
            );
            thread::sleep(POLL_INTERVAL);
        }
    }
}

impl Drop for FakeCompositor {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(thread) = self.thread.take()
            && let Err(panic) = thread.join()
            && !thread::panicking()
        {
            panic::resume_unwind(panic);
        }
    }
}

fn serve(mut display: Display<CompositorState>, mut state: CompositorState, stop: &AtomicBool) {
    while !stop.load(Ordering::Acquire) {
        display
            .dispatch_clients(&mut state)
            .expect("dispatched requests");
        display.flush_clients().expect("flushed events");
        thread::sleep(POLL_INTERVAL);
    }
}

struct TestClient {
    received: Arc<Mutex<ReceivedRequests>>,
}

impl ClientData for TestClient {
    fn disconnected(&self, _client_id: ClientId, _reason: DisconnectReason) {
        self.received
            .lock()
            .expect("the received requests")
            .disconnected_clients += 1;
    }
}

struct CompositorState {
    received: Arc<Mutex<ReceivedRequests>>,
}

impl CompositorState {
    fn record(&self, record: impl FnOnce(&mut ReceivedRequests)) {
        record(&mut self.received.lock().expect("the received requests"));
    }
}

impl<I> GlobalDispatch<I, ()> for CompositorState
where
    I: Resource + 'static,
    CompositorState: Dispatch<I, ()>,
{
    fn bind(
        _state: &mut Self,
        _handle: &DisplayHandle,
        _client: &Client,
        resource: New<I>,
        _global_data: &(),
        data_init: &mut DataInit<'_, Self>,
    ) {
        data_init.init(resource, ());
    }
}

impl Dispatch<WlCompositor, ()> for CompositorState {
    fn request(
        _state: &mut Self,
        _client: &Client,
        _compositor: &WlCompositor,
        request: wl_compositor::Request,
        _data: &(),
        _handle: &DisplayHandle,
        data_init: &mut DataInit<'_, Self>,
    ) {
        if let wl_compositor::Request::CreateSurface { id } = request {
            data_init.init(id, ());
        }
    }
}

impl Dispatch<XdgWmBase, ()> for CompositorState {
    fn request(
        _state: &mut Self,
        _client: &Client,
        _wm_base: &XdgWmBase,
        request: xdg_wm_base::Request,
        _data: &(),
        _handle: &DisplayHandle,
        data_init: &mut DataInit<'_, Self>,
    ) {
        if let xdg_wm_base::Request::GetXdgSurface { id, .. } = request {
            data_init.init(id, ());
        }
    }
}

impl Dispatch<XdgSurface, ()> for CompositorState {
    fn request(
        _state: &mut Self,
        _client: &Client,
        _surface: &XdgSurface,
        request: xdg_surface::Request,
        _data: &(),
        _handle: &DisplayHandle,
        data_init: &mut DataInit<'_, Self>,
    ) {
        if let xdg_surface::Request::GetToplevel { id } = request {
            data_init.init(id, ());
        }
    }
}

impl Dispatch<XdgToplevel, ()> for CompositorState {
    fn request(
        state: &mut Self,
        _client: &Client,
        _toplevel: &XdgToplevel,
        request: xdg_toplevel::Request,
        _data: &(),
        _handle: &DisplayHandle,
        _data_init: &mut DataInit<'_, Self>,
    ) {
        match request {
            xdg_toplevel::Request::SetTitle { title } => {
                state.record(|received| received.titles.push(title));
            }
            xdg_toplevel::Request::SetAppId { app_id } => {
                state.record(|received| received.app_ids.push(app_id));
            }
            _ => {}
        }
    }
}

macro_rules! ignore_requests {
    ($($interface:ty),+) => {$(
        impl Dispatch<$interface, ()> for CompositorState {
            fn request(
                _state: &mut Self,
                _client: &Client,
                _resource: &$interface,
                _request: <$interface as Resource>::Request,
                _data: &(),
                _handle: &DisplayHandle,
                _data_init: &mut DataInit<'_, Self>,
            ) {
            }
        }
    )+};
}

ignore_requests!(WlSurface, WlShm, WlSeat, WlDataDeviceManager, WlOutput);
