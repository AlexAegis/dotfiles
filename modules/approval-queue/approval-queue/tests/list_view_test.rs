use std::time::{Duration, Instant, UNIX_EPOCH};

use approval_queue::{
    entry::Entry, key::Key, list_state::ListState, list_view, listed_entry::ListedEntry,
};
use ratatui::{Terminal, backend::TestBackend};
use serde_json::{Value, json};

const RECEIVED: u64 = 1_795_000_000;

fn listed(id: &str, user: &str, tool: &str, input: &Value, reason: Option<&str>) -> ListedEntry {
    ListedEntry {
        id: id.to_owned(),
        entry: Entry {
            received: RECEIVED,
            user: user.to_owned(),
            user_is_verified: true,
            host: "claude-code".to_owned(),
            session: "0a1b".to_owned(),
            folder: "/home/agent/git/pont".to_owned(),
            mode: "auto".to_owned(),
            tool: tool.to_owned(),
            input: input.as_object().unwrap().clone(),
            reason: reason.map(str::to_owned),
            list_only: false,
        },
    }
}

fn push() -> ListedEntry {
    listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": "git push origin 1a2b:master", "timeout": 5000 }),
        Some("Push to master"),
    )
}

fn screen(state: &ListState, seconds_after: u64) -> Vec<String> {
    drawn(state, seconds_after).0
}

fn drawn(state: &ListState, seconds_after: u64) -> (Vec<String>, u16) {
    let mut terminal = Terminal::new(TestBackend::new(110, 24)).unwrap();
    let now = UNIX_EPOCH + Duration::from_secs(RECEIVED + seconds_after);
    let mut scroll_limit = 0;
    terminal
        .draw(|frame| scroll_limit = list_view::draw(frame, state, now))
        .unwrap();
    let buffer = terminal.backend().buffer();
    let lines = (0..buffer.area.height)
        .map(|y| {
            (0..buffer.area.width)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>()
                .trim_end()
                .to_owned()
        })
        .collect();
    (lines, scroll_limit)
}

fn screen_as_the_list_draws_it(state: &mut ListState) -> Vec<String> {
    let (lines, scroll_limit) = drawn(state, 5);
    state.limit_scroll(scroll_limit);
    lines
}

fn long_command() -> ListedEntry {
    let lines_of_command: Vec<String> = (1..=40).map(|number| format!("line {number}")).collect();
    listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": lines_of_command.join("\n") }),
        None,
    )
}

fn state_with(entries: Vec<ListedEntry>) -> (ListState, Instant) {
    let start = Instant::now();
    let mut state = ListState::default();
    state.refresh(entries, start);
    (state, start + Duration::from_secs(5))
}

fn assert_shows(lines: &[String], part: &str) {
    assert!(
        lines.iter().any(|line| line.contains(part)),
        "{part:?} is not in:\n{}",
        lines.join("\n")
    );
}

#[test]
fn should_show_each_waiting_prompt_in_one_row() {
    let write = listed(
        "2-1",
        "alex",
        "Write",
        &json!({ "file_path": "/home/alex/notes.md", "content": "text" }),
        None,
    );
    let (state, _) = state_with(vec![push(), write]);
    let lines = screen(&state, 125);
    assert_shows(&lines, " approval-queue: 2 waiting ");
    assert_shows(&lines, "waits   user         session  tool         folder");
    assert_shows(
        &lines,
        "2m      agent        0a1b     Bash         /home/agent/git/pont",
    );
    assert_shows(&lines, "git push origin 1a2b:master");
    assert_shows(
        &lines,
        "2m      alex         0a1b     Write        /home/agent/git/pont",
    );
    assert_shows(&lines, "/home/alex/notes.md");
}

#[test]
fn should_show_the_selected_prompt_in_full() {
    let (state, _) = state_with(vec![push()]);
    let lines = screen(&state, 5);
    for part in [
        "user agent   host claude-code   mode auto   session 0a1b",
        "folder /home/agent/git/pont",
        "reason: Push to master",
        "tool Bash",
        "command: git push origin 1a2b:master",
        "timeout: 5000",
        "a allow   d deny   m deny with a message   j/k select   PgUp/PgDn scroll   q quit",
    ] {
        assert_shows(&lines, part);
    }
}

