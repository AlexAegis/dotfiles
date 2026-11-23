use std::time::{Duration, Instant};

use approval_queue::{
    action::Action, answer::Answer, entry::Entry, key::Key, list_state::ListState,
    listed_entry::ListedEntry,
};
use serde_json::Map;

fn listed(id: &str) -> ListedEntry {
    ListedEntry {
        id: id.to_owned(),
        entry: Entry {
            received: 0,
            user: "agent".to_owned(),
            user_is_verified: false,
            host: "claude-code".to_owned(),
            session: "0a1b".to_owned(),
            folder: "/home/agent".to_owned(),
            mode: "auto".to_owned(),
            tool: "Bash".to_owned(),
            title: None,
            risk: None,
            input: Map::new(),
            reason: None,
            list_only: false,
        },
    }
}

fn entries(ids: &[&str]) -> Vec<ListedEntry> {
    ids.iter().map(|id| listed(id)).collect()
}

fn milliseconds(count: u64) -> Duration {
    Duration::from_millis(count)
}

fn state_with(ids: &[&str], start: Instant) -> ListState {
    let mut state = ListState::default();
    state.refresh(entries(ids), start);
    state
}

fn selected_id(state: &ListState) -> Option<&str> {
    state.selected().map(|listed| listed.id.as_str())
}

fn assert_answers(action: Option<Action>, id: &str, answer: Answer) {
    match action {
        Some(Action::Answer {
            id: answered,
            answer: given,
        }) => assert_eq!((answered.as_str(), given), (id, answer)),
        other => panic!("{other:?} is no answer"),
    }
}

fn typed(state: &mut ListState, text: &str, now: Instant) {
    for character in text.chars() {
        assert_eq!(state.press(Key::Character(character), now), None);
    }
}

const TYPING_PACE: Duration = Duration::from_millis(200);

fn type_at_5_keys_per_second(
    state: &mut ListState,
    waiting: &mut Vec<&str>,
    text: &str,
    start: Instant,
) -> (Vec<Action>, Instant) {
    let mut actions = Vec::new();
    let mut now = start;
    let mut last = start;
    for character in text.chars() {
        let key = if character == '\n' {
            Key::Enter
        } else {
            Key::Character(character)
        };
        if let Some(action) = state.press(key, now) {
            if let Action::Answer { id, .. } = &action {
                waiting.retain(|listed| listed != id);
                state.refresh(entries(waiting), now + milliseconds(50));
            }
            actions.push(action);
        }
        last = now;
        now += TYPING_PACE;
    }
    (actions, last)
}

#[test]
fn should_select_the_oldest_entry_at_first() {
    let state = state_with(&["1-1", "2-1"], Instant::now());
    assert_eq!(selected_id(&state), Some("1-1"));
    assert_eq!(state.selected_index(), Some(0));
}

#[test]
fn should_select_no_entry_of_an_empty_queue() {
    let start = Instant::now();
    let mut state = state_with(&[], start);
    assert_eq!(selected_id(&state), None);
    for key in ['a', 'd', 'm'] {
        assert_eq!(
            state.press(Key::Character(key), start + milliseconds(5000)),
            None
        );
        assert_eq!(state.notice(), Some("No prompt waits"));
    }
    assert_eq!(state.message(), None);
}

#[test]
fn should_keep_the_selection_while_other_entries_come_and_go() {
    let start = Instant::now();
    let mut state = state_with(&["1-1", "2-1", "3-1"], start);
    state.press(Key::Down, start);
    state.refresh(entries(&["2-1", "3-1", "4-1"]), start);
    assert_eq!(selected_id(&state), Some("2-1"));
    assert_eq!(state.selected_index(), Some(0));
    state.refresh(entries(&["0-1", "2-1"]), start);
    assert_eq!(selected_id(&state), Some("2-1"));
    assert_eq!(state.selected_index(), Some(1));
}

