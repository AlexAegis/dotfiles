//! The systemd units of the queue: the socket, the service processes of
//! the waiting prompts and the tray program.

use std::process::{Command, Stdio};

use anyhow::{Context, Result, ensure};

/// The units of the queue, the unit of the tray program last: its stop ends
/// the program that asks for the stop.
pub const UNITS: [&str; 3] = [
    "approval-queue.socket",
    "approval-queue@*.service",
    "approval-queue-notify.service",
];

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
