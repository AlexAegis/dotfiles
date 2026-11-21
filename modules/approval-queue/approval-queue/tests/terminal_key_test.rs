use approval_queue::{key::Key, terminal_key::key_of};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[test]
fn should_name_each_key_of_the_list() {
    for (code, key) in [
        (KeyCode::Up, Key::Up),
        (KeyCode::Down, Key::Down),
        (KeyCode::PageUp, Key::PageUp),
        (KeyCode::PageDown, Key::PageDown),
        (KeyCode::Enter, Key::Enter),
        (KeyCode::Esc, Key::Escape),
        (KeyCode::Backspace, Key::Backspace),
        (KeyCode::Char('a'), Key::Character('a')),
    ] {
        assert_eq!(key_of(KeyEvent::new(code, KeyModifiers::NONE)), Some(key));
    }
    assert_eq!(
        key_of(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)),
        Some(Key::Other)
    );
}

#[test]
fn should_take_a_key_with_ctrl_or_alt_as_no_plain_key() {
    for modifiers in [
        KeyModifiers::CONTROL,
        KeyModifiers::ALT,
        KeyModifiers::SUPER,
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    ] {
        for code in [
            KeyCode::Char('a'),
            KeyCode::Char('d'),
            KeyCode::Char('j'),
            KeyCode::Enter,
        ] {
            assert_eq!(key_of(KeyEvent::new(code, modifiers)), Some(Key::Other));
        }
    }
}

#[test]
fn should_keep_the_character_of_a_key_with_shift() {
    assert_eq!(
        key_of(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT)),
        Some(Key::Character('A'))
    );
}

#[test]
fn should_take_ctrl_c_as_the_interrupt_key() {
    assert_eq!(
        key_of(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
        Some(Key::Interrupt)
    );
    assert_eq!(
        key_of(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::ALT)),
        Some(Key::Other)
    );
}

#[test]
fn should_take_no_key_from_a_release() {
    assert_eq!(
        key_of(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::NONE,
            KeyEventKind::Release
        )),
        None
    );
}
