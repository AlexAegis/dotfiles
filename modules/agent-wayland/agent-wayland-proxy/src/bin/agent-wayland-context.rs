use std::{env, path::PathBuf};

use agent_wayland_proxy::{logging, security_context::RestrictedSocket, systemd_notify};
use anyhow::{Context, Result};

fn main() -> Result<()> {
    logging::init();
    let restricted_socket = path_from_env("AGENT_WAYLAND_KWIN_SOCKET")?;
    let context = RestrictedSocket::create(&restricted_socket)?;
    tracing::info!(
        "KWin security context listens on {}",
        restricted_socket.display()
    );
    systemd_notify::ready()?;
    Err(context.keep_alive().context("KWin connection lost"))
}

fn path_from_env(name: &str) -> Result<PathBuf> {
    env::var_os(name)
        .map(PathBuf::from)
        .with_context(|| format!("{name} is not set"))
}
