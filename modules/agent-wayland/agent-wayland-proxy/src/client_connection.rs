use std::{
    cell::Cell,
    error::Error,
    fmt,
    os::{fd::OwnedFd, unix::net::UnixStream},
    path::Path,
    rc::Rc,
};

use anyhow::{Context, Result};
use wl_proxy::{
    client::ClientHandler,
    protocols::wayland::{
        wl_display::{WlDisplay, WlDisplayHandler},
        wl_registry::WlRegistry,
    },
    state::{Destructor, State},
};

use crate::{BASELINE, registry_filter::RegistryFilter, state_error};

/// Proxies one agent client to the restricted KWin socket until either side ends.
pub fn run(client: UnixStream, restricted_socket: &Path) -> Result<()> {
    let kwin = UnixStream::connect(restricted_socket).with_context(|| {
        format!(
            "Connection to the restricted KWin socket {} failed",
            restricted_socket.display()
        )
    })?;
    serve(client, kwin)
}

/// Proxies one agent client to a connected compositor until either side ends.
pub fn serve(client: UnixStream, compositor: UnixStream) -> Result<()> {
    let state = State::builder(BASELINE)
        .with_server_fd(&Rc::new(OwnedFd::from(compositor)))
        .build()
        .map_err(|error| state_error::describe(&error).context("Proxy setup failed"))?;
    let _destroy_on_return = state.create_destructor();
    let client = state
        .add_client(&Rc::new(OwnedFd::from(client)))
        .map_err(|error| state_error::describe(&error))?;
    client.set_handler(DestroyOnDisconnect(state.create_destructor()));
    let violation = Rc::new(Cell::new(false));
    client.display().set_handler(Display {
        violation: violation.clone(),
    });
    while state.is_not_destroyed() {
        state
            .dispatch_blocking()
            .map_err(|error| state_error::describe(&error))?;
        if violation.get() {
            return Err(UnofferedBind.into());
        }
    }
    Ok(())
}

/// Ends the connection of a client that bound a global outside what it was offered.
#[derive(Debug)]
pub struct UnofferedBind;

impl fmt::Display for UnofferedBind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("Client bound a global that it was not offered")
    }
}

impl Error for UnofferedBind {}

struct DestroyOnDisconnect(
    #[expect(dead_code, reason = "dropping it destroys the state")] Destructor,
);

impl ClientHandler for DestroyOnDisconnect {}

struct Display {
    violation: Rc<Cell<bool>>,
}

impl WlDisplayHandler for Display {
    fn handle_get_registry(&mut self, display: &Rc<WlDisplay>, registry: &Rc<WlRegistry>) {
        display.send_get_registry(registry);
        registry.set_handler(RegistryFilter::new(self.violation.clone()));
    }
}
