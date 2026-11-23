use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use approval_queue::{
    action::Action,
    answer::Answer,
    entry::Entry,
    key::Key,
    list_state::ListState,
    listed_entry::ListedEntry,
    question::questions_of,
    question_draft::{QuestionDraft, Step},
};
use serde_json::{Value, json};

fn entry(tool: &str, input: &Value) -> Entry {
    Entry {
        received: 0,
        user: "agent".to_owned(),
        user_is_verified: false,
        host: "claude-code".to_owned(),
        session: "0a1b".to_owned(),
        folder: "/home/agent".to_owned(),
        mode: "auto".to_owned(),
        tool: tool.to_owned(),
        title: None,
        risk: None,
        input: input.as_object().unwrap().clone(),
        reason: None,
        list_only: false,
    }
}

fn question(text: &str, labels: &[&str], multi_select: bool) -> Value {
    let options: Vec<Value> = labels
        .iter()
        .map(|label| json!({ "label": label, "description": format!("about {label}") }))
        .collect();
    json!({ "question": text, "header": "Pick", "options": options, "multiSelect": multi_select })
}

fn asked(questions: &[Value]) -> Entry {
    entry("AskUserQuestion", &json!({ "questions": questions }))
}

fn two_questions() -> Entry {
    asked(&[
        question("Which library?", &["date-fns", "luxon"], false),
        question("Which features?", &["zones", "locales", "ranges"], true),
    ])
}

fn answers(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(question, answer)| ((*question).to_owned(), (*answer).to_owned()))
        .collect()
}

fn state_with(entries: Vec<Entry>) -> (ListState, Instant) {
    let start = Instant::now();
    let mut state = ListState::default();
    let listed = entries
        .into_iter()
        .enumerate()
        .map(|(index, entry)| ListedEntry {
            id: format!("1-{index}"),
            entry,
        })
        .collect();
    state.refresh(listed, start);
    (state, start + Duration::from_secs(2))
}

fn press(state: &mut ListState, key: char, time: &mut Instant) -> Option<Action> {
    *time += Duration::from_secs(2);
    state.press(Key::Character(key), *time)
}

#[test]
fn should_read_the_questions_of_a_question_prompt() {
    let questions = questions_of(&two_questions()).unwrap();
    assert_eq!(questions.len(), 2);
    assert_eq!(questions[0].question, "Which library?");
    assert_eq!(questions[0].header, "Pick");
    assert_eq!(questions[0].options[1].label, "luxon");
    assert_eq!(questions[0].options[1].description, "about luxon");
    assert!(!questions[0].multi_select);
    assert!(questions[1].multi_select);
    let bare = asked(&[json!({ "question": "Why?" })]);
    assert_eq!(questions_of(&bare).unwrap()[0].options, []);
    let nine: Vec<String> = (1..=9).map(|number| number.to_string()).collect();
    let nine: Vec<&str> = nine.iter().map(String::as_str).collect();
    let four: Vec<Value> = (0..4)
        .map(|number| question(&format!("Q{number}?"), &nine, false))
        .collect();
    assert_eq!(questions_of(&asked(&four)).unwrap().len(), 4);
}

#[test]
fn should_read_no_questions_that_the_list_cannot_answer() {
    let one = question("Why?", &["a"], false);
    let many_options: Vec<String> = (0..10).map(|number| number.to_string()).collect();
    let many_options: Vec<&str> = many_options.iter().map(String::as_str).collect();
    let five: Vec<Value> = (0..5)
        .map(|number| question(&format!("Q{number}?"), &["a"], false))
        .collect();
    for input in [
        json!({}),
        json!({ "questions": [] }),
        json!({ "questions": "Why?" }),
        json!({ "questions": [{ "header": "no text" }] }),
        json!({ "questions": [question("", &["a"], false)] }),
        json!({ "questions": [question("Why?", &[""], false)] }),
        json!({ "questions": [one.clone(), one.clone()] }),
        json!({ "questions": [question("Why?", &many_options, false)] }),
        json!({ "questions": five }),
        json!({ "questions": [question("Why?", &["yes\u{1b}[2J"], false)] }),
        json!({ "questions": [question("Why?", &[&"x".repeat(501)], false)] }),
        json!({ "questions": [question("Why?", &[&"x".repeat(249), &"x".repeat(250)], true)] }),
    ] {
        assert_eq!(
            questions_of(&entry("AskUserQuestion", &input)),
            None,
            "{input}"
        );
    }
    let other_tool = entry("Bash", &json!({ "questions": [one] }));
    assert_eq!(questions_of(&other_tool), None);
}

