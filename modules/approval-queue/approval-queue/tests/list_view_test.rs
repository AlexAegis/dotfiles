use std::time::{Duration, Instant, UNIX_EPOCH};

use approval_queue::{
    entry::Entry,
    key::Key,
    list_state::ListState,
    list_view,
    listed_entry::ListedEntry,
    session_state::{Activity, ListedSession, SessionState},
};
use ratatui::{
    Terminal,
    backend::TestBackend,
    style::{Color, Modifier},
};
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
            title: None,
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
    drawn_at_width(state, seconds_after, 110)
}

fn drawn_at_width(state: &ListState, seconds_after: u64, width: u16) -> (Vec<String>, u16) {
    let mut terminal = Terminal::new(TestBackend::new(width, 24)).unwrap();
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
fn should_show_each_waiting_prompt_as_an_entry_with_its_kind_title_and_details() {
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
    let kinds: Vec<&String> = lines
        .iter()
        .filter(|line| line.starts_with("│PERMISSION") && line.contains(" 2m││"))
        .collect();
    assert_eq!(kinds.len(), 2, "{}", lines.join("\n"));
    assert_shows(&lines, "│Bash git push origin 1a2b:master ");
    assert_shows(&lines, "│agent  Bash  pont  0a1b ");
    assert_shows(&lines, "│Write /home/alex/notes.md ");
    assert_shows(&lines, "│alex  Write  pont  0a1b ");
}

#[test]
fn should_show_the_title_of_the_request_and_name_a_question() {
    let mut question = listed(
        "1-1",
        "agent",
        "AskUserQuestion",
        &json!({ "questions": [] }),
        None,
    );
    question.entry.title = Some("Which library\u{1b} formats dates?".to_owned());
    let (state, _) = state_with(vec![question]);
    let lines = screen(&state, 5);
    assert_shows(&lines, "│QUESTION ");
    assert_shows(&lines, "│Which library\\u{1b} formats dates? ");
    assert_shows(&lines, " QUESTION: AskUserQuestion ");
    assert!(!lines.iter().any(|line| line.contains("PERMISSION")));
}

#[test]
fn should_cut_the_title_of_a_request_without_one_to_one_line_of_144_characters() {
    let long = listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": format!("echo  a\n{}", "b".repeat(300)) }),
        None,
    );
    let (state, _) = state_with(vec![long]);
    let lines = drawn_at_width(&state, 5, 400).0;
    assert_shows(&lines, &format!("││Bash echo a {}... ", "b".repeat(129)));
}

#[test]
fn should_show_the_command_also_when_the_title_equals_it() {
    let mut prompt = listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": "echo hi\nrm -r x", "description": "other" }),
        None,
    );
    prompt.entry.title = Some("echo hi\\nrm -r x".to_owned());
    let (state, _) = state_with(vec![prompt]);
    let lines = drawn_at_width(&state, 5, 80).0;
    assert_shows(&lines, "│command:");
    assert_shows(&lines, "│  rm -r x");
    assert_shows(&lines, "│description: other");
}

#[test]
fn should_cut_a_title_of_characters_of_many_bytes_by_characters() {
    let long = listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": "é".repeat(300) }),
        None,
    );
    let (state, _) = state_with(vec![long]);
    let lines = drawn_at_width(&state, 5, 400).0;
    assert_shows(&lines, &format!("││Bash {}... ", "é".repeat(136)));
}

#[test]
fn should_show_the_control_and_direction_characters_of_each_field_as_escapes() {
    let evil = "x\u{1b}[2J\u{202e}\u{9b}y";
    let mut prompt = listed(
        "1-1",
        evil,
        evil,
        &json!({ evil: evil, "list": [evil], "object": { evil: evil }, "command": evil }),
        Some(evil),
    );
    prompt.entry.host = evil.to_owned();
    prompt.entry.session = evil.to_owned();
    prompt.entry.folder = format!("/home/{evil}");
    prompt.entry.mode = evil.to_owned();
    let shown = "x\\u{1b}[2J\\u{202e}\\u{9b}y";
    let lines = drawn_at_width(&state_with(vec![prompt.clone()]).0, 5, 400).0;
    for part in [
        format!("│{shown}  {shown}  {shown}  x\\u{{1b}}[ "),
        format!(" PERMISSION: {shown} "),
        format!("││user {shown}   host {shown}   mode {shown} "),
        format!("││session {shown} "),
        format!("││folder /home/{shown} "),
        format!("││reason: {shown} "),
        format!("││command: {shown} "),
        format!("││{shown}: {shown} "),
    ] {
        assert_shows(&lines, &part);
    }
    for title in [None, Some(evil.to_owned())] {
        prompt.entry.title = title;
        let (state, _) = state_with(vec![prompt.clone()]);
        for width in [60, 110, 400] {
            let lines = drawn_at_width(&state, 5, width).0;
            assert!(lines.iter().all(|line| {
                !line
                    .chars()
                    .any(|character| character.is_control() || character == '\u{202e}')
            }));
        }
    }
}

