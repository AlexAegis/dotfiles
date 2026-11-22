use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use approval_queue::{
    action::Action, answer::Answer, entry::Entry, key::Key, list_state::ListState,
    listed_entry::ListedEntry, question::questions_of, question_draft::QuestionDraft,
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
    press(&mut state, '2', &mut time);
    press(&mut state, '1', &mut time);
    press(&mut state, '2', &mut time);
    time += Duration::from_secs(2);
    let Some(Action::Answer { answer, .. }) = state.press(Key::Enter, time) else {
        panic!("no answer");
    };
    assert!(answer.check().is_ok());
}

#[test]
fn should_answer_a_question_with_one_answer_by_one_option() {
    let mut draft = QuestionDraft::new("1-0".to_owned(), questions_of(&two_questions()).unwrap());
    assert_eq!(draft.position(), (1, 2));
    assert!(!draft.pick(0));
    assert!(!draft.pick(3));
    assert_eq!(draft.finished(), None);
    assert!(draft.pick(2));
    assert_eq!(draft.position(), (2, 2));
    assert_eq!(
        draft.answered().collect::<Vec<_>>(),
        [("Which library?", "luxon")]
    );
    assert_eq!(draft.finished(), None);
}

#[test]
fn should_answer_a_question_with_several_answers_by_its_picked_options_in_their_order() {
    let mut draft = QuestionDraft::new("1-0".to_owned(), questions_of(&two_questions()).unwrap());
    draft.pick(1);
    assert!(!draft.confirm());
    assert!(draft.pick(3));
    assert!(draft.pick(1));
    assert!(draft.pick(2));
    assert!(draft.pick(2));
    assert!(draft.is_picked(0) && !draft.is_picked(1) && draft.is_picked(2));
    assert_eq!(draft.finished(), None);
    assert!(draft.confirm());
    assert_eq!(
        draft.finished(),
        Some(answers(&[
            ("Which library?", "date-fns"),
            ("Which features?", "zones, ranges"),
        ]))
    );
}

#[test]
fn should_answer_a_question_with_an_own_text() {
    let mut draft = QuestionDraft::new("1-0".to_owned(), questions_of(&two_questions()).unwrap());
    assert_eq!(draft.text(), None);
    draft.open_text();
    assert!(!draft.send_text());
    for character in " day\u{7}js x".chars() {
        draft.push(character);
    }
    draft.pop();
    assert_eq!(draft.text(), Some(" dayjs "));
    assert!(draft.send_text());
    assert_eq!(draft.text(), None);
    assert_eq!(
        draft.answered().collect::<Vec<_>>(),
        [("Which library?", "dayjs")]
    );
    draft.open_text();
    for _ in 0..600 {
        draft.push('é');
    }
    assert_eq!(draft.text().unwrap().chars().count(), 500);
    draft.close_text();
    assert_eq!(draft.text(), None);
    assert_eq!(draft.position(), (2, 2));
}

#[test]
fn should_send_the_answers_of_the_list_after_the_last_question() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    assert_eq!(press(&mut state, '2', &mut time), None);
    assert_eq!(press(&mut state, '1', &mut time), None);
    assert_eq!(press(&mut state, '3', &mut time), None);
    time += Duration::from_secs(2);
    let action = state.press(Key::Enter, time);
    assert_eq!(
        action,
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: Answer::Allow {
                answers: Some(answers(&[
                    ("Which library?", "luxon"),
                    ("Which features?", "zones, ranges"),
                ])),
            },
        })
    );
    assert_eq!(press(&mut state, '1', &mut time), None);
    assert_eq!(state.notice(), Some("The prompt has its answer"));
}

#[test]
fn should_take_an_own_text_as_the_answer_of_the_list() {
    let (mut state, mut time) = state_with(vec![asked(&[question("Why?", &["a"], false)])]);
    assert_eq!(press(&mut state, 't', &mut time), None);
    assert_eq!(state.question_text(), Some(""));
    for character in "a1dq".chars() {
        assert_eq!(state.press(Key::Character(character), time), None);
    }
    state.paste(" and\nmore", time);
    assert_eq!(state.question_text(), Some("a1dq and more"));
    assert_eq!(state.press(Key::Backspace, time), None);
    let action = state.press(Key::Enter, time);
    assert_eq!(
        action,
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: Answer::Allow {
                answers: Some(answers(&[("Why?", "a1dq and mor")])),
            },
        })
    );
    assert_eq!(state.question_text(), None);
}