#[test]
fn should_read_the_questions_whose_longest_answer_the_queue_takes() {
    let longest = "x".repeat(500);
    let halves = ["x".repeat(249), "x".repeat(249)];
    let (mut state, mut time) = state_with(vec![asked(&[
        question("One?", &[&longest, &longest], false),
        question("Several?", &[&halves[0], &halves[1]], true),
    ])]);
    for key in ['2', '\n', '1', '2'] {
        if key == '\n' {
            state.press(Key::Enter, time);
        } else {
            state.press(Key::Character(key), time);
        }
    }
    time += Duration::from_secs(2);
    let Some(Action::Answer { answer, .. }) = state.press(Key::Enter, time) else {
        panic!("no answer");
    };
    assert!(answer.check().is_ok());
}

const TOO_EARLY: &str = "Too early. Press the key after 1 second without a key.";

fn draft_of(entry: &Entry) -> QuestionDraft {
    QuestionDraft::new("1-0".to_owned(), questions_of(entry).unwrap())
}

fn allow(pairs: &[(&str, &str)], message: Option<&str>) -> Answer {
    Answer::Allow {
        answers: Some(answers(pairs)),
        message: message.map(str::to_owned),
    }
}

fn type_text(state: &mut ListState, text: &str, time: Instant) {
    for character in text.chars() {
        state.press(Key::Character(character), time);
    }
}

#[test]
fn should_mark_one_option_of_a_question_with_one_answer_and_stay() {
    let mut draft = draft_of(&two_questions());
    assert!(!draft.pick(0));
    assert!(!draft.pick(3));
    assert!(draft.pick(1));
    assert!(draft.pick(2));
    assert!(!draft.is_picked(0));
    assert!(draft.is_picked(1));
    assert_eq!(draft.position(), (1, 2));
    assert!(draft.pick(2));
    assert!(!draft.is_picked(1));
    assert!(matches!(draft.go_on(), Step::Refused(_)));
    assert_eq!(draft.position(), (1, 2));
}

#[test]
fn should_answer_a_question_with_several_answers_by_its_marked_options_in_their_order() {
    let mut draft = draft_of(&two_questions());
    draft.pick(1);
    assert_eq!(draft.go_on(), Step::Next);
    assert!(draft.is_at_last());
    assert!(matches!(draft.go_on(), Step::Refused(_)));
    for number in [3, 1, 2, 2] {
        assert!(draft.pick(number));
    }
    draft.open_text();
    for character in " own ".chars() {
        draft.push(character);
    }
    draft.keep_text();
    assert_eq!(draft.own_answer(), Some("own"));
    assert_eq!(
        draft.go_on(),
        Step::Finished(answers(&[
            ("Which library?", "date-fns"),
            ("Which features?", "zones, ranges, own"),
        ]))
    );
}

#[test]
fn should_keep_an_own_text_as_the_one_answer_of_a_question() {
    let mut draft = draft_of(&two_questions());
    draft.pick(1);
    draft.open_text();
    for character in "dayjs\u{1b}".chars() {
        draft.push(character);
    }
    draft.close_text();
    assert_eq!(draft.own_answer(), None);
    assert!(draft.is_picked(0));
    draft.open_text();
    for _ in 0..600 {
        draft.push('é');
    }
    assert_eq!(draft.text().unwrap().chars().count(), 500);
    draft.keep_text();
    assert!(!draft.is_picked(0));
    draft.open_text();
    assert_eq!(draft.text().unwrap().chars().count(), 500);
    for _ in 0..500 {
        draft.pop();
    }
    draft.push(' ');
    draft.keep_text();
    assert_eq!(draft.own_answer(), None);
    assert!(matches!(draft.go_on(), Step::Refused(_)));
    draft.open_text();
    draft.push('x');
    draft.keep_text();
    draft.pick(2);
    assert_eq!(draft.own_answer(), None);
    assert!(draft.is_picked(1));
}

#[test]
fn should_refuse_marks_and_an_own_answer_that_are_longer_than_the_longest_answer() {
    let label = "x".repeat(400);
    let mut draft = draft_of(&asked(&[question("Why?", &[&label], true)]));
    draft.pick(1);
    draft.open_text();
    for _ in 0..200 {
        draft.push('y');
    }
    draft.keep_text();
    assert_eq!(
        draft.go_on(),
        Step::Refused("The marked options and the own answer are longer than 500 characters")
    );
}