#[test]
fn should_move_the_selection_between_the_first_and_the_last_entry() {
    let start = Instant::now();
    let mut state = state_with(&["1-1", "2-1", "3-1"], start);
    for (key, selected) in [
        (Key::Up, "1-1"),
        (Key::Down, "2-1"),
        (Key::Character('j'), "3-1"),
        (Key::Down, "3-1"),
        (Key::Character('k'), "2-1"),
        (Key::Up, "1-1"),
    ] {
        assert_eq!(state.press(key, start), None);
        assert_eq!(selected_id(&state), Some(selected));
    }
}

#[test]
fn should_answer_the_selected_entry_after_the_quiet_time() {
    let start = Instant::now();
    for (key, given) in [('a', Answer::ALLOW), ('d', Answer::Deny { message: None })] {
        let mut state = state_with(&["1-1", "2-1"], start);
        assert_eq!(
            state.press(Key::Character(key), start + milliseconds(999)),
            None
        );
        assert_eq!(
            state.notice(),
            Some("Too early. Press the key after 1 second without a key.")
        );
        assert_eq!(state.answer_of("1-1"), None);
        assert_answers(
            state.press(Key::Character(key), start + milliseconds(1999)),
            "1-1",
            given.clone(),
        );
        assert_eq!(state.notice(), None);
        assert_eq!(state.answer_of("1-1"), Some(&given));
        assert_eq!(state.answer_of("2-1"), None);
    }
}

#[test]
fn should_answer_at_the_first_key_after_the_quiet_time() {
    let start = Instant::now();
    let mut state = state_with(&["1-1"], start);
    assert_answers(
        state.press(Key::Character('a'), start + milliseconds(1000)),
        "1-1",
        Answer::ALLOW,
    );
}

#[test]
fn should_start_the_quiet_time_again_at_each_refused_key() {
    let start = Instant::now();
    let mut state = state_with(&["1-1"], start);
    for pressed_after in [999, 1500, 2499] {
        assert_eq!(
            state.press(Key::Character('a'), start + milliseconds(pressed_after)),
            None
        );
        assert_eq!(
            state.notice(),
            Some("Too early. Press the key after 1 second without a key.")
        );
    }
    assert_answers(
        state.press(Key::Character('a'), start + milliseconds(3499)),
        "1-1",
        Answer::ALLOW,
    );
}

#[test]
fn should_take_no_answer_for_the_next_entry_right_after_the_selected_one_left() {
    let start = Instant::now();
    let mut state = state_with(&["1-1", "2-1"], start);
    assert_answers(
        state.press(Key::Character('a'), start + milliseconds(2000)),
        "1-1",
        Answer::ALLOW,
    );
    state.refresh(entries(&["2-1"]), start + milliseconds(2100));
    assert_eq!(selected_id(&state), Some("2-1"));
    assert_eq!(
        state.press(Key::Character('a'), start + milliseconds(3099)),
        None
    );
    assert_eq!(state.answer_of("2-1"), None);
    assert_answers(
        state.press(Key::Character('a'), start + milliseconds(4099)),
        "2-1",
        Answer::ALLOW,
    );
}

#[test]
fn should_start_the_quiet_time_at_each_key_outside_a_message() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    for (key, selected) in [
        (Key::Down, "2-1"),
        (Key::Character('j'), "2-1"),
        (Key::Up, "1-1"),
        (Key::Character('k'), "1-1"),
        (Key::PageDown, "1-1"),
        (Key::PageUp, "1-1"),
        (Key::Character('x'), "1-1"),
        (Key::Enter, "1-1"),
        (Key::Escape, "1-1"),
        (Key::Backspace, "1-1"),
        (Key::Other("Tab".to_owned()), "1-1"),
    ] {
        for refused in ['a', 'd', 'q', 's', 'S', 'u'] {
            let mut state = state_with(&["1-1", "2-1"], start);
            assert_eq!(state.press(key.clone(), later), None);
            assert_eq!(
                state.press(Key::Character(refused), later + milliseconds(999)),
                None,
                "{key:?} {refused}"
            );
            assert_eq!(state.message(), None, "{key:?} {refused}");
        }
        let mut state = state_with(&["1-1", "2-1"], start);
        state.press(key, later);
        assert_answers(
            state.press(Key::Character('a'), later + milliseconds(1000)),
            selected,
            Answer::ALLOW,
        );
    }
}