#[test]
fn should_show_the_age_in_its_largest_whole_unit() {
    let (state, _) = state_with(vec![push()]);
    for (seconds, age) in [
        (0, "0s"),
        (59, "59s"),
        (60, "1m"),
        (3599, "59m"),
        (3600, "1h"),
        (86_399, "23h"),
        (86_400, "1d"),
    ] {
        assert_shows(&screen(&state, seconds), &format!("│{age:<8}agent"));
    }
}

#[test]
fn should_show_a_command_of_many_lines_line_by_line() {
    let script = listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": "echo a\necho b" }),
        None,
    );
    let (state, _) = state_with(vec![script]);
    let lines = screen(&state, 5);
    assert_shows(&lines, "echo a\\necho b");
    let command = lines
        .iter()
        .position(|line| line.contains("│command:"))
        .unwrap();
    assert_eq!(lines[command].trim_end_matches('│').trim_end(), "│command:");
    assert_shows(&lines[command + 1..=command + 1], "│  echo a");
    assert_shows(&lines[command + 2..=command + 2], "│  echo b");
}

#[test]
fn should_show_control_characters_of_a_request_as_escapes() {
    let clear = listed(
        "1-1",
        "agent\u{1b}[31m",
        "Bash",
        &json!({ "command": "printf '\u{1b}[2J'", "\u{202e}key": "\u{7}" }),
        Some("because\u{1b}[0m"),
    );
    let (state, _) = state_with(vec![clear]);
    let lines = screen(&state, 5);
    for part in [
        "user agent\\u{1b}[31m",
        "command: printf '\\u{1b}[2J'",
        "\\u{202e}key: \\u{7}",
        "reason: because\\u{1b}[0m",
    ] {
        assert_shows(&lines, part);
    }
    assert!(lines.iter().all(|line| {
        !line
            .chars()
            .any(|character| character.is_control() || character == '\u{202e}')
    }));
}

#[test]
fn should_show_an_input_without_a_command_or_a_file_as_json() {
    let search = listed(
        "1-1",
        "agent",
        "WebSearch",
        &json!({ "query": "pont" }),
        None,
    );
    let (state, _) = state_with(vec![search]);
    assert_shows(&screen(&state, 5), "{\"query\":\"pont\"}");
}

#[test]
fn should_show_the_answer_in_place_of_the_age() {
    for (key, shown) in [('a', "allowed"), ('d', "denied")] {
        let (mut state, later) = state_with(vec![push()]);
        state.press(Key::Character(key), later);
        assert_shows(&screen(&state, 5), &format!("│{shown:<8}agent"));
    }
}

#[test]
fn should_show_that_no_prompt_waits() {
    let (state, _) = state_with(Vec::new());
    let lines = screen(&state, 5);
    assert_shows(&lines, " approval-queue: 0 waiting ");
    assert_shows(&lines, "No prompt waits");
}

#[test]
fn should_show_the_message_while_the_owner_types_it() {
    let (mut state, later) = state_with(vec![push()]);
    state.press(Key::Character('m'), later);
    for character in "Use a branch".chars() {
        state.press(Key::Character(character), later);
    }
    let lines = screen(&state, 5);
    assert_shows(&lines, "Message of the denial: Use a branch_");
    assert_shows(&lines, "Enter send   Esc cancel");
}

#[test]
fn should_show_the_notice_above_the_keys() {
    let start = Instant::now();
    let mut state = ListState::default();
    state.refresh(vec![push()], start);
    state.press(Key::Character('a'), start);
    let lines = screen(&state, 5);
    assert_eq!(
        lines[22],
        "Too early. Press the key after 1 second without a key."
    );
    assert!(lines[23].starts_with("a allow"));
}

#[test]
fn should_show_the_later_lines_of_the_selected_prompt_after_a_scroll() {
    let (mut state, later) = state_with(vec![long_command()]);
    let top = screen_as_the_list_draws_it(&mut state);
    assert_shows(&top, "│  line 1 ");
    assert!(!top.iter().any(|line| line.contains("line 25")));
    state.press(Key::PageDown, later);
    state.press(Key::PageDown, later);
    let scrolled = screen_as_the_list_draws_it(&mut state);
    assert_shows(&scrolled, "│  line 25");
    assert!(!scrolled.iter().any(|line| line.contains("│  line 1 ")));
}