#[test]
fn should_go_back_to_the_question_before_and_keep_its_marks() {
    let mut draft = draft_of(&two_questions());
    assert!(!draft.go_back());
    draft.pick(2);
    draft.go_on();
    draft.pick(1);
    assert_eq!(
        draft.answered().collect::<Vec<_>>(),
        [("Which library?", "luxon".to_owned())]
    );
    assert!(draft.go_back());
    assert_eq!(draft.position(), (1, 2));
    assert!(draft.is_picked(1));
    assert_eq!(draft.answered().count(), 0);
    draft.go_on();
    assert!(draft.is_picked(0));
}

#[test]
fn should_mark_and_unmark_options_at_once_and_send_only_after_the_quiet_time() {
    let (mut state, time) = state_with(vec![two_questions()]);
    state.gain_focus(time);
    for key in ['2', '1', '1', '2'] {
        assert_eq!(state.press(Key::Character(key), time), None);
        assert_eq!(state.notice(), None);
    }
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.question_draft().unwrap().position(), (2, 2));
    for key in ['1', '3', '1'] {
        state.press(Key::Character(key), time);
    }
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.notice(), Some(TOO_EARLY));
    assert_eq!(state.answer_of("1-0"), None);
    let later = time + Duration::from_secs(1);
    let expected = allow(
        &[("Which library?", "luxon"), ("Which features?", "ranges")],
        None,
    );
    assert_eq!(
        state.press(Key::Enter, later),
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: expected.clone(),
        })
    );
    assert_eq!(state.answer_of("1-0"), Some(&expected));
    assert_eq!(state.press(Key::Character('1'), later), None);
    assert_eq!(state.notice(), Some("The prompt has its answer"));
}

#[test]
fn should_not_go_on_from_a_question_without_a_mark() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    time += Duration::from_secs(2);
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(
        state.notice(),
        Some("Mark an option with its number first, or give an own answer with t")
    );
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
    press(&mut state, '7', &mut time);
    assert_eq!(
        state.notice(),
        Some("The question has no option with this number")
    );
}

#[test]
fn should_keep_an_own_text_with_enter_and_send_it_with_the_last_enter() {
    let (mut state, time) = state_with(vec![asked(&[question("Why?", &["a"], false)])]);
    assert_eq!(state.press(Key::Character('t'), time), None);
    assert_eq!(state.question_text(), Some(""));
    type_text(&mut state, "a1dq", time);
    state.paste(" and\nmore", time);
    state.press(Key::Backspace, time);
    assert_eq!(state.question_text(), Some("a1dq and mor"));
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.question_text(), None);
    assert_eq!(state.answer_of("1-0"), None);
    assert_eq!(
        state.question_draft().unwrap().own_answer(),
        Some("a1dq and mor")
    );
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.notice(), Some(TOO_EARLY));
    assert_eq!(
        state.press(Key::Enter, time + Duration::from_secs(1)),
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: allow(&[("Why?", "a1dq and mor")], None),
        })
    );
}

#[test]
fn should_close_an_own_text_with_escape_without_a_change() {
    let (mut state, time) = state_with(vec![asked(&[question("Why?", &["a"], false)])]);
    state.press(Key::Character('1'), time);
    assert_eq!(state.press(Key::Character('t'), time), None);
    assert_eq!(state.notice(), None);
    type_text(&mut state, "x", time);
    assert_eq!(state.question_text(), Some("x"));
    assert_eq!(state.press(Key::Escape, time), None);
    assert_eq!(state.question_text(), None);
    let draft = state.question_draft().unwrap();
    assert_eq!(draft.own_answer(), None);
    assert!(draft.is_picked(0));
}

#[test]
fn should_name_the_keys_of_a_question_and_not_allow_it_with_a() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    assert_eq!(press(&mut state, 'a', &mut time), None);
    assert_eq!(
        state.notice(),
        Some(
            "A question takes an answer: a number marks an option, t opens a text, Enter goes on."
        )
    );
    assert_eq!(state.answer_of("1-0"), None);
}