#[test]
fn should_refuse_a_key_that_answers_a_question_in_the_quiet_time() {
    let (mut state, time) = state_with(vec![asked(&[
        question("Which features?", &["zones", "locales"], true),
        question("Why?", &["a"], false),
    ])]);
    assert_eq!(state.press(Key::Character('j'), time), None);
    for key in [Key::Character('1'), Key::Character('t'), Key::Enter] {
        assert_eq!(state.press(key, time), None);
        assert_eq!(
            state.notice(),
            Some("Too early. Press the key after 1 second without a key.")
        );
    }
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
    assert_eq!(state.question_text(), None);
}

#[test]
fn should_name_the_keys_of_a_question_and_not_allow_it_with_a() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    assert_eq!(press(&mut state, 'a', &mut time), None);
    assert_eq!(
        state.notice(),
        Some("A question takes an answer: a number picks an option, t opens a text.")
    );
    assert_eq!(press(&mut state, '3', &mut time), None);
    assert_eq!(
        state.notice(),
        Some("The question has no option with this number")
    );
    assert_eq!(press(&mut state, '1', &mut time), None);
    time += Duration::from_secs(2);
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.notice(), Some("Pick an option with its number first"));
}

#[test]
fn should_start_the_questions_again_at_escape_and_decline_as_each_prompt() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    press(&mut state, '1', &mut time);
    assert_eq!(state.question_draft().unwrap().position(), (2, 2));
    time += Duration::from_secs(2);
    assert_eq!(state.press(Key::Escape, time), None);
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
    assert_eq!(
        press(&mut state, 'd', &mut time),
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: Answer::Deny { message: None },
        })
    );

    let (mut state, mut time) = state_with(vec![two_questions()]);
    press(&mut state, 'm', &mut time);
    state.press(Key::Character('1'), time);
    assert_eq!(
        state.press(Key::Enter, time),
        Some(Action::Answer {
            id: "1-0".to_owned(),
            answer: Answer::Deny {
                message: Some("1".to_owned())
            },
        })
    );
}

#[test]
fn should_keep_the_answers_so_far_at_an_escape_in_the_quiet_time() {
    let (mut state, mut time) = state_with(vec![two_questions(), two_questions()]);
    press(&mut state, '1', &mut time);
    time += Duration::from_secs(2);
    state.gain_focus(time);
    time += Duration::from_millis(10);
    assert_eq!(state.press(Key::Escape, time), None);
    assert_eq!(
        state.notice(),
        Some("Too early. Press the key after 1 second without a key.")
    );
    assert_eq!(state.question_draft().unwrap().position(), (2, 2));

    press(&mut state, 'j', &mut time);
    let first = ListedEntry {
        id: "1-0".to_owned(),
        entry: two_questions(),
    };
    time += Duration::from_secs(2);
    state.refresh(vec![first], time);
    time += Duration::from_millis(10);
    state.press(Key::Escape, time);
    assert_eq!(state.question_draft().unwrap().id(), "1-0");
    assert_eq!(state.question_draft().unwrap().position(), (2, 2));
}

#[test]
fn should_take_no_key_in_an_open_own_text_right_after_a_new_focus() {
    let (mut state, mut time) = state_with(vec![asked(&[question("Why?", &["a"], false)])]);
    press(&mut state, 't', &mut time);
    state.press(Key::Character('x'), time);
    state.gain_focus(time);
    time += Duration::from_millis(10);
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(
        state.notice(),
        Some("Too early. Press the key after 1 second without a key.")
    );
    state.paste("y", time);
    assert_eq!(state.question_text(), Some("x"));
    time += Duration::from_secs(2);
    assert!(matches!(
        state.press(Key::Enter, time),
        Some(Action::Answer { .. })
    ));
}

#[test]
fn should_show_the_next_question_after_an_own_text_and_keep_a_blank_text_open() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    state.limit_scroll(30);
    time += Duration::from_secs(2);
    state.press(Key::PageDown, time);
    press(&mut state, 't', &mut time);
    state.press(Key::Character(' '), time);
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.question_text(), Some(" "));
    assert_eq!(state.scroll(), 10);
    state.press(Key::Character('x'), time);
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.question_text(), None);
    assert_eq!(state.question_draft().unwrap().position(), (2, 2));
    assert_eq!(state.scroll(), 0);
}

