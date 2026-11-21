use std::time::{Duration, Instant};

use approval_queue::{
    action::Action, answer::Answer, entry::Entry, list_state::ListState, listed_entry::ListedEntry,
    terminal_event::take_event,
};
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use serde_json::Map;

fn listed(id: &str) -> ListedEntry {
    ListedEntry {
        id: id.to_owned(),
        entry: Entry {
            received: 0,
            user: "agent".to_owned(),
            user_is_verified: true,
            host: "claude-code".to_owned(),
            session: "0a1b".to_owned(),
            folder: "/home/agent".to_owned(),
            mode: "auto".to_owned(),
            tool: "Bash".to_owned(),
            input: Map::new(),
            reason: None,
            list_only: false,
        },
    }
}

fn list_with_2_entries_and_a_time_after_its_quiet_time() -> (ListState, Instant) {
    let start = Instant::now();
    let mut state = ListState::default();
    state.refresh(vec![listed("1-1"), listed("2-1")], start);
    (state, start + Duration::from_secs(5))
}

fn key(code: KeyCode, modifiers: KeyModifiers) -> Event {
    Event::Key(KeyEvent::new(code, modifiers))
}

fn plain(character: char) -> Event {
    key(KeyCode::Char(character), KeyModifiers::NONE)
}

#[test]
fn should_answer_on_a_plain_key() {
    let (mut state, now) = list_with_2_entries_and_a_time_after_its_quiet_time();
    assert_eq!(
        take_event(&mut state, plain('a'), now),
        Some(Action::Answer {
            id: "1-1".to_owned(),
            answer: Answer::Allow
        })
    );
}

#[test]
fn should_take_pasted_text_as_no_key() {
    let (mut state, now) = list_with_2_entries_and_a_time_after_its_quiet_time();
    for text in ["a", "a\nd", "q", "m\n"] {
        assert_eq!(
            take_event(&mut state, Event::Paste(text.to_owned()), now),
            None
        );
    }
    assert_eq!(state.answer_of("1-1"), None);
    assert_eq!(state.message(), None);
    assert_eq!(
        state.notice(),
        Some("Pasted text is no answer. The m key opens a message.")
    );
}

#[test]
fn should_add_pasted_text_to_an_open_message() {
    let (mut state, now) = list_with_2_entries_and_a_time_after_its_quiet_time();
    assert_eq!(take_event(&mut state, plain('m'), now), None);
    assert_eq!(
        take_event(&mut state, Event::Paste("use a\nbranch".to_owned()), now),
        None
    );
    assert_eq!(state.message(), Some("use a branch"));
    assert_eq!(state.answer_of("1-1"), None);
}

#[test]
fn should_take_a_key_with_ctrl_as_no_answer() {
    let (mut state, now) = list_with_2_entries_and_a_time_after_its_quiet_time();
    for character in ['a', 'd', 'm', 'q'] {
        assert_eq!(
            take_event(
                &mut state,
                key(KeyCode::Char(character), KeyModifiers::CONTROL),
                now
            ),
            None
        );
    }
    assert_eq!(state.answer_of("1-1"), None);
    assert_eq!(state.message(), None);
}

#[test]
fn should_start_the_quiet_time_at_a_key_that_the_list_does_not_use() {
    for unused in [
        key(KeyCode::Char('d'), KeyModifiers::CONTROL),
        key(KeyCode::Tab, KeyModifiers::NONE),
    ] {
        let (mut state, now) = list_with_2_entries_and_a_time_after_its_quiet_time();
        assert_eq!(take_event(&mut state, unused, now), None);
        assert_eq!(
            take_event(&mut state, plain('a'), now + Duration::from_millis(999)),
            None
        );
        assert_eq!(state.answer_of("1-1"), None);
    }
}

#[test]
fn should_keep_a_key_that_the_list_does_not_use_out_of_a_message() {
    let (mut state, now) = list_with_2_entries_and_a_time_after_its_quiet_time();
    take_event(&mut state, plain('m'), now);
    for unused in [
        key(KeyCode::Char('d'), KeyModifiers::CONTROL),
        key(KeyCode::Tab, KeyModifiers::NONE),
    ] {
        assert_eq!(take_event(&mut state, unused, now), None);
    }
    assert_eq!(state.message(), Some(""));
}

#[test]
fn should_quit_on_ctrl_c() {
    let (mut state, now) = list_with_2_entries_and_a_time_after_its_quiet_time();
    assert_eq!(
        take_event(
            &mut state,
            key(KeyCode::Char('c'), KeyModifiers::CONTROL),
            now
        ),
        Some(Action::Quit)
    );
}

#[test]
fn should_take_no_key_from_another_event() {
    let (mut state, now) = list_with_2_entries_and_a_time_after_its_quiet_time();
    for event in [Event::FocusGained, Event::Resize(80, 24)] {
        assert_eq!(take_event(&mut state, event, now), None);
    }
    assert_eq!(state.notice(), None);
}