#[test]
fn should_go_back_at_escape_and_decline_as_a_permission_prompt() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    assert_eq!(state.press(Key::Escape, time), None);
    assert_eq!(state.notice(), Some("Key Esc has no function"));
    state.press(Key::Character('1'), time);
    state.press(Key::Enter, time);
    assert_eq!(state.question_draft().unwrap().position(), (2, 2));
    assert_eq!(state.press(Key::Escape, time), None);
    let draft = state.question_draft().unwrap();
    assert_eq!(draft.position(), (1, 2));
    assert!(draft.is_picked(0));
    assert_eq!(state.press(Key::Character('d'), time), None);
    assert_eq!(state.notice(), Some(TOO_EARLY));
    assert_eq!(
        press(&mut state, 'd', &mut time),
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: Answer::Deny { message: None },
        })
    );
}

#[test]
fn should_take_no_key_in_an_open_own_text_right_after_a_new_focus() {
    let (mut state, mut time) = state_with(vec![asked(&[question("Why?", &["a"], false)])]);
    state.press(Key::Character('t'), time);
    state.press(Key::Character('x'), time);
    state.gain_focus(time);
    time += Duration::from_millis(10);
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.notice(), Some(TOO_EARLY));
    state.paste("y", time);
    assert_eq!(state.question_text(), Some("x"));
    time += Duration::from_secs(2);
    state.press(Key::Character('z'), time);
    assert_eq!(state.question_text(), Some("xz"));
}

#[test]
fn should_keep_the_keys_of_a_prompt_without_questions() {
    let bash = entry("Bash", &json!({ "command": "ls" }));
    let unreadable = entry("AskUserQuestion", &json!({ "questions": [] }));
    for prompt in [bash, unreadable] {
        let (mut state, mut time) = state_with(vec![prompt]);
        assert_eq!(state.question_draft(), None);
        assert_eq!(press(&mut state, '1', &mut time), None);
        assert_eq!(state.notice(), Some("Key 1 has no function"));
        assert_eq!(
            press(&mut state, 'a', &mut time),
            Some(Action::Answer {
                id: "1-0".to_owned(),
                answer: Answer::ALLOW,
            })
        );
    }
}

#[test]
fn should_keep_the_marks_of_each_prompt_and_drop_them_when_the_prompt_leaves() {
    let (mut state, time) = state_with(vec![two_questions(), two_questions()]);
    state.press(Key::Character('1'), time);
    state.press(Key::Enter, time);
    state.press(Key::Character('j'), time);
    state.press(Key::Character('2'), time);
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
    state.press(Key::Character('k'), time);
    let first = state.question_draft().unwrap();
    assert_eq!(first.id(), "1-0");
    assert_eq!(first.position(), (2, 2));
    let both = ["1-0", "1-1"].map(|id| ListedEntry {
        id: id.to_owned(),
        entry: two_questions(),
    });
    state.refresh(both[1..].to_vec(), time);
    state.refresh(both.to_vec(), time);
    state.press(Key::Character('k'), time);
    assert_eq!(state.question_draft().unwrap().id(), "1-0");
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
    state.press(Key::Character('j'), time);
    assert!(state.question_draft().unwrap().is_picked(1));
}

#[test]
fn should_keep_an_own_text_open_when_its_prompt_leaves() {
    let command = entry("Bash", &json!({ "command": "ls" }));
    let (mut state, mut time) = state_with(vec![two_questions(), command.clone()]);
    press(&mut state, 't', &mut time);
    state.press(Key::Character('x'), time);
    let left = || {
        vec![ListedEntry {
            id: "1-1".to_owned(),
            entry: command.clone(),
        }]
    };
    state.refresh(left(), time);
    assert_eq!(
        state.notice(),
        Some("The prompt left the queue. Esc closes the text.")
    );
    assert_eq!(press(&mut state, 'a', &mut time), None);
    assert_eq!(press(&mut state, '1', &mut time), None);
    assert_eq!(state.question_text(), Some("xa1"));
    state.paste("d\n", time);
    assert_eq!(state.question_text(), Some("xa1d "));
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.question_text(), None);
    assert_eq!(state.notice(), None);
    assert_eq!(state.press(Key::Character('a'), time), None);
    assert_eq!(state.answer_of("1-1"), None);

    let (mut state, mut time) = state_with(vec![two_questions(), command.clone()]);
    press(&mut state, 't', &mut time);
    state.refresh(left(), time);
    assert_eq!(state.press(Key::Escape, time), None);
    assert_eq!(state.question_text(), None);
    assert!(matches!(
        press(&mut state, 'a', &mut time),
        Some(Action::Answer { id, answer: Answer::ALLOW }) if id == "1-1"
    ));
}

