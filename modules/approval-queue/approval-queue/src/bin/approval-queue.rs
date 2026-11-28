use std::{
    env, io,
    path::Path,
    process::ExitCode,
    time::{Duration, Instant, SystemTime},
};

use anyhow::{Context, Result, ensure};
use approval_queue::{
    list_actions, list_state::ListState, list_view, list_window, queue_folder::QueueFolder,
    queue_units, terminal_event::take_event,
};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        event::{
            self, DisableBracketedPaste, DisableFocusChange, EnableBracketedPaste,
            EnableFocusChange,
        },
        execute,
        terminal::SetTitle,
    },
};

const REFRESH_INTERVAL: Duration = Duration::from_millis(250);

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
    ensure!(
        env::args_os().len() == 1,
        "Usage: approval-queue (no arguments; lists the waiting permission prompts)"
    );
    let runtime_folder = env::var_os("XDG_RUNTIME_DIR").context("XDG_RUNTIME_DIR is not set")?;
    let queue = QueueFolder::in_runtime_folder(Path::new(&runtime_folder));
    queue.create()?;
    let _open_list = queue.open_list()?;
    let mut terminal = ratatui::try_init().context("Terminal not usable")?;
    let modes = ListModes::turn_on();
    let result = show(&mut terminal, &queue);
    drop(modes);
    ratatui::restore();
    result
}

/// The terminal modes of the list. They end with the value, also when the
/// list panics.
struct ListModes;

impl ListModes {
    fn turn_on() -> Self {
        // A terminal without this mode sends pasted text as keys
        let _ = execute!(io::stdout(), EnableBracketedPaste);
        // The list starts its quiet time when its window gets the focus
        let _ = execute!(io::stdout(), EnableFocusChange);
        // The window title by which approval-queue-notify finds the list
        let _ = execute!(io::stdout(), SetTitle(list_window::TITLE));
        Self
    }
}

impl Drop for ListModes {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), DisableFocusChange);
        let _ = execute!(io::stdout(), DisableBracketedPaste);
        // A shell that sets no title of its own would keep the title of the list
        let _ = execute!(io::stdout(), SetTitle(""));
    }
}

fn show(terminal: &mut DefaultTerminal, queue: &QueueFolder) -> Result<()> {
    let mut state = ListState::default();
    loop {
        queue.remove_leftovers()?;
        state.refresh(queue.waiting()?, Instant::now());
        state.show_mute(queue.pings_muted_until());
        state.show_sessions(queue.sessions()?);
        state.show_queue_stopped(queue_units::is_stopped());
        let mut scroll_limit = 0;
        terminal.draw(|frame| scroll_limit = list_view::draw(frame, &state, SystemTime::now()))?;
        state.limit_scroll(scroll_limit);
        if !event::poll(REFRESH_INTERVAL)? {
            continue;
        }
        if let Some(action) = take_event(&mut state, event::read()?, Instant::now())
            && list_actions::carry_out(action, queue, &mut state, SystemTime::now())
        {
            return Ok(());
        }
    }
}
