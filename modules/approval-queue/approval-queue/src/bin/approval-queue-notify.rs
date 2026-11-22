use std::{
    env,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use approval_queue::{
    list_window,
    ping_watch::{CADENCE, PingWatch},
    queue_folder::QueueFolder,
};

const REFRESH_INTERVAL: Duration = Duration::from_millis(500);
/// Long enough for the terminal of a ping to open its window.
const SCRIPT_KEEP_TIME: Duration = Duration::from_secs(10);

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let runtime_folder =
        PathBuf::from(env::var_os("XDG_RUNTIME_DIR").context("XDG_RUNTIME_DIR is not set")?);
    let list_program = env::current_exe()
        .context("Own path unknown")?
        .with_file_name("approval-queue");
    let queue = QueueFolder::in_runtime_folder(&runtime_folder);
    queue.create()?;
    let (ping_sender, pings) = mpsc::channel();
    let mut watch = PingWatch::new(CADENCE, move |ping| {
        let _ = ping_sender.send(ping.keys.len());
    });
    let mut forget_script_at = None;
    loop {
        let now = Instant::now();
        match queue.waiting_ids() {
            Ok(waiting) => watch.refresh(waiting, now),
            Err(error) => eprintln!("Queue unreadable: {error:#}"),
        }
        if let Ok(prompt_count) = pings.try_recv() {
            eprintln!("Ping for {prompt_count} new prompts");
            if let Err(error) = show_list(&queue, &runtime_folder, &list_program) {
                eprintln!("List not shown: {error:#}");
            }
            forget_script_at = Some(now + SCRIPT_KEEP_TIME);
        }
        if forget_script_at.is_some_and(|at| now >= at) {
            let _ = list_window::forget_script();
            forget_script_at = None;
        }
        thread::sleep(REFRESH_INTERVAL);
    }
}

/// Brings the window of the list to view. Opens a terminal with the list
/// first when no list is open.
fn show_list(queue: &QueueFolder, runtime_folder: &Path, list_program: &Path) -> Result<()> {
    if let Err(error) = list_window::bring_to_view(runtime_folder) {
        eprintln!("Window of the list not focused: {error:#}");
    }
    if !queue.has_open_list()? {
        list_window::open_terminal(list_program)?;
    }
    Ok(())
}
