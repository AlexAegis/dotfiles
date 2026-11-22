use std::{
    fs,
    io::{Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use approval_queue::{
    answer::Answer, entry::Entry, listed_entry::ListedEntry, outcome::Outcome,
    queue_folder::QueueFolder, request::MAX_REQUEST_BYTES, service::Service,
};
use serde_json::{Value, json};
use tempfile::TempDir;

const OWNER: &str = "alex";

enum Then {
    Answer(Answer),
    Send(&'static [u8]),
    SendInParts(&'static [u8], &'static [u8]),
    SendWithoutTitleFolder(&'static [u8]),
    Close,
    CloseList,
    Wait,
    ReadReply,
}

struct Case {
    request: Vec<u8>,
    sender_is_own_user: bool,
    with_open_list: bool,
    pause: Duration,
    then: Then,
    look_after: Duration,
    longest_wait: Duration,
    no_list_grace: Duration,
}

impl Case {
    fn of(request: Vec<u8>, then: Then) -> Self {
        Self {
            request,
            sender_is_own_user: false,
            with_open_list: false,
            pause: Duration::ZERO,
            then,
            look_after: Duration::ZERO,
            longest_wait: Duration::from_secs(10),
            no_list_grace: Duration::from_secs(10),
        }
    }
}

struct Exchange {
    listed: Vec<ListedEntry>,
    listed_later: Vec<ListedEntry>,
    reply: String,
    served: Result<(Entry, Outcome)>,
    left: Vec<ListedEntry>,
}

fn request(user: &str) -> Value {
    json!({
        "version": 1,
        "host": "claude-code",
        "user": user,
        "session": "0a1b",
        "folder": "/home/agent/git/pont",
        "mode": "auto",
        "tool": "Bash",
        "input": { "command": "git push origin 1a2b:master" },
        "reason": "Push to master",
        "list_only": false,
    })
}

fn line(user: &str) -> Vec<u8> {
    format!("{}\n", request(user)).into_bytes()
}

fn list_only_line() -> Vec<u8> {
    let mut request = request("agent");
    request["list_only"] = json!(true);
    format!("{request}\n").into_bytes()
}

fn exchange(case: Case) -> Exchange {
    let Case {
        request,
        sender_is_own_user,
        with_open_list,
        pause,
        then,
        look_after,
        longest_wait,
        no_list_grace,
    } = case;
    let runtime_folder = TempDir::new().unwrap();
    let queue = QueueFolder::in_runtime_folder(runtime_folder.path());
    queue.create().unwrap();
    let mut open_list = with_open_list.then(|| queue.open_list().unwrap());
    let (mut client, mut server) = UnixStream::pair().unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(10)))
        .unwrap();
    thread::scope(|scope| {
        let service = Service {
            queue: &queue,
            own_user: OWNER,
            poll_interval: Duration::from_millis(5),
            longest_wait,
            no_list_grace,
        };
        let serving = scope.spawn(move || {
            let result = service.serve(&mut server, sender_is_own_user);
            drop(server);
            result
        });
        client.write_all(&request).unwrap();
        if matches!(then, Then::ReadReply) {
            client.shutdown(Shutdown::Write).unwrap();
        }
        let deadline = Instant::now() + Duration::from_secs(2);
        let mut listed = Vec::new();
        while listed.is_empty() && !serving.is_finished() && Instant::now() < deadline {
            listed = queue.waiting().unwrap();
            thread::sleep(Duration::from_millis(2));
        }
        thread::sleep(pause);
        match then {
            Then::Answer(answer) => queue.answer(&listed[0].id, &answer).unwrap(),
            Then::Send(bytes) => client.write_all(bytes).unwrap(),
            Then::SendWithoutTitleFolder(bytes) => {
                fs::remove_dir(runtime_folder.path().join("approval-queue/titles")).unwrap();
                client.write_all(bytes).unwrap();
            }
            Then::SendInParts(first, second) => {
                client.write_all(first).unwrap();
                thread::sleep(Duration::from_millis(40));
                client.write_all(second).unwrap();
            }
            Then::Close => client.shutdown(Shutdown::Both).unwrap(),
            Then::CloseList => drop(open_list.take()),
            Then::Wait | Then::ReadReply => {}
        }
        thread::sleep(look_after);
        let listed_later = queue.waiting().unwrap();
        let mut reply = String::new();
        let _ = client.read_to_string(&mut reply);
        Exchange {
            listed,
            listed_later,
            reply,
            served: serving.join().unwrap(),
            left: queue.waiting().unwrap(),
        }
    })
}

fn waited(then: Then) -> Exchange {
    exchange(Case::of(line("agent"), then))
}

fn refused(request: &[u8]) -> Exchange {
    exchange(Case::of(request.to_vec(), Then::ReadReply))
}

#[test]
fn should_list_the_request_while_it_waits() {
    let result = waited(Then::Answer(Answer::ALLOW));
    assert_eq!(result.listed.len(), 1);
    let entry = &result.listed[0].entry;
    assert_eq!(entry.user, "agent");
    assert!(!entry.user_is_verified);
    assert!(!entry.list_only);
    assert_eq!(entry.host, "claude-code");
    assert_eq!(entry.session, "0a1b");
    assert_eq!(entry.folder, "/home/agent/git/pont");
    assert_eq!(entry.mode, "auto");
    assert_eq!(entry.tool, "Bash");
    assert_eq!(
        entry.input.get("command"),
        Some(&json!("git push origin 1a2b:master"))
    );
    assert_eq!(entry.reason.as_deref(), Some("Push to master"));
}

#[test]
fn should_reply_allow_when_the_owner_allows() {
    let result = waited(Then::Answer(Answer::ALLOW));
    assert_eq!(result.reply, "{\"behavior\":\"allow\"}\n");
    assert_eq!(result.served.unwrap().1, Outcome::Answered(Answer::ALLOW));
    assert_eq!(result.left, []);
}

#[test]
fn should_reply_a_denial_with_its_message() {
    let denial = Answer::Deny {
        message: Some("Use a branch".to_owned()),
    };
    let result = waited(Then::Answer(denial.clone()));
    assert_eq!(
        result.reply,
        "{\"behavior\":\"deny\",\"message\":\"Use a branch\"}\n"
    );
    assert_eq!(result.served.unwrap().1, Outcome::Answered(denial));
    assert_eq!(result.left, []);
}

#[test]
fn should_reply_a_denial_without_a_message() {
    let result = waited(Then::Answer(Answer::Deny { message: None }));
    assert_eq!(result.reply, "{\"behavior\":\"deny\"}\n");
}

#[test]
fn should_leave_the_queue_when_the_hook_closes() {
    let result = waited(Then::Close);
    assert_eq!(result.listed.len(), 1);
    assert_eq!(result.reply, "");
    assert_eq!(result.served.unwrap().1, Outcome::ClientLeft);
    assert_eq!(result.left, []);
}

#[test]
fn should_end_without_a_reply_after_the_longest_wait() {
    let result = exchange(Case {
        longest_wait: Duration::from_millis(40),
        ..Case::of(line("agent"), Then::Wait)
    });
    assert_eq!(result.listed.len(), 1);
    assert_eq!(result.reply, "");
    assert_eq!(result.served.unwrap().1, Outcome::Expired);
    assert_eq!(result.left, []);
}

#[test]
fn should_send_a_list_only_prompt_back_when_no_list_is_open() {
    let result = exchange(Case {
        no_list_grace: Duration::from_millis(40),
        ..Case::of(list_only_line(), Then::Wait)
    });
    assert_eq!(result.listed.len(), 1);
    assert!(result.listed[0].entry.list_only);
    assert_eq!(result.reply, "{\"error\":\"No list is open\"}\n");
    assert_eq!(result.served.unwrap().1, Outcome::NoList);
    assert_eq!(result.left, []);
}

#[test]
fn should_keep_a_list_only_prompt_while_a_list_is_open() {
    let result = exchange(Case {
        with_open_list: true,
        pause: Duration::from_millis(150),
        no_list_grace: Duration::from_millis(40),
        ..Case::of(list_only_line(), Then::Answer(Answer::ALLOW))
    });
    assert_eq!(result.reply, "{\"behavior\":\"allow\"}\n");
}

#[test]
fn should_send_a_list_only_prompt_back_after_its_list_closed() {
    let result = exchange(Case {
        with_open_list: true,
        pause: Duration::from_millis(850),
        look_after: Duration::from_millis(100),
        no_list_grace: Duration::from_millis(800),
        ..Case::of(list_only_line(), Then::CloseList)
    });
    assert_eq!(result.listed_later.len(), 1);
    assert_eq!(result.reply, "{\"error\":\"No list is open\"}\n");
    assert_eq!(result.served.unwrap().1, Outcome::NoList);
}

#[test]
fn should_keep_another_prompt_without_an_open_list() {
    let result = exchange(Case {
        pause: Duration::from_millis(150),
        no_list_grace: Duration::from_millis(40),
        ..Case::of(line("agent"), Then::Answer(Answer::ALLOW))
    });
    assert_eq!(result.reply, "{\"behavior\":\"allow\"}\n");
}

fn sent_later(bytes: &'static [u8]) -> Exchange {
    exchange(Case {
        look_after: Duration::from_millis(100),
        longest_wait: Duration::from_millis(400),
        ..Case::of(line("agent"), Then::Send(bytes))
    })
}

#[test]
fn should_give_the_waiting_prompt_the_title_of_a_later_line() {
    let result = sent_later(b"{\"title\":\"Push one commit to master\"}\n");
    assert_eq!(result.listed[0].entry.title, None);
    assert_eq!(
        result.listed_later[0].entry.title.as_deref(),
        Some("Push one commit to master")
    );
    assert_eq!(result.listed_later[0].id, result.listed[0].id);
    assert_eq!(result.served.unwrap().1, Outcome::Expired);
    assert_eq!(result.left, []);
}

#[test]
fn should_take_the_last_title_and_keep_the_prompt_at_a_line_of_another_form() {
    let result = sent_later(
        b"x\n{\"title\":\"B\"}\n{\"title\":\"\"}\n{\"title\":\"A\",\"more\":1}\n\xff\n{\"tit",
    );
    assert_eq!(result.listed_later[0].entry.title.as_deref(), Some("B"));
    assert_eq!(result.served.unwrap().1, Outcome::Expired);
}

#[test]
fn should_keep_the_prompt_with_its_title_when_the_queue_cannot_store_a_later_title() {
    let result = exchange(Case {
        look_after: Duration::from_millis(100),
        longest_wait: Duration::from_millis(400),
        ..Case::of(
            line("agent"),
            Then::SendWithoutTitleFolder(b"{\"title\":\"Push one\"}\n"),
        )
    });
    assert_eq!(result.listed_later[0].entry.title, None);
    assert_eq!(result.served.unwrap().1, Outcome::Expired);
}

#[test]
fn should_take_a_title_that_comes_in_one_read_with_the_request() {
    let mut request = line("agent");
    request.extend_from_slice(b"{\"title\":\"Push one\"}\n");
    let result = exchange(Case {
        look_after: Duration::from_millis(100),
        longest_wait: Duration::from_millis(400),
        ..Case::of(request, Then::Wait)
    });
    assert_eq!(
        result.listed_later[0].entry.title.as_deref(),
        Some("Push one")
    );
    assert_eq!(result.served.unwrap().1, Outcome::Expired);
}

#[test]
fn should_take_a_title_whose_line_comes_in_two_parts() {
    let result = exchange(Case {
        look_after: Duration::from_millis(100),
        longest_wait: Duration::from_millis(400),
        ..Case::of(
            line("agent"),
            Then::SendInParts(b"{\"title\":\"Pu", b"sh one\"}\n"),
        )
    });
    assert_eq!(
        result.listed_later[0].entry.title.as_deref(),
        Some("Push one")
    );
}

#[test]
fn should_leave_the_queue_when_the_hook_sends_a_whole_line_above_the_limit() {
    static LONG: [u8; 2051] = {
        let mut lines = [b'x'; 2051];
        lines[1] = b'\n';
        lines[2050] = b'\n';
        lines
    };
    let result = sent_later(&LONG);
    assert_eq!(result.reply, "");
    assert_eq!(
        result.served.unwrap_err().to_string(),
        "Hook sent a line longer than 2048 bytes after its request"
    );
}

#[test]
fn should_leave_the_queue_when_the_hook_sends_a_long_line_without_an_end() {
    static LONG: [u8; 2048] = [b'x'; 2048];
    let result = sent_later(&LONG);
    assert_eq!(result.listed.len(), 1);
    assert_eq!(result.reply, "");
    assert_eq!(
        result.served.unwrap_err().to_string(),
        "Hook sent a line longer than 2048 bytes after its request"
    );
    assert_eq!(result.left, []);
}

#[test]
fn should_take_the_request_of_the_owner_from_the_owner() {
    let result = exchange(Case {
        sender_is_own_user: true,
        ..Case::of(line(OWNER), Then::Answer(Answer::ALLOW))
    });
    assert_eq!(result.listed[0].entry.user, OWNER);
    assert!(result.listed[0].entry.user_is_verified);
    assert_eq!(result.reply, "{\"behavior\":\"allow\"}\n");
}

#[test]
fn should_refuse_a_request_that_names_a_user_other_than_its_sender() {
    for (user, sender_is_own_user) in [(OWNER, false), ("agent", true)] {
        let result = exchange(Case {
            sender_is_own_user,
            ..Case::of(line(user), Then::ReadReply)
        });
        assert_eq!(
            result.reply, "{\"error\":\"Request names a user other than its sender\"}\n",
            "{user}"
        );
        assert_eq!(result.listed, [], "{user}");
        assert!(result.served.is_err(), "{user}");
    }
}

#[test]
fn should_refuse_a_user_that_only_looks_like_the_owner() {
    for user in ["alex ", "\u{430}lex", "alex\u{200b}"] {
        let result = refused(&line(user));
        assert_eq!(
            result.reply, "{\"error\":\"Request field user is no account name\"}\n",
            "{user:?}"
        );
        assert_eq!(result.listed, [], "{user:?}");
    }
}

#[test]
fn should_refuse_a_request_of_another_form() {
    for (request, reason) in [
        (b"".to_vec(), "Request empty"),
        (b"{}".to_vec(), "Request ends without a newline"),
        (b"{}\n".to_vec(), "Request has another form"),
        (b"\xff\n".to_vec(), "Request not UTF-8"),
    ] {
        let result = refused(&request);
        assert_eq!(
            result.reply,
            format!("{}\n", json!({ "error": reason })),
            "{reason}"
        );
        assert_eq!(result.listed, [], "{reason}");
        assert!(result.served.is_err(), "{reason}");
    }
}

#[test]
fn should_take_a_request_of_the_longest_size_and_refuse_a_longer_one() {
    let padded = |size: usize| {
        let mut request = request("agent");
        request["input"]["padding"] = json!("");
        let size_without_padding = format!("{request}\n").len();
        request["input"]["padding"] = json!("a".repeat(size - size_without_padding));
        let line = format!("{request}\n").into_bytes();
        assert_eq!(line.len(), size);
        line
    };
    let longest = exchange(Case::of(
        padded(MAX_REQUEST_BYTES),
        Then::Answer(Answer::ALLOW),
    ));
    assert_eq!(longest.reply, "{\"behavior\":\"allow\"}\n");
    let longer = refused(&padded(MAX_REQUEST_BYTES + 1));
    assert_eq!(
        longer.reply,
        "{\"error\":\"Request longer than 65536 bytes\"}\n"
    );
    assert_eq!(longer.listed, []);
}
