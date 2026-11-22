use approval_queue::{entry::Entry, request::Request};
use serde_json::{Value, json};

fn request() -> Value {
    json!({
        "version": 1,
        "host": "claude-code",
        "user": "agent",
        "session": "0a1b",
        "folder": "/home/agent/git/pont",
        "mode": "auto",
        "tool": "Bash",
        "input": { "command": "git push origin 1a2b:master" },
        "reason": "Push to master",
        "list_only": false,
    })
}

fn changed(field: &str, value: Value) -> String {
    let mut request = request();
    request[field] = value;
    request.to_string()
}

fn refusal(line: &str) -> String {
    Request::parse(line).unwrap_err().to_string()
}

#[test]
fn should_parse_a_request_with_every_field() {
    let parsed = Request::parse(&request().to_string()).unwrap();
    assert_eq!(parsed.host, "claude-code");
    assert_eq!(parsed.user, "agent");
    assert_eq!(parsed.session, "0a1b");
    assert_eq!(parsed.folder, "/home/agent/git/pont");
    assert_eq!(parsed.mode, "auto");
    assert_eq!(parsed.tool, "Bash");
    assert_eq!(
        parsed.input.get("command"),
        Some(&json!("git push origin 1a2b:master"))
    );
    assert_eq!(parsed.reason.as_deref(), Some("Push to master"));
    assert!(!parsed.list_only);
    assert!(
        Request::parse(&changed("list_only", json!(true)))
            .unwrap()
            .list_only
    );
}

#[test]
fn should_parse_a_request_without_a_reason() {
    let parsed = Request::parse(&changed("reason", Value::Null)).unwrap();
    assert_eq!(parsed.reason, None);
}

#[test]
fn should_refuse_a_request_of_another_version() {
    assert_eq!(
        refusal(&changed("version", json!(2))),
        "Request version 2 is not 1"
    );
}

#[test]
fn should_refuse_an_empty_field() {
    for field in ["host", "user", "session", "folder", "mode", "tool"] {
        assert_eq!(
            refusal(&changed(field, json!(""))),
            format!("Request field {field} is empty")
        );
    }
}

#[test]
fn should_refuse_a_field_above_its_limit() {
    for (field, limit) in [
        ("host", 32),
        ("user", 64),
        ("session", 128),
        ("folder", 4096),
        ("mode", 32),
        ("tool", 256),
    ] {
        assert!(Request::parse(&changed(field, json!("a".repeat(limit)))).is_ok());
        assert_eq!(
            refusal(&changed(field, json!("a".repeat(limit + 1)))),
            format!("Request field {field} is longer than {limit} bytes")
        );
    }
}

#[test]
fn should_refuse_a_request_of_another_form() {
    let mut without_tool = request();
    without_tool.as_object_mut().unwrap().remove("tool");
    let mut without_reason = request();
    without_reason.as_object_mut().unwrap().remove("reason");
    let mut without_list_only = request();
    without_list_only
        .as_object_mut()
        .unwrap()
        .remove("list_only");
    for line in [
        "not json".to_owned(),
        "[]".to_owned(),
        without_tool.to_string(),
        without_reason.to_string(),
        without_list_only.to_string(),
        changed("list_only", json!("true")),
        changed("list_only", Value::Null),
        changed("reason", json!(7)),
        changed("input", json!("git push")),
        changed("input", json!(["git", "push"])),
        changed("more", json!(1)),
    ] {
        assert_eq!(refusal(&line), "Request has another form", "{line}");
    }
}

#[test]
fn should_take_a_user_in_the_form_of_an_account_name() {
    for user in ["alex", "agent", "_service", "agent-2", "a_b1"] {
        assert_eq!(
            Request::parse(&changed("user", json!(user))).unwrap().user,
            user
        );
    }
}

#[test]
fn should_refuse_a_user_that_only_looks_like_an_account_name() {
    for user in [
        "alex ",
        " alex",
        "al ex",
        "Alex",
        "\u{430}lex",
        "alex\u{200b}",
        "alex\u{3164}",
        "alex\n",
        "1alex",
        "-alex",
        "alex.b",
    ] {
        assert_eq!(
            refusal(&changed("user", json!(user))),
            "Request field user is no account name",
            "{user:?}"
        );
    }
}

#[test]
fn should_parse_a_request_with_a_title_and_one_without() {
    assert_eq!(Request::parse(&request().to_string()).unwrap().title, None);
    let title = "é".repeat(144);
    let parsed = Request::parse(&changed("title", json!(title))).unwrap();
    assert_eq!(parsed.title, Some(title));
}

#[test]
fn should_refuse_a_title_that_is_empty_too_long_or_has_a_control_character() {
    assert_eq!(
        refusal(&changed("title", json!(""))),
        "Request field title is empty"
    );
    assert_eq!(
        refusal(&changed("title", json!("a".repeat(145)))),
        "Request field title is longer than 144 characters"
    );
    for title in ["Push\nit", "Push\u{1b}[2J", "Push\tit", "Push\u{9b}2J"] {
        assert_eq!(
            refusal(&changed("title", json!(title))),
            "Request field title has a control character"
        );
    }
    assert!(Request::parse(&changed("title", json!(7))).is_err());
}

#[test]
fn should_keep_the_title_of_the_request_in_its_entry() {
    let request = Request::parse(&changed("title", json!("Push the fix"))).unwrap();
    let entry = Entry::of(request, 1_795_000_000, true);
    assert_eq!(entry.title.as_deref(), Some("Push the fix"));
    let stored = serde_json::to_string(&entry).unwrap();
    assert_eq!(serde_json::from_str::<Entry>(&stored).unwrap(), entry);
    let without_title = stored.replace(r#""title":"Push the fix","#, "");
    assert_eq!(
        serde_json::from_str::<Entry>(&without_title).unwrap().title,
        None
    );
}