#[test]
fn should_take_no_answer_for_another_entry_right_after_an_answer() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1", "2-1"], start);
    assert_answers(
        state.press(Key::Character('a'), later),
        "1-1",
        Answer::ALLOW,
    );
    state.press(Key::Down, later);
    assert_eq!(
        state.press(Key::Character('a'), later + milliseconds(999)),
        None
    );
    assert_eq!(
        state.notice(),
        Some("Too early. Press the key after 1 second without a key.")
    );
    assert_answers(
        state.press(Key::Character('d'), later + milliseconds(1999)),
        "2-1",
        Answer::Deny { message: None },
    );
}

#[test]
fn should_send_no_answer_from_text_typed_at_5_keys_per_second() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    for (text, kept) in [
        ("read the docs and add a test", None),
        ("remember me, a madam\n", Some("ember me, a madam")),
        ("the queue", None),
        ("ja, ja, da da da", None),
        ("kjkjkjkjkja", None),
    ] {
        let mut waiting = vec!["1-1", "2-1", "3-1"];
        let mut state = state_with(&waiting, start);
        let (actions, last) = type_at_5_keys_per_second(&mut state, &mut waiting, text, later);
        assert_eq!(actions, [], "{text}");
        assert_eq!(state.message(), None, "{text}");
        assert_eq!(
            state.press(Key::Character('a'), last + milliseconds(999)),
            None,
            "{text}"
        );
        assert_eq!(state.kept_message(), kept, "{text}");
        let selected = selected_id(&state).map(str::to_owned);
        assert_answers(
            state.press(Key::Character('a'), last + milliseconds(1999)),
            selected.as_deref().unwrap_or_default(),
            Answer::Allow {
                answers: None,
                message: kept.map(str::to_owned),
            },
        );
    }
}

#[test]
fn should_answer_one_prompt_at_most_from_text_that_starts_with_a_key_of_the_list() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    for (text, answered, answer, open_message) in [
        ("add a test and add a doc", "1-1", Answer::ALLOW, None),
        (
            "do a dance, madam",
            "1-1",
            Answer::Deny { message: None },
            Some("adam"),
        ),
        ("ajkjkjkjkjkja", "1-1", Answer::ALLOW, None),
        ("dada", "1-1", Answer::Deny { message: None }, None),
    ] {
        let mut waiting = vec!["1-1", "2-1", "3-1"];
        let mut state = state_with(&waiting, start);
        let (actions, _) = type_at_5_keys_per_second(&mut state, &mut waiting, text, later);
        assert_eq!(
            actions,
            [Action::Answer {
                id: answered.to_owned(),
                answer
            }],
            "{text}"
        );
        assert_eq!(state.message(), open_message, "{text}");
    }
}

#[test]
fn should_send_no_answer_from_text_that_starts_on_an_empty_queue() {
    let start = Instant::now();
    let mut state = state_with(&[], start);
    assert_eq!(state.press(Key::Character('m'), start), None);
    assert_eq!(state.notice(), Some("No prompt waits"));
    let mut waiting = vec!["1-1", "2-1"];
    state.refresh(entries(&waiting), start + milliseconds(100));
    let (actions, last) = type_at_5_keys_per_second(
        &mut state,
        &mut waiting,
        "adamadamadam\n",
        start + milliseconds(200),
    );
    assert_eq!(actions, []);
    assert_eq!(state.message(), None);
    assert_eq!(state.kept_message(), Some("adamadam"));
    assert_answers(
        state.press(Key::Character('a'), last + milliseconds(1000)),
        "1-1",
        Answer::Allow {
            answers: None,
            message: Some("adamadam".to_owned()),
        },
    );
}

