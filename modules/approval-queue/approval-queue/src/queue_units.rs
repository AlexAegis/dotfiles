//! The systemd units of the queue: the socket, the service processes of
//! the waiting prompts and the tray program.

use std::{
    fs,
    os::unix::fs::FileTypeExt,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, ensure};

/// The units of the queue, the unit of the tray program last: its stop ends
/// the program that asks for the stop.
pub const UNITS: [&str; 3] = [
    "approval-queue.socket",
    "approval-queue@*.service",
    "approval-queue-notify.service",
];

/// The `ListenStream` of the socket unit, which removes the socket at its
/// stop.
pub const REQUEST_SOCKET: &str = "/run/approval-queue/request.sock";

/// The command that starts the queue again after a stop. It names the
/// socket too, because the socket can stop while the tray program runs.
pub const START_COMMAND: &str =
    "systemctl --user start approval-queue.socket approval-queue-notify.service";

/// Whether the socket of the queue is gone.
#[must_use]
pub fn is_stopped() -> bool {
    is_stopped_at(Path::new(REQUEST_SOCKET))
}

/// Whether no socket is at the path. A connection would tell more, but each
/// connection to the socket unit starts a service process.
#[must_use]
pub fn is_stopped_at(socket: &Path) -> bool {
    !fs::symlink_metadata(socket).is_ok_and(|metadata| metadata.file_type().is_socket())
}

/// The command that stops the queue until its units start again, such as at
/// the next login. It does not wait for the stop.
#[must_use]
pub fn stop_command() -> Vec<&'static str> {
    ["systemctl", "--user", "--no-block", "stop"]
        .into_iter()
        .chain(UNITS)
        .collect()
}

/// Stops the queue. No new prompt comes to it. Each waiting prompt stays in
/// the terminal of its session.
pub fn stop() -> Result<()> {
    let command = stop_command();
    let status = Command::new(command[0])
        .args(&command[1..])
        .stdin(Stdio::null())
        .status()
        .context("systemctl not started")?;
    ensure!(status.success(), "systemctl failed: {status}");
    Ok(())
}
