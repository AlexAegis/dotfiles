use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::key::Key;

/// The key of a terminal event, when the event is a key press. A key with
/// Ctrl, Alt or a similar modifier is not its plain key: Ctrl-D is no `d`.
/// Ctrl-C is the one such key of the list.
#[must_use]
pub fn key_of(event: KeyEvent) -> Option<Key> {
    if event.kind != KeyEventKind::Press {
        return None;
    }
    let modifiers = event.modifiers.difference(KeyModifiers::SHIFT);
    if modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
        return Some(Key::Interrupt);
    }
    if !modifiers.is_empty() {
        return Some(Key::Other(name(event.code, modifiers)));
    }
    Some(match event.code {
        KeyCode::Up => Key::Up,
        KeyCode::Down => Key::Down,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::Enter => Key::Enter,
        KeyCode::Esc => Key::Escape,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::Char(character) => Key::Character(character),
        code => Key::Other(name(code, modifiers)),
    })
}

/// The name of a key for a notice: its modifiers, then its code.
fn name(code: KeyCode, modifiers: KeyModifiers) -> String {
    let modifiers: String = [
        (KeyModifiers::CONTROL, "Ctrl+"),
        (KeyModifiers::ALT, "Alt+"),
        (KeyModifiers::SUPER, "Super+"),
        (KeyModifiers::HYPER, "Hyper+"),
        (KeyModifiers::META, "Meta+"),
    ]
    .into_iter()
    .filter(|(modifier, _)| modifiers.contains(*modifier))
    .map(|(_, text)| text)
    .collect();
    let code = match code {
        KeyCode::Char(' ') => "space".to_owned(),
        KeyCode::Char(character) => character.to_string(),
        KeyCode::F(number) => format!("F{number}"),
        code => format!("{code:?}"),
    };
    format!("{modifiers}{code}")
}