#[test]
fn should_show_another_question_from_the_top_of_the_pane() {
    let (mut state, time) = state_with(vec![two_questions()]);
    state.limit_scroll(30);
    state.press(Key::PageDown, time);
    state.press(Key::Character('1'), time);
    assert_eq!(state.scroll(), 10);
    state.press(Key::Enter, time);
    assert_eq!(state.scroll(), 0);
    state.press(Key::PageDown, time);
    state.press(Key::Character('3'), time);
    assert_eq!(state.scroll(), 10);
    state.press(Key::Escape, time);
    assert_eq!(state.scroll(), 0);
}

#[test]
fn should_name_the_text_key_for_pasted_text_on_a_question() {
    let (mut state, time) = state_with(vec![two_questions()]);
    state.paste("1", time);
    assert_eq!(
        state.notice(),
        Some("Pasted text is no answer. The t key opens a text.")
    );
    assert!(!state.question_draft().unwrap().is_picked(0));
}

#[test]
fn should_send_the_kept_message_with_an_approval_a_decline_and_the_answers() {
    let command = entry("Bash", &json!({ "command": "ls" }));
    let (mut state, mut time) = state_with(vec![
        command.clone(),
        command,
        asked(&[question("Why?", &["a"], false)]),
    ]);
    for _ in 0..3 {
        assert_eq!(press(&mut state, 'm', &mut time), None);
        assert_eq!(state.message(), Some(""));
        type_text(&mut state, " use staging ", time);
        assert_eq!(state.press(Key::Enter, time), None);
        assert_eq!(state.message(), None);
        assert_eq!(state.kept_message(), Some("use staging"));
        state.press(Key::Character('j'), time);
    }
    let message = Some("use staging".to_owned());
    state.press(Key::Character('k'), time);
    state.press(Key::Character('k'), time);
    assert_eq!(
        press(&mut state, 'a', &mut time),
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: Answer::Allow {
                answers: None,
                message: message.clone(),
            },
        })
    );
    press(&mut state, 'j', &mut time);
    assert_eq!(
        press(&mut state, 'd', &mut time),
        Some(Action::Answer {
            id: "1-1".to_owned(),
            answer: Answer::Deny {
                message: message.clone()
            },
        })
    );
    press(&mut state, 'j', &mut time);
    press(&mut state, '1', &mut time);
    time += Duration::from_secs(2);
    assert_eq!(
        state.press(Key::Enter, time),
        Some(Action::Answer {
            id: "1-2".to_owned(),
            answer: allow(&[("Why?", "a")], Some("use staging")),
        })
    );
}

#[test]
fn should_open_the_kept_message_again_and_take_it_away_as_an_empty_text() {
    let (mut state, mut time) = state_with(vec![entry("Bash", &json!({ "command": "ls" }))]);
    state.press(Key::Character('m'), time);
    type_text(&mut state, "ad", time);
    assert_eq!(state.press(Key::Escape, time), None);
    assert_eq!(state.kept_message(), Some("ad"));
    assert_eq!(state.answer_of("1-0"), None);
    press(&mut state, 'm', &mut time);
    assert_eq!(state.message(), Some("ad"));
    state.press(Key::Backspace, time);
    state.press(Key::Backspace, time);
    state.press(Key::Enter, time);
    assert_eq!(state.kept_message(), None);
    assert_eq!(state.press(Key::Character('a'), time), None);
    assert_eq!(state.notice(), Some(TOO_EARLY));
    assert_eq!(
        press(&mut state, 'a', &mut time),
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: Answer::ALLOW,
        })
    );
}

#[test]
fn should_drop_the_message_of_a_prompt_that_left() {
    let command = entry("Bash", &json!({ "command": "ls" }));
    let (mut state, mut time) = state_with(vec![command.clone(), command.clone()]);
    state.press(Key::Character('m'), time);
    type_text(&mut state, "x", time);
    let second = || {
        vec![ListedEntry {
            id: "1-1".to_owned(),
            entry: command.clone(),
        }]
    };
    state.refresh(second(), time);
    assert_eq!(
        state.notice(),
        Some("The prompt left the queue. Esc closes the message.")
    );
    type_text(&mut state, "a", time);
    assert_eq!(state.message(), Some("xa"));
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.message(), None);
    assert_eq!(state.kept_message(), None);
    assert_eq!(
        press(&mut state, 'a', &mut time),
        Some(Action::Answer {
            id: "1-1".to_owned(),
            answer: Answer::ALLOW,
        })
    );
}

