use std::{
    fs,
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    process::{self, Command},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use approval_queue::{
    log_text,
    queue_folder::{MAX_SESSIONS_OF_USER, QueueFolder},
    service::{Sender, Served, Service},
    session_state::{Activity, Change, ListedSession, ReportedState, SessionReport, SessionState},
};
use serde_json::{Value, json};
use tempfile::TempDir;

const OWNER: &str = "alex";
const SESSION: &str = "0a1b2c3d-0000-4000-8000-000000000001";
const NO_PROCESS: u32 = u32::MAX;

fn report(state: &str) -> Value {
    json!({
        "version": 1,
        "kind": "session",
        "host": "claude-code",
        "user": "agent",
        "session": SESSION,
        "folder": "/home/agent/git/pont",
        "pid": process::id(),
        "state": state,
        "sent_at": 1000,
    })
}

fn with(mut report: Value, key: &str, value: Value) -> Value {
    report[key] = value;
    report
}

fn parsed(report: &Value) -> Result<SessionReport> {
    SessionReport::parse(&report.to_string())
}

fn stored(change: Change) -> SessionState {
    match change {
        Change::Store(state) => state,
        other => panic!("stored no state: {other:?}"),
    }
}

fn state_of(report: &Value, now: u64) -> SessionState {
    stored(SessionState::after(
        None,
        &parsed(report).unwrap(),
        now,
        false,
    ))
}

fn queue() -> (TempDir, QueueFolder) {
    let runtime_folder = TempDir::new().unwrap();
    let queue = QueueFolder::in_runtime_folder(runtime_folder.path());
    queue.create().unwrap();
    (runtime_folder, queue)
}

struct Sent {
    served: Result<Served>,
    reply: String,
    sessions: Vec<ListedSession>,
}

fn sent(queue: &QueueFolder, line: &Value, sender_is_own_user: bool) -> Sent {
    let sender = Sender {
        uid: if sender_is_own_user { 1000 } else { 1001 },
        pid: process::id(),
    };
    sent_by(queue, line, sender, 1000)
}

fn sent_by(queue: &QueueFolder, line: &Value, sender: Sender, own_uid: u32) -> Sent {
    let (mut client, mut server) = UnixStream::pair().unwrap();
    client.write_all(format!("{line}\n").as_bytes()).unwrap();
    client.shutdown(Shutdown::Write).unwrap();
    let service = Service {
        queue,
        own_user: OWNER,
        own_uid,
        poll_interval: Duration::from_millis(5),
        longest_wait: Duration::from_millis(100),
        no_list_grace: Duration::from_millis(100),
    };
    let result = service.serve(&mut server, sender);
    drop(server);
    let mut reply = String::new();
    client.read_to_string(&mut reply).unwrap();
    Sent {
        served: result,
        reply,
        sessions: queue.sessions().unwrap(),
    }
}

#[test]
fn should_read_a_report_with_and_without_its_optional_fields() {
    let plain = parsed(&report("idle")).unwrap();
    assert_eq!(plain.waits_for, Vec::<String>::new());
    assert_eq!(plain.seconds_ago, 0);
    assert!(!plain.unless_waiting);
    let mut full = with(report("idle"), "waits_for", json!(["cargo test"]));
    full = with(full, "seconds_ago", json!(86_400));
    full = with(full, "unless_waiting", json!(true));
    let full = parsed(&full).unwrap();
    assert_eq!(full.waits_for, ["cargo test"]);
    assert_eq!(full.seconds_ago, 86_400);
    assert!(full.unless_waiting);
}

#[test]
fn should_refuse_a_report_of_another_form() {
    let mut without_time = report("idle");
    without_time.as_object_mut().unwrap().remove("sent_at");
    for (key, value, reason) in [
        ("version", json!(2), "Report version 2 is not 1"),
        ("kind", json!("prompt"), "Report has another form"),
        ("more", json!(1), "Report has another form"),
        ("state", json!("busy"), "Report has another form"),
        ("host", json!(""), "Report field host is empty"),
        (
            "user",
            json!("Alex"),
            "Report field user is no account name",
        ),
        (
            "user",
            json!("al ex"),
            "Report field user is no account name",
        ),
        (
            "user",
            json!("\u{430}lex"),
            "Report field user is no account name",
        ),
        (
            "session",
            json!("../x"),
            "Report field session is no session ID",
        ),
        (
            "session",
            json!("a".repeat(129)),
            "Report field session is no session ID",
        ),
        ("pid", json!(0), "Report field pid is 0"),
        (
            "unless_waiting",
            json!(true),
            "Report field unless_waiting is only for the state idle",
        ),
        (
            "seconds_ago",
            json!(86_401),
            "Report field seconds_ago is more than 86400",
        ),
        (
            "waits_for",
            json!(vec!["x"; 17]),
            "Report field waits_for has more than 16 items",
        ),
        (
            "waits_for",
            json!(["x".repeat(145)]),
            "Report field waits_for has an item longer than 144 characters",
        ),
    ] {
        assert_eq!(
            parsed(&with(report("waiting"), key, value))
                .unwrap_err()
                .to_string(),
            reason,
            "{key}"
        );
    }
    assert_eq!(
        parsed(&without_time).unwrap_err().to_string(),
        "Report has another form"
    );
}

#[test]
fn should_take_only_a_line_that_names_its_kind_as_a_report() {
    assert!(SessionReport::is_named_in(&report("idle").to_string()));
    assert!(SessionReport::is_named_in("{\"kind\":1}"));
    assert!(!SessionReport::is_named_in("{\"version\":1}"));
    assert!(!SessionReport::is_named_in("[\"kind\"]"));
    assert!(!SessionReport::is_named_in("kind"));
}

#[test]
fn should_keep_the_time_of_an_activity_that_stays_and_start_a_new_one_at_the_report() {
    let waiting = with(report("waiting"), "waits_for", json!(["cargo\ntest"]));
    let first = state_of(&waiting, 1000);
    assert_eq!(first.activity, Activity::Waiting);
    assert_eq!(first.since, 1000);
    assert_eq!(first.waits_for, ["cargo\ntest"]);
    assert_eq!(first.user_text(), "?agent");
    let again = stored(SessionState::after(
        Some(&first),
        &parsed(&waiting).unwrap(),
        1500,
        false,
    ));
    assert_eq!(again.since, 1000);
    let mut idle = with(report("idle"), "seconds_ago", json!(60));
    idle = with(idle, "waits_for", json!(["left over"]));
    let later = stored(SessionState::after(
        Some(&again),
        &parsed(&idle).unwrap(),
        2000,
        true,
    ));
    assert_eq!(later.activity, Activity::Idle);
    assert_eq!(later.since, 1940);
    assert_eq!(later.waits_for, Vec::<String>::new());
    assert_eq!(later.user_text(), "agent");
    assert_eq!(
        SessionState::after(Some(&later), &parsed(&report("ended")).unwrap(), 2100, true),
        Change::Remove
    );
}

#[test]
fn should_keep_the_state_of_a_later_report_when_an_earlier_one_comes_after_it() {
    let later = state_of(&with(report("active"), "sent_at", json!(2000)), 10);
    for state in ["idle", "ended"] {
        assert_eq!(
            SessionState::after(Some(&later), &parsed(&report(state)).unwrap(), 11, false),
            Change::Keep
        );
    }
    let at_the_same_time = with(report("idle"), "sent_at", json!(2000));
    assert!(matches!(
        SessionState::after(Some(&later), &parsed(&at_the_same_time).unwrap(), 11, false),
        Change::Store(_)
    ));
}

#[test]
fn should_replace_a_state_whose_send_time_is_after_the_time_of_the_service() {
    let ahead = state_of(&with(report("active"), "sent_at", json!(16_001)), 20);
    assert_eq!(
        SessionState::after(Some(&ahead), &parsed(&report("ended")).unwrap(), 11, false),
        Change::Remove
    );
    let in_time = state_of(&with(report("active"), "sent_at", json!(16_000)), 20);
    assert_eq!(
        SessionState::after(
            Some(&in_time),
            &parsed(&report("ended")).unwrap(),
            11,
            false
        ),
        Change::Keep
    );
}

#[test]
fn should_keep_a_waiting_state_at_an_idle_signal_and_take_the_signal_for_another_state() {
    let mut signal = with(report("idle"), "unless_waiting", json!(true));
    signal = with(signal, "seconds_ago", json!(60));
    let signal = parsed(&signal).unwrap();
    let waiting = state_of(&report("waiting"), 1000);
    assert_eq!(
        SessionState::after(Some(&waiting), &signal, 1100, false),
        Change::Keep
    );
    let active = state_of(&report("active"), 1000);
    let idle = stored(SessionState::after(Some(&active), &signal, 1100, false));
    assert_eq!((idle.activity, idle.since), (Activity::Idle, 1040));
}

#[test]
fn should_store_the_state_of_a_report_without_a_reply() {
    let (_runtime_folder, queue) = queue();
    let result = sent(&queue, &report("active"), false);
    assert_eq!(result.reply, "");
    let Served::Session(session, Some(state)) = result.served.unwrap() else {
        panic!("served no state");
    };
    assert_eq!(session, SESSION);
    assert_eq!(state.activity, Activity::Active);
    assert!(!state.user_is_verified);
    assert_eq!(
        result.sessions,
        [ListedSession {
            session: SESSION.to_owned(),
            state
        }]
    );
    assert_eq!(queue.waiting().unwrap(), []);
}

#[test]
fn should_replace_the_state_at_the_next_report_and_remove_it_when_the_session_ends() {
    let (_runtime_folder, queue) = queue();
    sent(&queue, &report("active"), false);
    let idle = sent(&queue, &report("idle"), false);
    assert_eq!(idle.sessions.len(), 1);
    assert_eq!(idle.sessions[0].state.activity, Activity::Idle);
    let ended = sent(&queue, &report("ended"), false);
    assert_eq!(
        ended.served.unwrap(),
        Served::Session(SESSION.to_owned(), None)
    );
    assert_eq!(ended.sessions, []);
    let again = sent(&queue, &report("ended"), false);
    assert!(again.served.is_ok());
}

#[test]
fn should_keep_the_stored_state_at_a_report_that_was_sent_before_it() {
    let (_runtime_folder, queue) = queue();
    sent(
        &queue,
        &with(report("active"), "sent_at", json!(2000)),
        false,
    );
    let late = sent(&queue, &report("idle"), false);
    assert_eq!(late.reply, "");
    assert_eq!(late.sessions[0].state.activity, Activity::Active);
    let Served::Session(_, Some(state)) = late.served.unwrap() else {
        panic!("served no state");
    };
    assert_eq!(state.activity, Activity::Active);
}

#[test]
fn should_refuse_a_report_that_names_a_user_other_than_its_sender() {
    let (_runtime_folder, queue) = queue();
    for (user, sender_is_own_user) in [("agent", true), (OWNER, false)] {
        let result = sent(
            &queue,
            &with(report("idle"), "user", json!(user)),
            sender_is_own_user,
        );
        assert_eq!(
            result.reply,
            "{\"error\":\"Line names a user other than its sender\"}\n"
        );
        assert!(result.served.is_err());
        assert_eq!(result.sessions, []);
    }
    let own = sent(&queue, &with(report("idle"), "user", json!(OWNER)), true);
    assert!(own.sessions[0].state.user_is_verified);
}

#[test]
fn should_take_a_report_that_names_a_process_above_its_sender() {
    let (_runtime_folder, queue) = queue();
    let above = rustix::process::getppid().unwrap().as_raw_nonzero().get();
    let result = sent(&queue, &with(report("idle"), "pid", json!(above)), false);
    assert_eq!(result.reply, "");
    assert_eq!(result.sessions.len(), 1);
}

#[test]
fn should_refuse_a_report_whose_process_is_neither_its_sender_nor_above_it() {
    let (_runtime_folder, queue) = queue();
    let mut below = Command::new("sleep").arg("30").spawn().unwrap();
    let results = [below.id(), 1, NO_PROCESS]
        .map(|pid| sent(&queue, &with(report("idle"), "pid", json!(pid)), false));
    below.kill().unwrap();
    below.wait().unwrap();
    for result in results {
        assert_eq!(
            result.reply,
            "{\"error\":\"Report field pid names neither its sender nor a process above it\"}\n"
        );
        assert_eq!(result.sessions, []);
    }
}

#[test]
fn should_refuse_a_report_whose_sender_ended() {
    let (_runtime_folder, queue) = queue();
    let mut ended = Command::new("true").spawn().unwrap();
    ended.wait().unwrap();
    let sender = Sender {
        uid: 1001,
        pid: ended.id(),
    };
    let result = sent_by(&queue, &report("idle"), sender, 1000);
    assert_eq!(
        result.reply,
        "{\"error\":\"Report field pid names neither its sender nor a process above it\"}\n"
    );
}

#[test]
fn should_refuse_a_process_of_the_owner_from_another_sender() {
    let (_runtime_folder, queue) = queue();
    let own_uid = rustix::process::getuid().as_raw();
    let other = Sender {
        uid: own_uid + 1,
        pid: process::id(),
    };
    let result = sent_by(&queue, &report("idle"), other, own_uid);
    assert_eq!(
        result.reply,
        "{\"error\":\"Report field pid names neither its sender nor a process above it\"}\n"
    );
    assert_eq!(result.sessions, []);
}

#[test]
fn should_refuse_a_report_that_was_sent_after_the_time_of_the_service() {
    let (_runtime_folder, queue) = queue();
    let now = u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    )
    .unwrap();
    let result = sent(
        &queue,
        &with(report("active"), "sent_at", json!(now + 60_000)),
        false,
    );
    assert_eq!(
        result.reply,
        "{\"error\":\"Report field sent_at is after the time of the service\"}\n"
    );
    assert_eq!(result.sessions, []);
    let in_time = sent(
        &queue,
        &with(report("active"), "sent_at", json!(now)),
        false,
    );
    assert_eq!(in_time.sessions.len(), 1);
}

