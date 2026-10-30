use std::{
    net::Shutdown,
    os::unix::net::UnixStream,
    sync::mpsc::{self, Receiver, RecvTimeoutError},
    thread,
    time::Duration,
};

use agent_wayland_proxy::client_connection;

use super::FakeCompositor;
use wayland_client::{
    Connection, Dispatch, DispatchError, EventQueue, Proxy, QueueHandle, delegate_noop,
    globals::{GlobalList, GlobalListContents, registry_queue_init},
    protocol::{
        wl_compositor::WlCompositor, wl_data_device_manager::WlDataDeviceManager,
        wl_registry::WlRegistry, wl_seat::WlSeat, wl_shm::WlShm, wl_surface::WlSurface,
    },
};
use wayland_protocols::xdg::shell::client::{
    xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel, xdg_wm_base::XdgWmBase,
};

const WAIT_LIMIT: Duration = Duration::from_secs(5);

pub struct AgentClient {
    socket: UnixStream,
    connection: Connection,
    globals: GlobalList,
    queue: EventQueue<ClientState>,
    state: ClientState,
    proxy_ending: Receiver<anyhow::Result<()>>,
}

impl AgentClient {
    pub fn connect(compositor: &FakeCompositor) -> Self {
        let (agent_end, proxy_end) = UnixStream::pair().expect("a socket pair");
        let compositor_end = compositor.connect();
        let (proxy_ended, proxy_ending) = mpsc::channel();
        thread::spawn(move || {
            let _ = proxy_ended.send(client_connection::serve(proxy_end, compositor_end));
        });
        let socket = agent_end
            .try_clone()
            .expect("a second handle of the agent socket");
        let connection = Connection::from_socket(agent_end).expect("a client connection");
        let (globals, queue) = within_wait_limit(&socket, || registry_queue_init(&connection))
            .expect("the first globals");
        Self {
            socket,
            connection,
            globals,
            queue,
            state: ClientState,
            proxy_ending,
        }
    }

    pub fn offered(&self) -> Vec<(u32, String)> {
        self.globals
            .contents()
            .clone_list()
            .into_iter()
            .map(|global| (global.name, global.interface))
            .collect()
    }

    pub fn offered_name_of(&self, interface: &str) -> u32 {
        self.offered()
            .into_iter()
            .find_map(|(name, offered)| (offered == interface).then_some(name))
            .expect("an offered global of that interface")
    }

    pub fn bind<I>(&self, name: u32, version: u32) -> I
    where
        I: Proxy + 'static,
        ClientState: Dispatch<I, ()>,
    {
        self.globals
            .registry()
            .bind(name, version, &self.queue.handle(), ())
    }

    pub fn open_window(&self) -> XdgToplevel {
        let handle = self.queue.handle();
        let compositor: WlCompositor = self.globals.bind(&handle, 1..=6, ()).expect("a compositor");
        let wm_base: XdgWmBase = self.globals.bind(&handle, 1..=6, ()).expect("a wm base");
        let surface = compositor.create_surface(&handle, ());
        wm_base
            .get_xdg_surface(&surface, &handle, ())
            .get_toplevel(&handle, ())
    }

    pub fn roundtrip(&mut self) -> Result<usize, DispatchError> {
        let Self {
            socket,
            queue,
            state,
            ..
        } = self;
        within_wait_limit(socket, || queue.roundtrip(state))
    }

    pub fn disconnect(self) -> anyhow::Result<()> {
        let Self {
            socket,
            connection,
            globals,
            queue,
            proxy_ending,
            ..
        } = self;
        drop((globals, queue, connection, socket));
        proxy_ending
            .recv_timeout(WAIT_LIMIT)
            .expect("the proxy code to end the connection within the wait limit")
    }
}

fn within_wait_limit<T>(socket: &UnixStream, wait: impl FnOnce() -> T) -> T {
    let socket = socket
        .try_clone()
        .expect("a handle of the agent socket for the wait limit");
    let (finished, finishing) = mpsc::channel::<()>();
    let watchdog = thread::spawn(move || {
        let expired = finishing.recv_timeout(WAIT_LIMIT) == Err(RecvTimeoutError::Timeout);
        if expired {
            let _ = socket.shutdown(Shutdown::Both);
        }
        expired
    });
    let result = wait();
    drop(finished);
    let expired = watchdog.join().expect("the wait limit thread");
    assert!(!expired, "the wait did not end within {WAIT_LIMIT:?}");
    result
}

pub struct ClientState;

impl Dispatch<WlRegistry, GlobalListContents> for ClientState {
    fn event(
        _state: &mut Self,
        _registry: &WlRegistry,
        _event: <WlRegistry as Proxy>::Event,
        _data: &GlobalListContents,
        _connection: &Connection,
        _handle: &QueueHandle<Self>,
    ) {
    }
}

delegate_noop!(ClientState: ignore WlCompositor);
delegate_noop!(ClientState: ignore WlSurface);
delegate_noop!(ClientState: ignore WlShm);
delegate_noop!(ClientState: ignore WlSeat);
delegate_noop!(ClientState: ignore WlDataDeviceManager);
delegate_noop!(ClientState: ignore XdgWmBase);
delegate_noop!(ClientState: ignore XdgSurface);
delegate_noop!(ClientState: ignore XdgToplevel);
