//! The desktop window of the list: its title, the terminal that opens it and
//! the `KWin` script that brings it to view.

use std::{
    ffi::OsString,
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, bail, ensure};

/// The title that the list gives its terminal window.
pub const TITLE: &str = "approval-queue";

/// The `KWin` script. It finds the window by a caption that is `TITLE`.
pub const FOCUS_SCRIPT: &str = include_str!("focus_list.js");

const SCRIPT_NAME: &str = "approval-queue-focus-list";

/// The command that opens a terminal window with the list, as a unit of its
/// own, so the window outlives the process that asked for it.
#[must_use]
pub fn terminal_command(list_program: &Path) -> Vec<OsString> {
    let mut command: Vec<OsString> = [
        "systemd-run",
        "--user",
        "--collect",
        "--quiet",
        "--description=List of the waiting permission prompts",
        "ghostty",
        "--title=approval-queue",
        "-e",
    ]
    .map(OsString::from)
    .to_vec();
    command.push(list_program.into());
    command
}

/// Opens a terminal window with the list.
pub fn open_terminal(list_program: &Path) -> Result<()> {
    run(&terminal_command(list_program)).map(drop)
}

/// Loads the script into `KWin` and starts it. The script also shows a list
/// window that opens after the call, until `forget_script` unloads it.
pub fn bring_to_view(runtime_folder: &Path) -> Result<()> {
    let script = runtime_folder.join("approval-queue").join("focus-list.js");
    fs::write(&script, FOCUS_SCRIPT)
        .with_context(|| format!("Script {} not written", script.display()))?;
    let _ = forget_script();
    let script_text = script.to_str().context("Path of the script is not UTF-8")?;
    let loaded = run(&scripting_call(
        "loadScript",
        &["ss", script_text, SCRIPT_NAME],
    ))?;
    ensure!(is_loaded(&loaded)?, "KWin did not load the script");
    // Starts each loaded script that does not run yet. The reply of
    // loadScript has an ID, but KWin can give that ID to two scripts.
    run(&scripting_call("start", &[])).map(drop)
}

/// Unloads the script, so that it shows no window that opens from now on.
pub fn forget_script() -> Result<()> {
    run(&scripting_call("unloadScript", &["s", SCRIPT_NAME])).map(drop)
}

/// Reads the reply of `loadScript`, which has the form `i <id>`. The ID is
/// negative when `KWin` did not load the script.
pub fn is_loaded(reply: &str) -> Result<bool> {
    let id = reply
        .trim()
        .strip_prefix("i ")
        .and_then(|id| id.parse::<i64>().ok())
        .with_context(|| format!("Reply {reply:?} of KWin has another form"))?;
    Ok(id >= 0)
}

fn scripting_call(method: &str, arguments: &[&str]) -> Vec<OsString> {
    [
        "busctl",
        "--user",
        "--timeout=2",
        "call",
        "org.kde.KWin",
        "/Scripting",
        "org.kde.kwin.Scripting",
        method,
    ]
    .iter()
    .chain(arguments)
    .map(OsString::from)
    .collect()
}

fn run(command: &[OsString]) -> Result<String> {
    let (program, arguments) = command.split_first().context("Command is empty")?;
    let output = Command::new(program)
        .args(arguments)
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("{} not started", program.display()))?;
    if !output.status.success() {
        bail!(
            "{} failed: {}",
            program.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let reply = String::from_utf8(output.stdout).context("Reply is not UTF-8")?;
    ensure!(reply.len() < 4096, "Reply is too long");
    Ok(reply)
}