#[test]
fn should_refuse_a_report_for_the_session_of_another_user() {
    let (_runtime_folder, queue) = queue();
    sent(&queue, &with(report("active"), "user", json!(OWNER)), true);
    for state in ["idle", "ended"] {
        let result = sent(&queue, &report(state), false);
        assert_eq!(
            result.reply,
            "{\"error\":\"Session belongs to another user\"}\n"
        );
        assert_eq!(result.sessions[0].state.user, OWNER);
        assert_eq!(result.sessions[0].state.activity, Activity::Active);
    }
}

#[test]
fn should_refuse_a_new_session_of_a_user_that_has_the_most_sessions() {
    let (_runtime_folder, queue) = queue();
    let report_of = |session: &str, user: &str| {
        parsed(&with(
            with(report("idle"), "session", json!(session)),
            "user",
            json!(user),
        ))
        .unwrap()
    };
    for number in 0..MAX_SESSIONS_OF_USER {
        queue
            .report_session(&report_of(&format!("s{number}"), "agent"), 1000, false)
            .unwrap();
    }
    assert_eq!(
        queue
            .report_session(&report_of("more", "agent"), 1000, false)
            .unwrap_err()
            .to_string(),
        "User has 64 sessions with a state"
    );
    let mut ended = report_of("more", "agent");
    ended.state = ReportedState::Ended;
    assert_eq!(queue.report_session(&ended, 1000, false).unwrap(), None);
    queue
        .report_session(&report_of("s0", "agent"), 1001, false)
        .unwrap();
    queue
        .report_session(&report_of("other", "other"), 1000, false)
        .unwrap();
}