#[test]
fn should_show_only_the_selected_entry_reversed() {
    let second = listed("1-2", "agent", "Bash", &json!({ "command": "ls" }), None);
    let (mut state, later) = state_with(vec![push(), second]);
    state.press(Key::Down, later);
    let mut terminal = Terminal::new(TestBackend::new(110, 24)).unwrap();
    terminal
        .draw(|frame| {
            list_view::draw(frame, &state, UNIX_EPOCH + Duration::from_secs(RECEIVED));
        })
        .unwrap();
    let buffer = terminal.backend().buffer();
    let is_reversed = |row| buffer[(1, row)].modifier.contains(Modifier::REVERSED);
    let reversed: Vec<u16> = (0..24).filter(|row| is_reversed(*row)).collect();
    assert_eq!(reversed, [5, 6, 7]);
}

#[test]
fn should_draw_at_each_small_size_without_a_panic() {
    let entries: Vec<ListedEntry> = (1..=9)
        .map(|number| {
            listed(
                &format!("1-{number}"),
                "agent",
                "Bash",
                &json!({ "command": "ls" }),
                None,
            )
        })
        .collect();
    for (count, has_message) in [(0, false), (1, false), (9, false), (9, true)] {
        let (mut state, later) = state_with(entries[..count].to_vec());
        for _ in 0..count {
            state.press(Key::Down, later);
        }
        if has_message {
            state.press(Key::Character('m'), later + Duration::from_secs(2));
            state.paste(&"long ".repeat(100), later + Duration::from_secs(2));
        }
        for width in 1..=110 {
            for height in 1..=12 {
                let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
                terminal
                    .draw(|frame| {
                        list_view::draw(frame, &state, UNIX_EPOCH);
                    })
                    .unwrap();
            }
        }
    }
}

#[test]
fn should_not_draw_the_wait_over_the_kind_in_a_narrow_list() {
    let (state, _) = state_with(vec![push()]);
    let narrow = drawn_at_width(&state, 5, 14).0;
    assert_shows(&narrow, "│PERMISSION  │");
    let wide = drawn_at_width(&state, 5, 15).0;
    assert_shows(&wide, "│PERMISSION 5s│");
}

#[test]
fn should_not_repeat_the_input_text_that_is_the_title() {
    let mut prompt = listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": "ls", "description": "List the files" }),
        None,
    );
    prompt.entry.title = Some("List the files".to_owned());
    let (state, _) = state_with(vec![prompt]);
    let lines = screen(&state, 5);
    assert_shows(&lines, "││List the files ");
    assert_shows(&lines, "command: ls");
    assert!(!lines.iter().any(|line| line.contains("description")));
}

#[test]
fn should_keep_the_selected_entry_in_view_and_count_the_entries_out_of_view() {
    let entries: Vec<ListedEntry> = (1..=9)
        .map(|number| {
            listed(
                &format!("1-{number}"),
                "agent",
                "Bash",
                &json!({ "command": format!("echo {number}") }),
                None,
            )
        })
        .collect();
    let (mut state, later) = state_with(entries);
    let top = screen(&state, 5);
    assert_shows(&top, "│Bash echo 1 ");
    assert_shows(&top, "│Bash echo 5 ");
    assert_shows(&top, " below: 4 ");
    assert!(!top.iter().any(|line| line.contains("above: ")));
    for _ in 0..8 {
        state.press(Key::Down, later);
    }
    let end = screen(&state, 5);
    assert!(end.iter().any(|line| line.starts_with("│Bash echo 5 ")));
    assert!(end.iter().any(|line| line.starts_with("│Bash echo 9 ")));
    assert_shows(&end, " above: 4 ");
    assert!(!end.iter().any(|line| line.contains("│Bash echo 4 ")));
    assert!(!end.iter().any(|line| line.contains("below: ")));
}

