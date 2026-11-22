use std::time::Instant;

use ratatui::crossterm::event::Event;

use crate::{action::Action, list_state::ListState, terminal_key::key_of};

/// Gives one event of the terminal to the list: a key press as its key, and
/// pasted text as text. The window of the list got the focus: the quiet time
/// starts, so a key that the owner typed for another window gives no answer.
/// Returns what the list does then.
pub fn take_event(state: &mut ListState, event: Event, now: Instant) -> Option<Action> {
    match event {
        Event::Key(event) => key_of(event).and_then(|key| state.press(key, now)),
        Event::Paste(text) => {
            state.paste(&text, now);
            None
        }
        Event::FocusGained => {
            state.gain_focus(now);
            None
        }
        _ => None,
    }
}
