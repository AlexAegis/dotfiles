use std::{
    env,
    os::{
        linux::net::SocketAddrExt,
        unix::{
            ffi::OsStrExt,
            net::{SocketAddr, UnixDatagram},
        },
    },
};

use anyhow::{Context, Result};

/// Tells systemd that the service is ready. Does nothing outside a `Type=notify` unit.
pub fn ready() -> Result<()> {
    let Some(path) = env::var_os("NOTIFY_SOCKET") else {
        return Ok(());
    };
    let address = match path.as_bytes().strip_prefix(b"@") {
        Some(name) => SocketAddr::from_abstract_name(name),
        None => SocketAddr::from_pathname(&path),
    }
    .context("NOTIFY_SOCKET is not a socket address")?;
    UnixDatagram::unbound()
        .and_then(|socket| socket.send_to_addr(b"READY=1", &address))
        .context("Readiness notification failed")?;
    Ok(())
}
