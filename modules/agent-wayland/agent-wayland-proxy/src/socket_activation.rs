use std::{
    env,
    os::{
        fd::{FromRawFd, OwnedFd},
        unix::net::UnixStream,
    },
    process,
};

use anyhow::{Context, Result, bail};

const SD_LISTEN_FDS_START: i32 = 3;

/// Takes the client connection that the systemd socket unit accepted and passes.
pub fn take_connection() -> Result<UnixStream> {
    let pid: u32 = env::var("LISTEN_PID")
        .context("LISTEN_PID is not set; start the proxy through its socket unit")?
        .parse()
        .context("LISTEN_PID is not a number")?;
    let count: u32 = env::var("LISTEN_FDS")
        .context("LISTEN_FDS is not set")?
        .parse()
        .context("LISTEN_FDS is not a number")?;
    if pid != process::id() {
        bail!("LISTEN_PID {pid} names another process");
    }
    if count != 1 {
        bail!("Socket unit passed {count} sockets, expected 1");
    }
    // SAFETY: LISTEN_PID names this process, so systemd passed exactly one socket
    // at SD_LISTEN_FDS_START, and no other code of this process owns that fd.
    let socket = unsafe { OwnedFd::from_raw_fd(SD_LISTEN_FDS_START) };
    Ok(UnixStream::from(socket))
}
