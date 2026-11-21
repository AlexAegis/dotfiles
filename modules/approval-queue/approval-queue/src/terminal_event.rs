use std::time::Instant;

use ratatui::crossterm::event::Event;

use crate::{action::Action, list_state::ListState, terminal_key::key_of};

/// Gives one event of the terminal to the list: a key press as its key, and
/// pasted text as text. Returns what the list does then.
pub fn take_event(state: &mut ListState, event: Event, now: Instant) -> Option<Action> {
    match event {
        Event::Key(event) => key_of(event).and_then(|key| state.press(key, now)),
        Event::Paste(text) => {
            state.paste(&text, now);
            None
        }
        _ => None,
    }
}
