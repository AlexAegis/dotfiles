use std::{env, path::PathBuf};

use agent_wayland_proxy::{client_connection, logging, socket_activation};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    logging::init();
    let stream = socket_activation::take_connection()?;
    let restricted_socket = PathBuf::from(
        env::var_os("AGENT_WAYLAND_KWIN_SOCKET").context("AGENT_WAYLAND_KWIN_SOCKET is not set")?,
    );
    client_connection::run(stream, &restricted_socket)
}
