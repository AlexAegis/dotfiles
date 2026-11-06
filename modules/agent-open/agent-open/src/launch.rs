use std::{ffi::OsString, process::Command};

use anyhow::{Context, Result, bail};

use crate::{config::Config, request::Request};

/// The `systemd-run` command that opens the request as a transient service of
/// the user manager. The program gets the environment of the session, not the
/// sandbox of this service. `ExitType=cgroup` keeps the service, and so the
/// program, alive after a launcher such as `xdg-open` exits.
pub fn command_line(request: &Request, config: &Config) -> Vec<OsString> {
    let (program, argument, description) = match request {
        Request::Url(url) => (&config.browser, OsString::from(url.as_str()), "URL"),
        Request::LoopbackUrl(url) => (
            config.loopback_browser.as_ref().unwrap_or(&config.browser),
            OsString::from(url.as_str()),
            "loopback URL",
        ),
        Request::Edit(path) => (&config.editor, path.clone().into_os_string(), "path"),
    };
    let mut command_line: Vec<OsString> = [
        "systemd-run",
        "--user",
        "--quiet",
        "--collect",
        "--property=ExitType=cgroup",
    ]
    .into_iter()
    .map(OsString::from)
    .collect();
    command_line.push(format!("--description=agent-open: {description}").into());
    command_line.push("--".into());
    command_line.extend(program.iter().map(OsString::from));
    command_line.push(argument);
    command_line
}

pub fn run(command_line: &[OsString]) -> Result<()> {
    let (program, arguments) = command_line.split_first().context("Empty command line")?;
    let status = Command::new(program)
        .args(arguments)
        .status()
        .with_context(|| format!("{} failed to start", program.display()))?;
    if !status.success() {
        bail!("{} failed: {status}", program.display());
    }
    Ok(())
}