#[test]
fn should_reply_the_reason_for_a_refused_report() {
    let (_runtime_folder, queue) = queue();
    let result = sent(&queue, &with(report("idle"), "pid", json!(0)), false);
    assert_eq!(result.reply, "{\"error\":\"Report field pid is 0\"}\n");
    assert_eq!(result.sessions, []);
}

#[test]
fn should_list_the_sessions_by_their_ids_and_remove_what_is_no_state_of_a_running_process() {
    let (runtime_folder, queue) = queue();
    for (session, pid) in [
        ("b", process::id()),
        ("a", process::id()),
        ("c", NO_PROCESS),
    ] {
        let line = with(
            with(report("idle"), "session", json!(session)),
            "pid",
            json!(pid),
        );
        queue
            .report_session(&parsed(&line).unwrap(), 1000, false)
            .unwrap();
    }
    let sessions = runtime_folder.path().join("approval-queue/sessions");
    fs::write(sessions.join("d.json"), "[").unwrap();
    fs::write(sessions.join(".new.1.tmp"), "").unwrap();
    let state = fs::read(sessions.join("a.json")).unwrap();
    fs::write(sessions.join("other"), &state).unwrap();
    fs::write(sessions.join(".old.1.tmp"), &state).unwrap();
    fs::File::options()
        .write(true)
        .open(sessions.join(".old.1.tmp"))
        .unwrap()
        .set_modified(SystemTime::now() - Duration::from_mins(2))
        .unwrap();
    let listed: Vec<String> = queue
        .sessions()
        .unwrap()
        .into_iter()
        .map(|listed| listed.session)
        .collect();
    assert_eq!(listed, ["a", "b"]);
    let mut names: Vec<_> = fs::read_dir(&sessions)
        .unwrap()
        .map(|item| item.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    assert_eq!(names, [".new.1.tmp", "a.json", "b.json"]);
}

#[test]
fn should_keep_a_state_that_a_report_stored_after_the_read_of_a_removal() {
    let (_runtime_folder, queue) = queue();
    let gone = with(report("idle"), "pid", json!(NO_PROCESS));
    let read = queue
        .report_session(&parsed(&gone).unwrap(), 1000, false)
        .unwrap();
    let stored = queue
        .report_session(&parsed(&report("active")).unwrap(), 1001, false)
        .unwrap();
    queue.forget_session(SESSION, read.as_ref());
    assert_eq!(queue.sessions().unwrap()[0].state, stored.clone().unwrap());
    queue.forget_session(SESSION, stored.as_ref());
    assert_eq!(queue.sessions().unwrap(), []);
}

#[test]
fn should_name_the_session_its_user_and_its_state_in_the_log() {
    let state = state_of(&report("waiting"), 1000);
    assert_eq!(
        log_text::reported("0a\n1b", Some(&state)),
        "Session 0a\\n1b of ?agent: waiting"
    );
    assert_eq!(log_text::reported("0a1b", None), "Session 0a1b: ended");
}
