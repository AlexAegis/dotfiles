use std::{
    env, io,
    path::Path,
    process::ExitCode,
    time::{Duration, Instant, SystemTime},
};

use anyhow::{Context, Result, ensure};
use approval_queue::{
    action::Action, list_state::ListState, list_view, queue_folder::QueueFolder,
    terminal_event::take_event,
};
use ratatui::{
    DefaultTerminal,
    crossterm::{
        event::{self, DisableBracketedPaste, EnableBracketedPaste},
        execute,
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
    // A terminal without this mode sends pasted text as keys
    let _ = execute!(io::stdout(), EnableBracketedPaste);
    let result = show(&mut terminal, &queue);
    let _ = execute!(io::stdout(), DisableBracketedPaste);
    ratatui::restore();
    result
}

fn show(terminal: &mut DefaultTerminal, queue: &QueueFolder) -> Result<()> {
    let mut state = ListState::default();
    loop {
        queue.remove_answers_without_entry()?;
        state.refresh(queue.waiting()?, Instant::now());
        let mut scroll_limit = 0;
        terminal.draw(|frame| scroll_limit = list_view::draw(frame, &state, SystemTime::now()))?;
        state.limit_scroll(scroll_limit);
        if !event::poll(REFRESH_INTERVAL)? {
            continue;
        }
        match take_event(&mut state, event::read()?, Instant::now()) {
            Some(Action::Quit) => return Ok(()),
            Some(Action::Answer { id, answer }) => {
                if let Err(error) = queue.answer(&id, &answer) {
                    state.report_refused_answer(&id, &error.to_string());
                }
            }
            None => {}
        }
    }
}
