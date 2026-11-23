use approval_queue::{answer::Answer, entry::Entry, log_text, outcome::Outcome, request::Request};
use serde_json::{Map, json};

fn entry(session: &str) -> Entry {
    Entry {
        received: 1_795_000_000,
        user: "agent".to_owned(),
        user_is_verified: true,
        host: "claude-code".to_owned(),
        session: session.to_owned(),
        folder: "/home/agent".to_owned(),
        mode: "auto".to_owned(),
        tool: "Bash".to_owned(),
        title: None,
        risk: None,
        input: Map::new(),
        reason: None,
        list_only: false,
    }
}

#[test]
fn should_name_the_user_the_tool_the_session_and_the_outcome() {
    for (outcome, word) in [
        (Outcome::Answered(Answer::ALLOW), "allowed"),
        (Outcome::Answered(Answer::Deny { message: None }), "denied"),
        (Outcome::ClientLeft, "left the queue"),
        (Outcome::Expired, "expired"),
        (
            Outcome::NoList,
            "went back to its terminal, no list is open",
        ),
    ] {
        assert_eq!(
            log_text::served(&entry("0a1b"), &outcome),
            format!("Prompt of agent for Bash in session 0a1b: {word}")
        );
    }
}

#[test]
fn should_mark_the_name_of_a_user_that_the_service_did_not_verify() {
    let stated = Entry {
        user_is_verified: false,
        ..entry("0a1b")
    };
    assert_eq!(
        log_text::served(&stated, &Outcome::ClientLeft),
        "Prompt of ?agent for Bash in session 0a1b: left the queue"
    );
}

#[test]
fn should_keep_the_text_of_a_request_on_the_line_of_its_prompt() {
    let forged = entry("1: denied\nPrompt of alex for Bash in session 2");
    assert_eq!(
        log_text::served(&forged, &Outcome::Answered(Answer::ALLOW)),
        "Prompt of agent for Bash in session 1: denied\\nPrompt of alex for Bash in session 2: allowed"
    );
}

#[test]
fn should_keep_the_reasons_of_a_refusal_on_one_line() {
    let request = json!({ "x\nPrompt of alex for Bash in session 2: allowed": 1 });
    let error = Request::parse(&request.to_string()).unwrap_err();
    let line = log_text::failed(&error.context("Request refused"));
    assert!(
        line.starts_with(
            "Request refused: Request has another form: unknown field `x\\nPrompt of alex for Bash in session 2: allowed`"
        ),
        "{line}"
    );
    assert!(!line.contains('\n'));
}
