use std::{
    ffi::OsString,
    io::{ErrorKind, Read, Write},
    net::Shutdown,
    os::unix::net::UnixStream,
    path::Path,
};

use agent_open::{config::Config, connection};

const PULL_REQUEST: &str = "https://github.com/AlexAegis/pont/pull/1";

fn config() -> Config {
    let text = r#"
url_pages = [{ host = "github.com", paths = ["/*/*/pull/*/**"] }]
editor_roots = []
browser = ["xdg-open"]
editor = ["code"]
"#;
    Config::parse(text, Path::new("/nonexistent")).unwrap()
}

struct Exchange {
    reply: String,
    read_error: Option<ErrorKind>,
    launched: Option<Vec<OsString>>,
}

fn exchange(request: &[u8], launch_result: anyhow::Result<()>) -> Exchange {
    let (mut client, mut server) = UnixStream::pair().unwrap();
    client.write_all(request).unwrap();
    client.shutdown(Shutdown::Write).unwrap();
    let mut launched = None;
    let config = config();
    let _ = connection::serve(&mut server, &config, |command_line| {
        launched = Some(command_line.to_vec());
        launch_result
    });
    drop(server);
    let mut reply = Vec::new();
    let mut buffer = [0; 256];
    let read_error = loop {
        match client.read(&mut buffer) {
            Ok(0) => break None,
            Ok(count) => reply.extend_from_slice(&buffer[..count]),
            Err(error) => break Some(error.kind()),
        }
    };
    Exchange {
        reply: String::from_utf8(reply).unwrap(),
        read_error,
        launched,
    }
}

#[test]
fn should_launch_an_allowed_request_and_reply_ok() {
    let result = exchange(format!("url {PULL_REQUEST}\n").as_bytes(), Ok(()));
    assert_eq!(result.reply, "ok\n");
    assert_eq!(result.launched.unwrap().last().unwrap(), PULL_REQUEST);
}

#[test]
fn should_reply_the_reason_of_a_refusal_without_launching() {
    let result = exchange(b"url https://github.com/settings/tokens\n", Ok(()));
    assert_eq!(result.reply, "error: URL page not allowed\n");
    assert!(result.launched.is_none());
}

#[test]
fn should_reply_the_same_refusal_for_every_refused_path() {
    for path in [
        "/home/alex/.ssh/id_ed25519",
        "/home/alex/.ssh/missing",
        "/etc",
    ] {
        let result = exchange(format!("edit {path}\n").as_bytes(), Ok(()));
        assert_eq!(result.reply, "error: Path not allowed\n", "{path}");
        assert!(result.launched.is_none());
    }
}

#[test]
fn should_reply_an_error_when_the_launch_fails() {
    let result = exchange(
        format!("url {PULL_REQUEST}\n").as_bytes(),
        Err(anyhow::anyhow!("systemd-run failed")),
    );
    assert_eq!(result.reply, "error: systemd-run failed\n");
}

#[test]
fn should_refuse_requests_without_a_newline() {
    for (request, reason) in [
        (b"".as_slice(), "error: Request empty\n"),
        (
            PULL_REQUEST.as_bytes(),
            "error: Request ends without a newline\n",
        ),
    ] {
        let result = exchange(request, Ok(()));
        assert_eq!(result.reply, reason);
        assert!(result.launched.is_none());
    }
}

#[test]
fn should_reply_to_a_request_longer_than_4096_bytes_and_reset() {
    let request = format!("url {PULL_REQUEST}/{}\n", "a".repeat(5000));
    let result = exchange(request.as_bytes(), Ok(()));
    assert_eq!(result.reply, "error: Request longer than 4096 bytes\n");
    assert_eq!(result.read_error, Some(ErrorKind::ConnectionReset));
    assert!(result.launched.is_none());
}

#[test]
fn should_refuse_a_request_that_is_not_utf8() {
    let result = exchange(b"url https://github.com/\xff\n", Ok(()));
    assert_eq!(
        result.reply,
        "error: Request not UTF-8: invalid utf-8 sequence of 1 bytes from index 23\n"
    );
    assert!(result.launched.is_none());
}