#[test]
fn should_check_the_answers_and_the_message_of_an_allow() {
    let allow = |pairs: &[(&str, &str)]| allow(pairs, None);
    assert!(Answer::ALLOW.check().is_ok());
    assert!(allow(&[("Why?", "a, b")]).check().is_ok());
    assert!(allow(&[]).check().is_err());
    assert!(allow(&[("Why?", "")]).check().is_err());
    assert!(allow(&[("Why?", "a\nb")]).check().is_err());
    assert!(allow(&[("Why?", &"é".repeat(500))]).check().is_ok());
    assert!(allow(&[("Why?", &"é".repeat(501))]).check().is_err());
    assert!(
        allow(&[("1", "a"), ("2", "a"), ("3", "a"), ("4", "a")])
            .check()
            .is_ok()
    );
    assert!(
        allow(&[("1", "a"), ("2", "a"), ("3", "a"), ("4", "a"), ("5", "a")])
            .check()
            .is_err()
    );
    let with_message = |message: &str| Answer::Allow {
        answers: None,
        message: Some(message.to_owned()),
    };
    assert!(with_message(&"é".repeat(500)).check().is_ok());
    assert!(with_message(&"é".repeat(501)).check().is_err());
    assert!(with_message("a\nb").check().is_err());
    assert_eq!(
        serde_json::to_string(&self::allow(&[("Why?", "a")], Some("b"))).unwrap(),
        r#"{"behavior":"allow","answers":{"Why?":"a"},"message":"b"}"#
    );
    assert_eq!(
        serde_json::to_string(&Answer::ALLOW).unwrap(),
        r#"{"behavior":"allow"}"#
    );
    assert_eq!(
        serde_json::from_str::<Answer>(r#"{"behavior":"allow"}"#).unwrap(),
        Answer::ALLOW
    );
    assert_eq!(
        serde_json::from_str::<Answer>(r#"{"behavior":"allow","message":"b"}"#).unwrap(),
        with_message("b")
    );
}

#[test]
fn should_keep_the_marks_and_the_message_after_a_refused_answer() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    press(&mut state, 'm', &mut time);
    type_text(&mut state, "note", time);
    state.press(Key::Enter, time);
    state.press(Key::Character('2'), time);
    state.press(Key::Enter, time);
    state.press(Key::Character('1'), time);
    time += Duration::from_secs(2);
    assert!(state.press(Key::Enter, time).is_some());
    state.report_refused_answer("1-0", "Answer not stored");
    let draft = state.question_draft().unwrap();
    assert_eq!(draft.position(), (2, 2));
    assert!(draft.is_picked(0));
    assert_eq!(state.kept_message(), Some("note"));
    time += Duration::from_secs(2);
    assert_eq!(
        state.press(Key::Enter, time),
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: allow(
                &[("Which library?", "luxon"), ("Which features?", "zones")],
                Some("note"),
            ),
        })
    );
}

#[test]
fn should_name_marks_and_an_own_answer_that_are_too_long_and_stay() {
    let label = "x".repeat(400);
    let (mut state, mut time) = state_with(vec![asked(&[
        question("Why?", &[&label], true),
        question("How?", &["a"], false),
    ])]);
    state.press(Key::Character('1'), time);
    press(&mut state, 't', &mut time);
    state.paste(&"y".repeat(200), time);
    state.press(Key::Enter, time);
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(
        state.notice(),
        Some("The marked options and the own answer are longer than 500 characters")
    );
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
}

#[test]
fn should_take_no_key_of_a_question_for_a_prompt_that_has_its_answer() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    state.press(Key::Character('1'), time);
    assert!(press(&mut state, 'd', &mut time).is_some());
    for key in [Key::Enter, Key::Character('2'), Key::Character('t')] {
        time += Duration::from_secs(2);
        assert_eq!(state.press(key, time), None);
        assert_eq!(state.notice(), Some("The prompt has its answer"));
    }
    let draft = state.question_draft().unwrap();
    assert_eq!(draft.position(), (1, 2));
    assert!(draft.is_picked(0));
    assert_eq!(state.question_text(), None);
}