#[test]
fn should_quit_on_q_after_the_typed_text_stopped() {
    let start = Instant::now();
    let mut state = state_with(&["1-1"], start);
    assert_eq!(state.press(Key::Character('x'), start), None);
    assert_eq!(
        state.press(Key::Character('q'), start + milliseconds(999)),
        None
    );
    assert_eq!(
        state.notice(),
        Some("Too early. Press the key after 1 second without a key.")
    );
    assert_eq!(
        state.press(Key::Character('q'), start + milliseconds(1998)),
        None
    );
    assert_eq!(
        state.press(Key::Character('q'), start + milliseconds(2998)),
        Some(Action::Quit)
    );
}

#[test]
fn should_answer_an_entry_one_time() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1"], start);
    assert_answers(
        state.press(Key::Character('a'), later),
        "1-1",
        Answer::ALLOW,
    );
    for key in ['a', 'd', 'm'] {
        assert_eq!(state.press(Key::Character(key), later), None);
        assert_eq!(state.notice(), Some("The prompt has its answer"));
    }
    assert_eq!(state.message(), None);
    assert_eq!(state.answer_of("1-1"), Some(&Answer::ALLOW));
}

#[test]
fn should_take_a_new_answer_after_the_queue_refused_one() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1"], start);
    state.press(Key::Character('a'), later);
    state.report_refused_answer("1-1", "Answer not stored");
    assert_eq!(state.answer_of("1-1"), None);
    assert_eq!(state.notice(), Some("Answer not stored"));
    assert_answers(
        state.press(Key::Character('d'), later + milliseconds(1000)),
        "1-1",
        Answer::Deny { message: None },
    );
}

#[test]
fn should_forget_the_answer_of_an_entry_that_left() {
    let start = Instant::now();
    let mut state = state_with(&["1-1", "2-1"], start);
    state.press(Key::Character('a'), start + milliseconds(5000));
    state.refresh(entries(&["2-1"]), start + milliseconds(5100));
    assert_eq!(state.answer_of("1-1"), None);
}

#[test]
fn should_deny_with_the_message_that_the_owner_typed_and_kept() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1"], start);
    assert_eq!(state.press(Key::Character('m'), later), None);
    assert_eq!(state.message(), Some(""));
    typed(&mut state, " Use a branch, qadm x", later);
    assert_eq!(state.press(Key::Backspace, later), None);
    assert_eq!(state.press(Key::Backspace, later), None);
    assert_eq!(state.message(), Some(" Use a branch, qadm"));
    assert_eq!(state.press(Key::Enter, later), None);
    assert_eq!(state.message(), None);
    assert_eq!(state.kept_message(), Some("Use a branch, qadm"));
    assert_eq!(state.answer_of("1-1"), None);
    assert_answers(
        state.press(Key::Character('d'), later + milliseconds(1000)),
        "1-1",
        Answer::Deny {
            message: Some("Use a branch, qadm".to_owned()),
        },
    );
}

#[test]
fn should_keep_no_message_when_the_typed_text_is_blank() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1"], start);
    state.press(Key::Character('m'), later);
    typed(&mut state, "  ", later);
    assert_eq!(state.press(Key::Enter, later), None);
    assert_eq!(state.kept_message(), None);
    assert_answers(
        state.press(Key::Character('d'), later + milliseconds(1000)),
        "1-1",
        Answer::Deny { message: None },
    );
}

#[test]
fn should_open_the_message_at_once() {
    let start = Instant::now();
    let mut state = state_with(&["1-1", "2-1"], start);
    assert_eq!(state.press(Key::Character('m'), start), None);
    assert_eq!(state.notice(), None);
    assert_eq!(state.message(), Some(""));
}