#[test]
fn should_put_the_list_above_the_selected_prompt_in_a_narrow_terminal() {
    let (state, _) = state_with(vec![push()]);
    let lines = drawn_at_width(&state, 5, 99).0;
    let list = lines
        .iter()
        .position(|line| line.starts_with("│PERMISSION"))
        .unwrap();
    let pane = lines
        .iter()
        .position(|line| line.starts_with("│user agent"))
        .unwrap();
    assert!(list < pane);
    let wide = drawn_at_width(&state, 5, 100).0;
    assert_shows(&wide, "││user agent");
}

#[test]
fn should_show_the_selected_prompt_in_full() {
    let (state, _) = state_with(vec![push()]);
    let lines = screen(&state, 5);
    for part in [
        " PERMISSION: Bash ",
        "user agent   host claude-code   mode auto",
        "session 0a1b",
        "folder /home/agent/git/pont",
        "reason: Push to master",
        "││Bash git push origin 1a2b:master ",
        "command: git push origin 1a2b:master",
        "timeout: 5000",
        " a  Approve    d  Decline    m  Message",
        "j/k select   PgUp/PgDn scroll   s/S mute pings 10 min/1 hour   u unmute   q quit",
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
        assert_shows(&screen(&state, seconds), &format!("  {age}││"));
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
    let lines = drawn_at_width(&state, 5, 80).0;
    assert_shows(&lines, "│Bash echo a echo b ");
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
    for (key, shown) in [('a', "approved"), ('d', "declined")] {
        let (mut state, later) = state_with(vec![push()]);
        state.press(Key::Character(key), later);
        assert_shows(&screen(&state, 5), &format!("  {shown}││"));
    }
}

#[test]
fn should_show_that_no_prompt_waits() {
    let (state, _) = state_with(Vec::new());
    let lines = screen(&state, 5);
    assert_shows(&lines, " approval-queue: 0 waiting ");
    assert_shows(&lines, "│No prompt waits");
    assert_shows(&lines, "││No prompt waits");
}

#[test]
fn should_show_the_message_while_the_owner_types_it() {
    let (mut state, later) = state_with(vec![push()]);
    state.press(Key::Character('m'), later);
    for character in "Use a branch".chars() {
        state.press(Key::Character(character), later);
    }
    let lines = screen(&state, 5);
    assert_shows(&lines, "│Message: Use a branch_");
    assert_shows(
        &lines,
        "Enter or Esc keeps the message for the answer to this prompt",
    );
    state.press(Key::Enter, later);
    let lines = screen(&state, 5);
    assert_shows(&lines, "│Message: Use a branch ");
    assert_shows(&lines, "j/k select");
}

fn text_of_the_pane(lines: &[String]) -> String {
    lines
        .iter()
        .filter_map(|line| line.split_once('│'))
        .filter_map(|(_, rest)| rest.rsplit_once('│'))
        .map(|(inner, _)| inner.rsplit_once('│').map_or(inner, |(_, pane)| pane))
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn should_show_each_character_of_a_long_kept_message() {
    let (mut state, later) = state_with(vec![push()]);
    state.press(Key::Character('m'), later);
    let long: String = "START"
        .chars()
        .chain((0..490).map(|index| if index % 7 == 0 { ' ' } else { 'x' }))
        .chain("END".chars())
        .collect();
    state.paste(&long, later);
    state.press(Key::Enter, later);
    for width in [80, 120] {
        let lines = drawn_at_width(&state, 5, width).0;
        let pane: String = text_of_the_pane(&lines).split_whitespace().collect();
        let message: String = long.split_whitespace().collect();
        assert!(pane.contains(&format!("Message:{message}")), "{width}");
        assert_shows(&lines, " a  Approve");
    }
}

#[test]
fn should_count_the_lines_of_a_message_that_a_small_pane_does_not_show() {
    let (mut state, later) = state_with(vec![push()]);
    state.press(Key::Character('m'), later);
    state.paste(&format!("START {}END", "word ".repeat(98)), later);
    let typed = drawn_at_size(&state, 80, 10);
    assert_shows(&typed, "END_");
    assert_shows(&typed, " message: 6 more lines, in a larger window ");
    state.press(Key::Enter, later);
    let kept = drawn_at_size(&state, 80, 10);
    assert_shows(&kept, "│Message: START");
    assert_shows(&kept, " message: 6 more lines, in a larger window ");
    assert_shows(&kept, " a  Approve");
    let whole = drawn_at_size(&state, 80, 40);
    assert!(!whole.iter().any(|line| line.contains("more lines, in a")));
}

fn drawn_at_size(state: &ListState, width: u16, height: u16) -> Vec<String> {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal
        .draw(|frame| {
            list_view::draw(frame, state, UNIX_EPOCH);
        })
        .unwrap();
    let buffer = terminal.backend().buffer();
    (0..buffer.area.height)
        .map(|y| {
            (0..buffer.area.width)
                .map(|x| buffer[(x, y)].symbol())
                .collect::<String>()
                .trim_end()
                .to_owned()
        })
        .collect()
}

#[test]
fn should_show_each_wide_character_of_a_message() {
    let (mut state, later) = state_with(vec![push()]);
    state.press(Key::Character('m'), later);
    state.paste(&format!("{}END", "中".repeat(100)), later);
    state.press(Key::Enter, later);
    let lines = drawn_at_width(&state, 5, 80).0;
    let shown = lines.join("").matches('中').count();
    assert_eq!(shown, 100);
    assert_shows(&lines, "END");
}

#[test]
fn should_show_the_kept_message_only_with_its_prompt() {
    let other = listed("2-1", "agent", "Bash", &json!({ "command": "ls" }), None);
    let (mut state, later) = state_with(vec![push(), other]);
    state.press(Key::Character('m'), later);
    state.press(Key::Character('x'), later);
    state.press(Key::Enter, later);
    assert_shows(&screen(&state, 5), "Message: x");
    state.press(Key::Down, later);
    let lines = screen(&state, 5);
    assert!(!lines.iter().any(|line| line.contains("Message:")));
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
    assert!(lines[23].starts_with("j/k select"));
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
    assert_eq!(drawn(&state, 5).1, 30);
    let top = screen_as_the_list_draws_it(&mut state);
    assert_shows(&top, " lines below: 30, PgDn ");
    assert!(!top.iter().any(|line| line.contains("lines above")));
    state.press(Key::PageDown, later);
    state.press(Key::PageDown, later);
    let middle = screen_as_the_list_draws_it(&mut state);
    assert_shows(&middle, " lines above: 20, PgUp ");
    assert_shows(&middle, " lines below: 10, PgDn ");
    state.press(Key::PageDown, later);
    state.press(Key::PageDown, later);
    let end = screen_as_the_list_draws_it(&mut state);
    assert_shows(&end, " lines above: 30, PgUp ");
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
    assert_shows(&lines, " lines above: 30, PgUp ");
    assert_shows(&lines, "│  line 40");
}

#[test]
fn should_mark_the_name_of_a_user_that_the_service_did_not_verify() {
    let mut prompt = push();
    prompt.entry.user_is_verified = false;
    let (state, _) = state_with(vec![prompt]);
    let lines = screen(&state, 5);
    assert_shows(&lines, "│?agent  Bash  pont  0a1b ");
    assert_shows(&lines, "user ?agent   host claude-code");
}

#[test]
fn should_name_a_prompt_that_only_the_list_can_answer() {
    let mut prompt = push();
    let line = "only in this list: its terminal shows the prompt when no list is open";
    assert!(
        !drawn_at_width(&state_with(vec![prompt.clone()]).0, 5, 80)
            .0
            .iter()
            .any(|shown| shown.contains(line))
    );
    prompt.entry.list_only = true;
    assert_shows(&drawn_at_width(&state_with(vec![prompt]).0, 5, 80).0, line);
}

#[test]
fn should_show_the_message_of_a_prompt_that_left_in_the_footer_and_in_no_pane() {
    let other = listed("2-1", "agent", "Bash", &json!({ "command": "ls" }), None);
    for left in [Vec::new(), vec![other]] {
        let (mut state, later) = state_with(vec![push()]);
        state.press(Key::Character('m'), later);
        state.press(Key::Character('x'), later);
        state.refresh(left, later);
        let lines = screen(&state, 5);
        assert_eq!(lines[22], "Message: x_");
        assert_eq!(
            lines[23],
            "The prompt left the queue. Esc closes the message."
        );
        assert!(!lines[..22].iter().any(|line| line.contains("Message:")));
    }
}

#[test]
fn should_show_the_start_of_a_long_session_id_in_the_row_and_the_whole_id_below() {
    let mut prompt = push();
    prompt.entry.session = "0a1b2c3d-0000-4000-8000-000000000001".to_owned();
    let (state, _) = state_with(vec![prompt]);
    let lines = screen(&state, 5);
    assert_shows(&lines, "│agent  Bash  pont  0a1b2c3d ");
    assert_shows(&lines, "session 0a1b2c3d-0000-4000-8000-000000000001");
}

#[test]
fn should_show_below_the_list_how_long_the_pings_are_muted() {
    let (mut state, _) = state_with(vec![push()]);
    state.show_mute(Some(UNIX_EPOCH + Duration::from_secs(RECEIVED + 5 + 541)));
    assert_shows(&screen(&state, 5), "└ pings muted for 10 min ");
    assert_shows(&screen(&state, 5 + 481), "└ pings muted for 1 min ");
    assert!(
        !screen(&state, 5 + 542)
            .iter()
            .any(|line| line.contains("muted"))
    );
}

#[test]
fn should_show_each_key_in_a_terminal_of_80_columns() {
    let (state, _) = state_with(vec![push()]);
    let lines = drawn_at_width(&state, 5, 80).0;
    for key in [
        " a  Approve",
        " d  Decline",
        " m  Message",
        "j/k select",
        "PgUp/PgDn scroll",
        "s/S mute pings 10 min/1 hour",
        "u unmute",
        "q quit",
    ] {
        assert_shows(&lines, key);
    }
}

fn color_at(state: &ListState, text: &str) -> Color {
    let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
    terminal
        .draw(|frame| {
            list_view::draw(frame, state, UNIX_EPOCH + Duration::from_secs(RECEIVED));
        })
        .unwrap();
    let buffer = terminal.backend().buffer();
    for y in (0..buffer.area.height).rev() {
        let line: String = (0..buffer.area.width)
            .map(|x| buffer[(x, y)].symbol())
            .collect();
        if let Some(byte) = line.find(text) {
            let x = u16::try_from(line[..byte].chars().count()).unwrap();
            return buffer[(x, y)].fg;
        }
    }
    panic!("{text:?} is not on the screen");
}

#[test]
fn should_color_the_command_of_a_bash_call_as_a_shell_script() {
    let one_line = listed(
        "1-2",
        "agent",
        "Bash",
        &json!({ "command": "echo 'a'" }),
        None,
    );
    let (state, _) = state_with(vec![one_line]);
    assert_shows(&drawn_at_width(&state, 5, 80).0, "│command: echo 'a'");
    assert_eq!(color_at(&state, "'a'"), Color::Green);
    let script = listed(
        "1-1",
        "agent",
        "Bash",
        &json!({ "command": "if true; then\n  echo 'yes' # note\nfi", "note": "if true" }),
        None,
    );
    let (state, _) = state_with(vec![script]);
    assert_eq!(color_at(&state, "if true; then"), Color::Blue);
    assert_eq!(color_at(&state, "yes"), Color::Green);
    assert_eq!(color_at(&state, "note: if true"), Color::Reset);
}

#[test]
fn should_color_the_content_of_a_write_call_by_the_language_of_its_file() {
    let input = |path: &str| json!({ "file_path": path, "content": "fn main() {}\n// done" });
    let rust = listed("1-1", "agent", "Write", &input("/a/main.rs"), None);
    let (state, _) = state_with(vec![rust]);
    assert_shows(&drawn_at_width(&state, 5, 80).0, "│content:");
    assert_eq!(color_at(&state, "fn main"), Color::Blue);
    let text = listed("1-2", "agent", "Write", &input("/a/main"), None);
    let (state, _) = state_with(vec![text]);
    assert_eq!(color_at(&state, "fn main"), Color::Reset);
    let command = listed(
        "1-3",
        "agent",
        "Write",
        &json!({ "file_path": "/a/b.rs", "command": "fn main() {}" }),
        None,
    );
    let (state, _) = state_with(vec![command]);
    assert_eq!(color_at(&state, "fn main"), Color::Reset);
}

#[test]
fn should_show_both_texts_of_an_edit_call_as_one_diff() {
    let edit = listed(
        "1-1",
        "agent",
        "Edit",
        &json!({
            "file_path": "/a/main.rs",
            "old_string": "one\ntwo",
            "new_string": "one\n2",
            "replace_all": false,
        }),
        None,
    );
    let (state, _) = state_with(vec![edit]);
    let lines = drawn_at_width(&state, 5, 80).0;
    for part in [
        "│file_path: /a/main.rs",
        "│change:",
        "│   one",
        "│  -two",
        "│  +2",
        "│replace_all: false",
    ] {
        assert_shows(&lines, part);
    }
    assert!(!lines.iter().any(|line| line.contains("_string")));
    assert_eq!(color_at(&state, "-two"), Color::Red);
    assert_eq!(color_at(&state, "+2"), Color::Green);
}

#[test]
fn should_show_an_edit_call_with_one_text_as_it_is() {
    let edit = listed(
        "1-1",
        "agent",
        "Edit",
        &json!({ "file_path": "/a/main.rs", "new_string": "one", "old_string": 7 }),
        None,
    );
    let (state, _) = state_with(vec![edit]);
    let lines = drawn_at_width(&state, 5, 80).0;
    assert_shows(&lines, "│new_string: one");
    assert_shows(&lines, "│old_string: 7");
    assert!(!lines.iter().any(|line| line.contains("change:")));
}

#[test]
fn should_show_both_texts_of_an_edit_call_as_they_are_when_the_input_names_a_change() {
    let edit = listed(
        "1-1",
        "agent",
        "Edit",
        &json!({ "change": "-a\n+b", "old_string": "x", "new_string": "y" }),
        None,
    );
    let (state, _) = state_with(vec![edit]);
    let lines = drawn_at_width(&state, 5, 80).0;
    for part in [
        "│change:",
        "│  -a",
        "│  +b",
        "│old_string: x",
        "│new_string: y",
    ] {
        assert_shows(&lines, part);
    }
    assert_eq!(color_at(&state, "-a"), Color::Reset);
}

fn question_prompt() -> ListedEntry {
    let mut prompt = listed(
        "1-1",
        "agent",
        "AskUserQuestion",
        &json!({ "questions": [
            {
                "question": "Which library\u{1b}?",
                "header": "Library",
                "options": [
                    { "label": "date-fns", "description": "small" },
                    { "label": "luxon" },
                ],
                "multiSelect": false,
            },
            {
                "question": "Which features?",
                "header": "Features",
                "options": [{ "label": "zones" }, { "label": "locales" }],
                "multiSelect": true,
            },
        ] }),
        None,
    );
    prompt.entry.title = Some("Which library?".to_owned());
    prompt
}

#[test]
fn should_show_the_current_question_with_its_options_by_number() {
    let (state, _) = state_with(vec![question_prompt()]);
    let lines = drawn_at_width(&state, 5, 80).0;
    for part in [
        "│Question 1 of 2: Library",
        "│Which library\\u{1b}?",
        "│  1  ( ) date-fns: small",
        "│  2  ( ) luxon ",
        "│  t  an own answer as text",
        " 1-9  Mark    t  Own answer    Enter  Next    d  Decline    m  Message",
    ] {
        assert_shows(&lines, part);
    }
    assert!(!lines.iter().any(|line| line.contains("questions:")));
    assert!(!lines.iter().any(|line| line.contains("Answers so far")));
    assert!(!lines.iter().any(|line| line.contains("Approve")));
}

#[test]
fn should_show_the_picked_options_and_the_answers_so_far_of_a_question() {
    let (mut state, later) = state_with(vec![question_prompt()]);
    state.press(Key::Character('2'), later);
    assert_shows(&drawn_at_width(&state, 5, 80).0, "│  2  (x) luxon");
    state.press(Key::Enter, later);
    state.press(Key::Character('2'), later);
    let (lines, scroll_limit) = drawn_at_width(&state, 5, 80);
    for part in [
        "│Question 2 of 2: Features",
        "│  1  [ ] zones",
        "│  2  [x] locales",
        " 1-9  Mark    t  Own answer    Enter  Send    d  Decline    m  Message",
    ] {
        assert_shows(&lines, part);
    }
    state.limit_scroll(scroll_limit);
    state.press(Key::PageDown, later);
    let lines = drawn_at_width(&state, 5, 80).0;
    assert_shows(&lines, "│Answers so far, Esc goes back:");
    assert_shows(&lines, "│  Which library\\u{1b}?: luxon");
}

#[test]
fn should_show_the_own_answer_to_a_question_while_the_owner_types_it() {
    let (mut state, later) = state_with(vec![question_prompt()]);
    state.press(Key::Character('t'), later);
    state.press(Key::Character('x'), later);
    let lines = drawn_at_width(&state, 5, 80).0;
    assert_eq!(lines[22], "Own answer: x_");
    assert_eq!(
        lines[23],
        "Enter keeps the own answer   Esc closes the text"
    );
    state.press(Key::Enter, later);
    assert_shows(&drawn_at_width(&state, 5, 80).0, "│  t  own answer: x");
}

#[test]
fn should_show_a_question_prompt_that_the_list_cannot_answer_as_its_input() {
    let odd = listed(
        "1-1",
        "agent",
        "AskUserQuestion",
        &json!({ "questions": [] }),
        None,
    );
    let (state, _) = state_with(vec![odd]);
    let lines = drawn_at_width(&state, 5, 80).0;
    assert_shows(&lines, "│questions: []");
    assert_shows(&lines, " a  Approve ");
}

fn session(id: &str, activity: Activity, since: u64, waits_for: &[&str]) -> ListedSession {
    ListedSession {
        session: id.to_owned(),
        state: SessionState {
            user: "agent".to_owned(),
            user_is_verified: false,
            host: "claude-code".to_owned(),
            folder: "/home/agent/git/pont".to_owned(),
            pid: 1,
            activity,
            since,
            waits_for: waits_for.iter().map(|wait| (*wait).to_owned()).collect(),
            sent_at: 0,
        },
    }
}

#[test]
fn should_show_no_sessions_part_without_a_session() {
    let (state, _) = state_with(vec![push()]);
    assert!(
        !screen(&state, 0)
            .iter()
            .any(|line| line.contains("sessions:"))
    );
}

#[test]
fn should_show_each_session_with_its_lamp_and_the_idle_ones_first() {
    let (mut state, _) = state_with(vec![push()]);
    state.show_sessions(vec![
        session("aaaaaaaa-1", Activity::Active, RECEIVED - 5, &[]),
        session(
            "bbbbbbbb-2",
            Activity::Waiting,
            RECEIVED - 720,
            &["cargo test", "CI run"],
        ),
        session("cccccccc-3", Activity::Waiting, RECEIVED - 30, &["sleep"]),
        session("dddddddd-4", Activity::Idle, RECEIVED - 90, &[]),
    ]);
    let lines = screen(&state, 0);
    let part: Vec<&str> = lines
        .iter()
        .skip_while(|line| !line.contains(" sessions: 4 "))
        .skip(1)
        .take(4)
        .map(|line| line.split('│').nth(1).unwrap().trim_end())
        .collect();
    assert_eq!(
        part,
        [
            "[##] pont  idle 1m  ?agent  dddddddd",
            "[# ] pont  waits 12m: cargo test (+1)  ?ag",
            "[# ] pont  waits 30s: sleep  ?agent  ccccc",
            "[##] pont  active 5s  ?agent  aaaaaaaa",
        ]
    );
}

#[test]
fn should_keep_the_prompt_and_show_the_sessions_in_a_narrow_terminal() {
    let (mut state, _) = state_with(vec![push()]);
    state.show_sessions(vec![
        session("aaaaaaaa-1", Activity::Active, RECEIVED, &[]),
        session("bbbbbbbb-2", Activity::Waiting, RECEIVED - 300, &[]),
        session("cccccccc-3", Activity::Waiting, RECEIVED, &["a\nb"]),
        session("dddddddd-4", Activity::Idle, RECEIVED, &[]),
    ]);
    let lines = drawn_at_width(&state, 0, 80).0;
    assert_shows(&lines, "│Bash git push origin 1a2b:master ");
    assert_shows(&lines, "│agent  Bash  pont  0a1b ");
    for part in [
        "[##] pont  idle 0s  ?agent  dddddddd",
        "[# ] pont  waits 5m  ?agent  bbbbbbbb",
        "[# ] pont  waits 0s: a\\nb  ?agent  cccccccc",
        "[##] pont  active 0s  ?agent  aaaaaaaa",
    ] {
        assert_shows(&lines, part);
    }
}

#[test]
fn should_count_the_sessions_below_the_part() {
    let (mut state, _) = state_with(vec![push()]);
    state.show_sessions(
        (0..12)
            .map(|number| session(&format!("{number:02}"), Activity::Active, RECEIVED, &[]))
            .collect(),
    );
    assert_shows(&screen(&state, 0), " below: 4 ");
}
