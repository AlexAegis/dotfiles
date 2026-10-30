use std::{
    cell::Cell,
    fs,
    io::{self, PipeWriter},
    os::{fd::OwnedFd, unix::net::UnixListener},
    path::Path,
    rc::Rc,
};

use anyhow::{Context, Result};
use wl_proxy::{
    protocols::{
        ObjectInterface,
        security_context_v1::wp_security_context_manager_v1::WpSecurityContextManagerV1,
        wayland::{
            wl_callback::{WlCallback, WlCallbackHandler},
            wl_registry::{WlRegistry, WlRegistryHandler},
        },
    },
    state::State,
};

use crate::{BASELINE, state_error};

const SANDBOX_ENGINE: &str = "org.alexaegis.agent-wayland";
const SANDBOX_APP_ID: &str = "agent";

/// A KWin security context that listens on `socket_path`. KWin stops listening when
/// this value drops, which closes the close fd of the context.
pub struct RestrictedSocket {
    state: Rc<State>,
    _close_writer: PipeWriter,
}

impl RestrictedSocket {
    /// Connects to KWin through `WAYLAND_DISPLAY`.
    pub fn create(socket_path: &Path) -> Result<Self> {
        let (state, close_writer) = create(socket_path)?;
        Ok(Self {
            state,
            _close_writer: close_writer,
        })
    }

    /// Dispatches the KWin connection until it fails.
    #[must_use]
    pub fn keep_alive(&self) -> anyhow::Error {
        keep_alive(&self.state)
    }
}

fn create(socket_path: &Path) -> Result<(Rc<State>, PipeWriter)> {
    let state = State::builder(BASELINE).build().map_err(|error| {
        state_error::describe(&error).context("Connection to the compositor failed")
    })?;
    let display = state.display();
    let registry = display.new_send_get_registry();
    let manager_name = Rc::new(Cell::new(None));
    registry.set_handler(ManagerFinder(manager_name.clone()));
    roundtrip(&state, &display)?;
    let name = manager_name
        .get()
        .context("Compositor offers no wp_security_context_manager_v1")?;
    let manager = state.create_object::<WpSecurityContextManagerV1>(1);
    registry.send_bind(name, manager.clone());

    match fs::remove_file(socket_path) {
        Err(error) if error.kind() != io::ErrorKind::NotFound => {
            return Err(error)
                .with_context(|| format!("Stale {} not removed", socket_path.display()));
        }
        _ => {}
    }
    let listener = UnixListener::bind(socket_path)
        .with_context(|| format!("Bind of {} failed", socket_path.display()))?;
    let (close_reader, close_writer) = io::pipe().context("Pipe for the close fd failed")?;
    let context = manager.new_send_create_listener(
        &Rc::new(OwnedFd::from(listener)),
        &Rc::new(OwnedFd::from(close_reader)),
    );
    context.send_set_sandbox_engine(SANDBOX_ENGINE);
    context.send_set_app_id(SANDBOX_APP_ID);
    context.send_commit();
    context.send_destroy();
    roundtrip(&state, &display).context("Compositor refused the security context")?;
    Ok((state, close_writer))
}

fn keep_alive(state: &Rc<State>) -> anyhow::Error {
    loop {
        if let Err(error) = state.dispatch_blocking() {
            return state_error::describe(&error);
        }
    }
}

fn roundtrip(
    state: &Rc<State>,
    display: &Rc<wl_proxy::protocols::wayland::wl_display::WlDisplay>,
) -> Result<()> {
    let done = Rc::new(Cell::new(false));
    display.new_send_sync().set_handler(SyncDone(done.clone()));
    while !done.get() {
        state
            .dispatch_blocking()
            .map_err(|error| state_error::describe(&error))?;
    }
    Ok(())
}

struct ManagerFinder(Rc<Cell<Option<u32>>>);

impl WlRegistryHandler for ManagerFinder {
    fn handle_global(
        &mut self,
        _slf: &Rc<WlRegistry>,
        name: u32,
        interface: ObjectInterface,
        _version: u32,
    ) {
        if interface == ObjectInterface::WpSecurityContextManagerV1 {
            self.0.set(Some(name));
        }
    }

    fn handle_global_remove(&mut self, _slf: &Rc<WlRegistry>, _name: u32) {}
}

struct SyncDone(Rc<Cell<bool>>);

impl WlCallbackHandler for SyncDone {
    fn handle_done(&mut self, _slf: &Rc<WlCallback>, _callback_data: u32) {
        self.0.set(true);
    }
}