#[test]
fn should_take_no_answer_right_after_a_closed_message() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let closed = later + milliseconds(2000);
    let mut state = state_with(&["1-1", "2-1"], start);
    state.press(Key::Character('m'), later);
    typed(&mut state, "no, read ", later);
    assert_eq!(state.press(Key::Enter, closed), None);
    assert_eq!(
        state.press(Key::Character('a'), closed + milliseconds(999)),
        None
    );
    assert_eq!(state.answer_of("1-1"), None);
    assert_answers(
        state.press(Key::Character('a'), closed + milliseconds(1999)),
        "1-1",
        Answer::Allow {
            answers: None,
            message: Some("no, read".to_owned()),
        },
    );
}

#[test]
fn should_add_pasted_text_to_the_message_on_one_line() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1", "2-1"], start);
    state.press(Key::Character('m'), later);
    state.paste("no, read\r\nja\td", later);
    assert_eq!(state.message(), Some("no, read  ja d"));
    assert_eq!(state.answer_of("1-1"), None);
    state.paste(&"x".repeat(600), later);
    assert_eq!(
        state.message().map(|message| message.chars().count()),
        Some(500)
    );
}

#[test]
fn should_take_pasted_text_outside_a_message_as_no_key() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1", "2-1"], start);
    state.paste("jadq", later);
    assert_eq!(selected_id(&state), Some("1-1"));
    assert_eq!(state.answer_of("1-1"), None);
    assert_eq!(state.message(), None);
    assert_eq!(
        state.notice(),
        Some("Pasted text is no answer. The m key opens a message.")
    );
    assert_eq!(
        state.press(Key::Character('a'), later + milliseconds(999)),
        None
    );
    assert_eq!(state.answer_of("1-1"), None);
}

#[test]
fn should_close_the_message_with_escape_and_keep_its_text() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let closed = later + milliseconds(2000);
    let mut state = state_with(&["1-1", "2-1"], start);
    state.press(Key::Character('m'), later);
    typed(&mut state, "x", later);
    assert_eq!(state.press(Key::Escape, closed), None);
    assert_eq!(state.message(), None);
    assert_eq!(state.kept_message(), Some("x"));
    assert_eq!(state.press(Key::Down, closed), None);
    assert_eq!(state.kept_message(), None);
    assert_answers(
        state.press(Key::Character('a'), closed + milliseconds(1000)),
        "2-1",
        Answer::ALLOW,
    );
}

#[test]
fn should_keep_the_message_open_as_text_when_its_prompt_left() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let end = later + milliseconds(5000);
    let mut state = state_with(&["1-1", "2-1"], start);
    state.press(Key::Character('m'), later);
    typed(&mut state, "No", later);
    state.refresh(entries(&["2-1"]), later);
    typed(&mut state, " and a d", end);
    assert_eq!(state.message(), Some("No and a d"));
    assert_eq!(
        state.notice(),
        Some("The prompt left the queue. Esc closes the message.")
    );
    assert_eq!(state.press(Key::Enter, end), None);
    assert_eq!(state.message(), None);
    assert_eq!(state.press(Key::Character('a'), end), None);
    assert_eq!(state.answer_of("2-1"), None);
    assert_answers(
        state.press(Key::Character('a'), end + milliseconds(1000)),
        "2-1",
        Answer::ALLOW,
    );
}

#[test]
fn should_keep_the_message_at_500_characters_and_without_control_characters() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1"], start);
    state.press(Key::Character('m'), later);
    typed(&mut state, &"é".repeat(499), later);
    typed(&mut state, "\u{1b}\n", later);
    typed(&mut state, "xyz", later);
    assert_eq!(
        state.message(),
        Some(format!("{}x", "é".repeat(499)).as_str())
    );
}

#[test]
fn should_quit_on_q_outside_a_message() {
    let start = Instant::now();
    let mut state = state_with(&["1-1"], start);
    assert_eq!(
        state.press(Key::Character('q'), start + milliseconds(1000)),
        Some(Action::Quit)
    );
    assert_eq!(
        ListState::default().press(Key::Character('q'), start),
        Some(Action::Quit)
    );
}