#[test]
fn should_count_the_lines_above_and_below_the_pane_of_the_selected_prompt() {
    let (mut state, later) = state_with(vec![long_command()]);
    assert_eq!(drawn(&state, 5).1, 28);
    let top = screen_as_the_list_draws_it(&mut state);
    assert_shows(&top, " lines below: 28, PgDn ");
    assert!(!top.iter().any(|line| line.contains("lines above")));
    state.press(Key::PageDown, later);
    state.press(Key::PageDown, later);
    let middle = screen_as_the_list_draws_it(&mut state);
    assert_shows(&middle, " lines above: 20, PgUp ");
    assert_shows(&middle, " lines below: 8, PgDn ");
    state.press(Key::PageDown, later);
    state.press(Key::PageDown, later);
    let end = screen_as_the_list_draws_it(&mut state);
    assert_shows(&end, " lines above: 28, PgUp ");
    assert_shows(&end, "│  line 40");
    assert!(!end.iter().any(|line| line.contains("lines below")));
}

#[test]
fn should_count_the_lines_that_one_long_line_takes_in_the_pane() {
    let wide = listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": format!("{}end", "word ".repeat(600)) }),
        None,
    );
    let (mut state, later) = state_with(vec![wide]);
    let top = screen_as_the_list_draws_it(&mut state);
    assert_shows(&top, " lines below: ");
    assert!(!top.iter().any(|line| line.contains("end")));
    for _ in 0..10 {
        state.press(Key::PageDown, later);
        screen_as_the_list_draws_it(&mut state);
    }
    let end = screen_as_the_list_draws_it(&mut state);
    assert_shows(&end, "word end");
    assert!(!end.iter().any(|line| line.contains("lines below")));
}

#[test]
fn should_show_a_prompt_that_fits_its_pane_without_line_counts() {
    let (mut state, later) = state_with(vec![push()]);
    assert_eq!(drawn(&state, 5).1, 0);
    state.press(Key::PageDown, later);
    let lines = screen_as_the_list_draws_it(&mut state);
    assert_shows(&lines, "user agent");
    assert!(!lines.iter().any(|line| line.contains("lines ")));
}

#[test]
fn should_show_the_last_lines_of_a_prompt_for_a_scroll_past_its_end() {
    let (mut state, later) = state_with(vec![long_command()]);
    state.limit_scroll(100);
    for _ in 0..5 {
        state.press(Key::PageDown, later);
    }
    let lines = screen(&state, 5);
    assert_shows(&lines, " lines above: 28, PgUp ");
    assert_shows(&lines, "│  line 40");
}

#[test]
fn should_mark_the_name_of_a_user_that_the_service_did_not_verify() {
    let mut prompt = push();
    prompt.entry.user_is_verified = false;
    let (state, _) = state_with(vec![prompt]);
    let lines = screen(&state, 5);
    assert_shows(&lines, "5s      ?agent       0a1b     Bash");
    assert_shows(&lines, "user ?agent   host claude-code");
}

#[test]
fn should_name_a_prompt_that_only_the_list_can_answer() {
    let mut prompt = push();
    let line = "only in this list: its terminal shows the prompt when no list is open";
    assert!(
        !screen(&state_with(vec![prompt.clone()]).0, 5)
            .iter()
            .any(|shown| shown.contains(line))
    );
    prompt.entry.list_only = true;
    assert_shows(&screen(&state_with(vec![prompt]).0, 5), line);
}

#[test]
fn should_show_the_notice_below_the_message_while_the_owner_types_it() {
    let (mut state, later) = state_with(vec![push()]);
    state.press(Key::Character('m'), later);
    state.press(Key::Character('x'), later);
    state.refresh(Vec::new(), later);
    let lines = screen(&state, 5);
    assert_eq!(lines[22], "Message of the denial: x_");
    assert_eq!(
        lines[23],
        "The prompt left the queue. Esc closes the message."
    );
}

#[test]
fn should_show_the_start_of_a_long_session_id_in_the_row_and_the_whole_id_below() {
    let mut prompt = push();
    prompt.entry.session = "0a1b2c3d-0000-4000-8000-000000000001".to_owned();
    let (state, _) = state_with(vec![prompt]);
    let lines = screen(&state, 5);
    assert_shows(&lines, "agent        0a1b2c3d Bash");
    assert_shows(&lines, "session 0a1b2c3d-0000-4000-8000-000000000001");
}