#[test]
fn should_name_enter_as_a_key_without_a_function_for_a_question_with_one_answer() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    time += Duration::from_secs(2);
    assert_eq!(state.press(Key::Enter, time), None);
    assert_eq!(state.notice(), Some("Key Enter has no function"));
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
fn should_drop_the_answers_so_far_when_the_prompt_leaves() {
    let (mut state, mut time) = state_with(vec![two_questions(), two_questions()]);
    press(&mut state, '1', &mut time);
    assert_eq!(state.question_draft().unwrap().position(), (2, 2));
    let second = ListedEntry {
        id: "1-1".to_owned(),
        entry: two_questions(),
    };
    state.refresh(vec![second], time);
    assert_eq!(state.question_draft().unwrap().id(), "1-1");
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
    let both = ["1-0", "1-1"].map(|id| ListedEntry {
        id: id.to_owned(),
        entry: two_questions(),
    });
    state.refresh(both.to_vec(), time);
    press(&mut state, 'k', &mut time);
    assert_eq!(state.question_draft().unwrap().id(), "1-0");
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
}

#[test]
fn should_check_the_answers_of_an_allow() {
    let allow = |pairs: &[(&str, &str)]| Answer::Allow {
        answers: Some(answers(pairs)),
    };
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
    assert_eq!(
        serde_json::to_string(&allow(&[("Why?", "a")])).unwrap(),
        r#"{"behavior":"allow","answers":{"Why?":"a"}}"#
    );
    assert_eq!(
        serde_json::to_string(&Answer::ALLOW).unwrap(),
        r#"{"behavior":"allow"}"#
    );
    assert_eq!(
        serde_json::from_str::<Answer>(r#"{"behavior":"allow"}"#).unwrap(),
        Answer::ALLOW
    );
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
fn should_keep_the_answers_so_far_of_each_prompt() {
    let (mut state, mut time) = state_with(vec![two_questions(), two_questions()]);
    press(&mut state, '1', &mut time);
    press(&mut state, 'j', &mut time);
    press(&mut state, '2', &mut time);
    assert_eq!(state.question_draft().unwrap().position(), (2, 2));
    time += Duration::from_secs(2);
    state.press(Key::Escape, time);
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
    press(&mut state, 'k', &mut time);
    let first = state.question_draft().unwrap();
    assert_eq!(first.id(), "1-0");
    assert_eq!(
        first.answered().collect::<Vec<_>>(),
        [("Which library?", "date-fns")]
    );
}

#[test]
fn should_show_the_next_question_from_the_top_of_the_pane() {
    let (mut state, mut time) = state_with(vec![two_questions()]);
    state.limit_scroll(30);
    time += Duration::from_secs(2);
    state.press(Key::PageDown, time);
    assert_eq!(state.scroll(), 10);
    press(&mut state, '1', &mut time);
    assert_eq!(state.scroll(), 0);
    state.press(Key::PageDown, time);
    press(&mut state, '3', &mut time);
    assert_eq!(state.scroll(), 10);
    time += Duration::from_secs(2);
    state.press(Key::Escape, time);
    assert_eq!(state.scroll(), 0);
}

#[test]
fn should_list_the_answers_so_far_in_the_order_of_the_questions() {
    let mut draft = QuestionDraft::new(
        "1-0".to_owned(),
        questions_of(&asked(&[
            question("Which library?", &["luxon"], false),
            question("Which features?", &["zones"], false),
            question("Why?", &["a"], false),
        ]))
        .unwrap(),
    );
    draft.pick(1);
    draft.pick(1);
    assert_eq!(
        draft.answered().collect::<Vec<_>>(),
        [("Which library?", "luxon"), ("Which features?", "zones")]
    );
}

#[test]
fn should_name_the_text_key_for_pasted_text_on_a_question() {
    let (mut state, time) = state_with(vec![two_questions()]);
    state.paste("1", time);
    assert_eq!(
        state.notice(),
        Some("Pasted text is no answer. The t key opens a text.")
    );
    assert_eq!(state.question_draft().unwrap().position(), (1, 2));
}