#[test]
fn should_quit_on_the_interrupt_key_also_inside_a_message() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1"], start);
    assert_eq!(state.press(Key::Interrupt, start), Some(Action::Quit));
    state.press(Key::Character('m'), later);
    assert_eq!(state.press(Key::Character('q'), later), None);
    assert_eq!(state.message(), Some("q"));
    assert_eq!(state.press(Key::Interrupt, later), Some(Action::Quit));
}

#[test]
fn should_scroll_the_selected_entry_and_start_at_its_top_after_a_move() {
    let start = Instant::now();
    let mut state = state_with(&["1-1", "2-1"], start);
    state.limit_scroll(25);
    assert_eq!(state.scroll(), 0);
    state.press(Key::PageUp, start);
    assert_eq!(state.scroll(), 0);
    state.press(Key::PageDown, start);
    state.press(Key::PageDown, start);
    assert_eq!(state.scroll(), 20);
    state.press(Key::PageUp, start);
    assert_eq!(state.scroll(), 10);
    state.press(Key::Down, start);
    assert_eq!(state.scroll(), 0);
}

#[test]
fn should_scroll_no_further_than_the_last_line_of_the_selected_entry() {
    let start = Instant::now();
    let mut state = state_with(&["1-1"], start);
    state.press(Key::PageDown, start);
    assert_eq!(state.scroll(), 0);
    state.limit_scroll(25);
    for _ in 0..4 {
        state.press(Key::PageDown, start);
    }
    assert_eq!(state.scroll(), 25);
    state.limit_scroll(7);
    assert_eq!(state.scroll(), 7);
}

#[test]
fn should_mute_and_unmute_the_pings_after_the_quiet_time() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    for (key, action) in [
        ('s', Action::MutePings(Duration::from_secs(600))),
        ('S', Action::MutePings(Duration::from_secs(3600))),
        ('u', Action::UnmutePings),
    ] {
        let mut state = state_with(&["1-1"], start);
        assert_eq!(state.press(Key::Character(key), later), Some(action));
        assert_eq!(state.notice(), None);
        assert_eq!(
            state.press(Key::Character(key), later + milliseconds(999)),
            None
        );
        assert_eq!(
            state.notice(),
            Some("Too early. Press the key after 1 second without a key.")
        );
    }
}

#[test]
fn should_mute_the_pings_while_no_prompt_waits() {
    let start = Instant::now();
    let mut state = state_with(&[], start);
    assert_eq!(
        state.press(Key::Character('s'), start),
        Some(Action::MutePings(Duration::from_secs(600)))
    );
}

#[test]
fn should_name_a_key_without_a_function() {
    let start = Instant::now();
    let mut state = state_with(&["1-1"], start);
    for (key, notice) in [
        (Key::Character('x'), "Key x has no function"),
        (Key::Character('A'), "Key A has no function"),
        (Key::Character(' '), "Key space has no function"),
        (Key::Enter, "Key Enter has no function"),
        (Key::Escape, "Key Esc has no function"),
        (Key::Backspace, "Key Backspace has no function"),
        (Key::Other("Alt+a".to_owned()), "Key Alt+a has no function"),
    ] {
        state.press(key, start);
        assert_eq!(state.notice(), Some(notice));
    }
    state.press(Key::Down, start);
    assert_eq!(state.notice(), None);
}

#[test]
fn should_start_the_quiet_time_when_the_window_gets_the_focus() {
    let start = Instant::now();
    let later = start + milliseconds(5000);
    let mut state = state_with(&["1-1"], start);
    state.gain_focus(later);
    assert_eq!(
        state.press(Key::Character('a'), later + milliseconds(999)),
        None
    );
    let mut state = state_with(&["1-1"], start);
    state.gain_focus(later);
    assert_answers(
        state.press(Key::Character('a'), later + milliseconds(1000)),
        "1-1",
        Answer::ALLOW,
    );
}
