use std::{
    fs,
    time::{Duration, Instant, SystemTime},
};

use approval_queue::{
    action::Action, answer::Answer, entry::Entry, list_actions::carry_out, list_state::ListState,
    queue_folder::QueueFolder,
};
use serde_json::Map;
use tempfile::TempDir;

fn entry() -> Entry {
    Entry {
        received: 1_795_000_000,
        user: "agent".to_owned(),
        user_is_verified: false,
        host: "claude-code".to_owned(),
        session: "0a1b".to_owned(),
        folder: "/home/agent/git/pont".to_owned(),
        mode: "auto".to_owned(),
        tool: "Bash".to_owned(),
        title: None,
        risk: None,
        input: Map::new(),
        reason: None,
        list_only: false,
    }
}

fn queue() -> (TempDir, QueueFolder) {
    let runtime_folder = TempDir::new().unwrap();
    let queue = QueueFolder::in_runtime_folder(runtime_folder.path());
    queue.create().unwrap();
    (runtime_folder, queue)
}

fn time() -> SystemTime {
    SystemTime::UNIX_EPOCH + Duration::from_secs(1_795_000_000)
}

#[test]
fn should_mute_the_pings_from_now_and_show_the_mute() {
    let (_runtime_folder, queue) = queue();
    let mut state = ListState::default();
    let quits = carry_out(
        Action::MutePings(Duration::from_mins(10)),
        &queue,
        &mut state,
        time(),
    );
    assert!(!quits);
    let until = time() + Duration::from_mins(10);
    assert_eq!(queue.pings_muted_until(), Some(until));
    assert_eq!(state.pings_muted_until(), Some(until));
    assert_eq!(state.notice(), None);
}

#[test]
fn should_end_the_mute() {
    let (_runtime_folder, queue) = queue();
    let mut state = ListState::default();
    carry_out(
        Action::MutePings(Duration::from_hours(1)),
        &queue,
        &mut state,
        time(),
    );
    assert!(!carry_out(Action::UnmutePings, &queue, &mut state, time()));
    assert_eq!(queue.pings_muted_until(), None);
    assert_eq!(state.pings_muted_until(), None);
}

#[test]
fn should_show_a_mute_that_the_queue_cannot_store_as_a_notice() {
    let (runtime_folder, queue) = queue();
    let mute = runtime_folder
        .path()
        .join("approval-queue/pings-muted-until");
    fs::create_dir(&mute).unwrap();
    let mut state = ListState::default();
    let quits = carry_out(
        Action::MutePings(Duration::from_mins(10)),
        &queue,
        &mut state,
        time(),
    );
    assert!(!quits);
    assert!(state.notice().unwrap().starts_with("Mute "));
    assert_eq!(state.pings_muted_until(), None);
    let mut names: Vec<_> = fs::read_dir(runtime_folder.path().join("approval-queue"))
        .unwrap()
        .map(|item| item.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [
            "answers",
            "entries",
            "lists",
            "pings-muted-until",
            "risks",
            "sessions",
            "titles"
        ]
    );
}

#[test]
fn should_store_the_answer_of_the_owner() {
    let (_runtime_folder, queue) = queue();
    let waiting = queue.add(&entry()).unwrap();
    let mut state = ListState::default();
    let quits = carry_out(
        Action::Answer {
            id: waiting.id().to_owned(),
            answer: Answer::ALLOW,
        },
        &queue,
        &mut state,
        time(),
    );
    assert!(!quits);
    assert_eq!(queue.answer_for(waiting.id()).unwrap(), Some(Answer::ALLOW));
}

#[test]
fn should_show_a_refused_answer_as_a_notice() {
    let (_runtime_folder, queue) = queue();
    let mut state = ListState::default();
    state.refresh(Vec::new(), Instant::now());
    carry_out(
        Action::Answer {
            id: "1-1".to_owned(),
            answer: Answer::ALLOW,
        },
        &queue,
        &mut state,
        time(),
    );
    assert_eq!(state.notice(), Some("Prompt left the queue"));
}

#[test]
fn should_quit_the_list() {
    let (_runtime_folder, queue) = queue();
    assert!(carry_out(
        Action::Quit,
        &queue,
        &mut ListState::default(),
        time()
    ));
}

#[test]
fn should_show_an_unmute_that_fails_as_a_notice() {
    let (runtime_folder, queue) = queue();
    let mute = runtime_folder
        .path()
        .join("approval-queue/pings-muted-until");
    fs::create_dir(&mute).unwrap();
    let mut state = ListState::default();
    assert!(!carry_out(Action::UnmutePings, &queue, &mut state, time()));
    assert!(state.notice().unwrap().starts_with("Mute "));
}
