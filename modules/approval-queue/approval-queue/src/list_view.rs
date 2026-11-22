use std::{
    sync::{Mutex, PoisonError},
    time::{SystemTime, UNIX_EPOCH},
};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Wrap},
};
use serde_json::Value;

use crate::{
    answer::Answer,
    display_text::{visible_line, visible_lines},
    entry::Entry,
    highlight::{self, Language},
    list_state::ListState,
    listed_entry::ListedEntry,
    question_draft::QuestionDraft,
    queue_folder,
    request::MAX_TITLE_CHARACTERS,
    tray_view::EntryKind,
};

const KEYS: &str =
    "j/k select   PgUp/PgDn scroll   s/S mute pings 10 min/1 hour   u unmute   q quit";
const MESSAGE_KEYS: &str = "Enter or Esc keeps the message for the answer to this prompt";
const OWN_ANSWER_KEYS: &str = "Enter keeps the own answer   Esc closes the text";
const LIST_ONLY: &str = "only in this list: its terminal shows the prompt when no list is open";
/// The count of the characters of a session ID that an entry of the list shows.
const SESSION_START: usize = 8;
/// The lines of one entry in the list: kind, title, details and a gap.
const ENTRY_HEIGHT: u16 = 4;
/// From this width on, the list and the selected prompt are side by side.
const TWO_COLUMN_WIDTH: u16 = 100;
/// The width of the longest kind and one blank after it.
const KIND_WIDTH: u16 = 11;
/// The name under which the pane shows both texts of an Edit call.
const DIFF_NAME: &str = "change";
/// The lines of the pane that stay for the prompt above a long message.
const DETAIL_HEIGHT_BESIDE_MESSAGE: u16 = 4;
const QUESTION_ACTIONS: [(&str, &str); 5] = [
    ("1-9", "Mark"),
    ("t", "Own answer"),
    ("Enter", "Next"),
    ("d", "Decline"),
    ("m", "Message"),
];
const LAST_QUESTION_ACTIONS: [(&str, &str); 5] = [
    ("1-9", "Mark"),
    ("t", "Own answer"),
    ("Enter", "Send"),
    ("d", "Decline"),
    ("m", "Message"),
];
const ACTIONS: [(&str, &str); 3] = [("a", "Approve"), ("d", "Decline"), ("m", "Message")];

/// Draws the waiting prompts as a list of entries, the selected one in full
/// with its actions and the keys. Returns how far the pane of the selected
/// prompt scrolls: the count of its lines below the pane.
pub fn draw(frame: &mut Frame, state: &ListState, now: SystemTime) -> u16 {
    let [body_area, footer_area] =
        Layout::vertical([Constraint::Min(6), Constraint::Length(2)]).areas(frame.area());
    let [list_area, pane_area] = if body_area.width >= TWO_COLUMN_WIDTH {
        Layout::horizontal([Constraint::Percentage(40), Constraint::Fill(1)]).areas(body_area)
    } else {
        let list_height = (body_area.height / 2).min(
            u16::try_from(state.entries().len())
                .unwrap_or(u16::MAX)
                .saturating_mul(ENTRY_HEIGHT)
                .saturating_add(2)
                .max(ENTRY_HEIGHT + 2),
        );
        Layout::vertical([Constraint::Length(list_height), Constraint::Fill(1)]).areas(body_area)
    };
    draw_list(frame, list_area, state, now);
    let scroll_limit = draw_pane(frame, pane_area, state);
    let notice = state.notice().map(visible_line);
    let footer = match (state.message(), state.question_text()) {
        (Some(_), _) if state.message_is_of_selected() => {
            [notice.unwrap_or_default(), MESSAGE_KEYS.to_owned()]
        }
        (Some(typed), _) => [
            format!("Message: {}_", visible_line(typed)),
            notice.unwrap_or_default(),
        ],
        (None, Some(text)) => [
            format!("Own answer: {}_", visible_line(text)),
            notice.unwrap_or_else(|| OWN_ANSWER_KEYS.to_owned()),
        ],
        (None, None) => [notice.unwrap_or_default(), KEYS.to_owned()],
    };
    frame.render_widget(Paragraph::new(footer.map(Line::from).to_vec()), footer_area);
    scroll_limit
}

fn draw_list(frame: &mut Frame, area: Rect, state: &ListState, now: SystemTime) {
    let mute = state
        .pings_muted_until()
        .and_then(|until| queue_folder::muted_minutes(until, now));
    let now = now
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let count = state.entries().len();
    let mut block = Block::bordered().title(format!(" approval-queue: {count} waiting "));
    if let Some(minutes) = mute {
        block = block.title_bottom(format!(" pings muted for {minutes} min "));
    }
    let inner = block.inner(area);
    let shown = usize::from((inner.height.saturating_add(1) / ENTRY_HEIGHT).max(1));
    let selected = state.selected_index();
    let first = selected.map_or(0, |index| (index + 1).saturating_sub(shown));
    if first > 0 {
        block = block.title_top(Line::from(format!(" above: {first} ")).right_aligned());
    }
    let below = count.saturating_sub(first + shown);
    if below > 0 {
        block = block.title_bottom(Line::from(format!(" below: {below} ")).right_aligned());
    }
    frame.render_widget(block, area);
    if count == 0 {
        frame.render_widget(Paragraph::new("No prompt waits"), inner);
        return;
    }
    let mut top = inner.y;
    for (index, listed) in state.entries().iter().enumerate().skip(first).take(shown) {
        let height = (ENTRY_HEIGHT - 1).min(inner.bottom().saturating_sub(top));
        let entry_area = Rect::new(inner.x, top, inner.width, height);
        draw_entry(
            frame,
            entry_area,
            listed,
            state,
            now,
            selected == Some(index),
        );
        top = top.saturating_add(ENTRY_HEIGHT);
    }
}

fn draw_entry(
    frame: &mut Frame,
    area: Rect,
    listed: &ListedEntry,
    state: &ListState,
    now: u64,
    is_selected: bool,
) {
    let entry = &listed.entry;
    let base = if is_selected {
        Style::new().add_modifier(Modifier::REVERSED)
    } else {
        Style::new()
    };
    let waits = match state.answer_of(&listed.id) {
        Some(Answer::Allow { .. }) => "approved".to_owned(),
        Some(Answer::Deny { .. }) => "declined".to_owned(),
        None => age(now.saturating_sub(entry.received)),
    };
    let session: String = visible_line(&entry.session)
        .chars()
        .take(SESSION_START)
        .collect();
    let lines = vec![
        Line::from(kind_span(entry)),
        Line::from(Span::styled(
            title(entry),
            Style::new().add_modifier(Modifier::BOLD),
        )),
        Line::from(format!(
            "{}  {}  {}  {session}",
            entry.user_text(),
            visible_line(&entry.tool),
            folder_name(&entry.folder),
        )),
    ];
    frame.render_widget(Paragraph::new(lines).style(base), area);
    let waits_width = u16::try_from(waits.chars().count()).unwrap_or(u16::MAX);
    if area.height > 0 && area.width >= KIND_WIDTH.saturating_add(waits_width) {
        frame.render_widget(
            Paragraph::new(waits).style(base),
            Rect {
                x: area.x + area.width - waits_width,
                width: waits_width,
                height: 1,
                ..area
            },
        );
    }
}

fn draw_pane(frame: &mut Frame, area: Rect, state: &ListState) -> u16 {
    let Some(listed) = state.selected() else {
        frame.render_widget(
            Paragraph::new("No prompt waits").block(Block::bordered()),
            area,
        );
        return 0;
    };
    let entry = &listed.entry;
    let mut pane = Block::bordered().title(Line::from(vec![
        Span::raw(" "),
        kind_span(entry),
        Span::raw(format!(": {} ", visible_line(&entry.tool))),
    ]));
    let inner = pane.inner(area);
    let (message, is_typed) = message_field(state);
    let has_message = !message.is_empty();
    let message = Paragraph::new(message).wrap(Wrap { trim: false });
    let message_lines = if has_message {
        u16::try_from(message.line_count(inner.width)).unwrap_or(u16::MAX)
    } else {
        0
    };
    let message_height = message_lines.min(
        inner
            .height
            .saturating_sub(DETAIL_HEIGHT_BESIDE_MESSAGE + 1)
            .max(1),
    );
    let message_lines_hidden = message_lines - message_height;
    let [detail_area, action_area, message_area] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(message_height),
    ])
    .areas(inner);
    let questions = state.question_draft();
    let lines = match &questions {
        Some(draft) => {
            let mut lines = head(entry);
            lines.extend(question_lines(draft));
            lines
        }
        None => detail_of(entry),
    };
    let detail = Paragraph::new(lines).wrap(Wrap { trim: false });
    let scroll_limit = u16::try_from(detail.line_count(detail_area.width))
        .unwrap_or(u16::MAX)
        .saturating_sub(detail_area.height);
    let scroll = state.scroll().min(scroll_limit);
    if scroll > 0 {
        pane = pane.title_top(Line::from(format!(" lines above: {scroll}, PgUp ")).right_aligned());
    }
    if scroll < scroll_limit {
        pane = pane.title_bottom(format!(" lines below: {}, PgDn ", scroll_limit - scroll));
    }
    if message_lines_hidden > 0 {
        pane = pane.title_bottom(
            Line::from(format!(
                " message: {message_lines_hidden} more lines, in a larger window "
            ))
            .right_aligned(),
        );
    }
    frame.render_widget(pane, area);
    frame.render_widget(detail.scroll((scroll, 0)), detail_area);
    let actions = match &questions {
        Some(draft) if draft.is_at_last() => &LAST_QUESTION_ACTIONS[..],
        Some(_) => &QUESTION_ACTIONS[..],
        None => &ACTIONS[..],
    };
    let actions: Vec<Span> = actions
        .iter()
        .flat_map(|(key, name)| {
            [
                Span::styled(
                    format!(" {key} "),
                    Style::new().add_modifier(Modifier::REVERSED),
                ),
                Span::raw(format!(" {name}   ")),
            ]
        })
        .collect();
    frame.render_widget(Paragraph::new(Line::from(actions)), action_area);
    let above = if is_typed { message_lines_hidden } else { 0 };
    frame.render_widget(message.scroll((above, 0)), message_area);
    scroll_limit
}

/// The text of the message field below the actions, and whether the owner
/// types it: the open message of the selected prompt, else the message that
/// the next answer to that prompt gets. The field wraps and shows the whole
/// message. In a pane too small for it, the field has at least one line and
/// shows the end of a message that the owner types, and the start of a kept
/// one. The border of the pane then counts the lines that do not show. An
/// empty text for no message.
fn message_field(state: &ListState) -> (String, bool) {
    match (state.message(), state.kept_message()) {
        (Some(typed), _) if state.message_is_of_selected() => {
            (format!("Message: {}_", visible_line(typed)), true)
        }
        (_, Some(kept)) => (format!("Message: {}", visible_line(kept)), false),
        _ => (String::new(), false),
    }
}

fn kind_span<'a>(entry: &Entry) -> Span<'a> {
    let (name, color) = match EntryKind::of_tool(&entry.tool) {
        EntryKind::Permission => ("PERMISSION", Color::Yellow),
        EntryKind::Question => ("QUESTION", Color::Blue),
    };
    Span::styled(name, Style::new().fg(color).add_modifier(Modifier::BOLD))
}

/// The title that the request had. Without one: the tool and the part of
/// the input that names the call, on one line of at most
/// `MAX_TITLE_CHARACTERS` characters.
fn title(entry: &Entry) -> String {
    if let Some(title) = &entry.title {
        return visible_line(title);
    }
    let summary = summary(entry);
    let words: Vec<&str> = entry
        .tool
        .split_whitespace()
        .chain(summary.split_whitespace())
        .collect();
    let title = visible_line(&words.join(" "));
    if title.chars().count() <= MAX_TITLE_CHARACTERS {
        return title;
    }
    let start: String = title.chars().take(MAX_TITLE_CHARACTERS - 3).collect();
    format!("{start}...")
}

/// The last part of the path of the folder.
fn folder_name(folder: &str) -> String {
    let name = folder
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .unwrap_or(folder);
    visible_line(name)
}

/// The part of the input that names the call: the command, else the file,
/// else the whole input.
fn summary(entry: &Entry) -> String {
    ["command", "file_path"]
        .into_iter()
        .find_map(|key| entry.input.get(key).and_then(Value::as_str))
        .map_or_else(
            || Value::Object(entry.input.clone()).to_string(),
            str::to_owned,
        )
}

/// The lines of the pane for the prompt. The lines of the last prompt stay
/// for the next draw of the same entry: the colors of a long script take
/// time.
fn detail_of(entry: &Entry) -> Vec<Line<'static>> {
    static LAST: Mutex<Option<(String, Vec<Line<'static>>)>> = Mutex::new(None);
    let Ok(key) = serde_json::to_string(entry) else {
        return detail(entry);
    };
    let mut last = LAST.lock().unwrap_or_else(PoisonError::into_inner);
    match &*last {
        Some((last_key, lines)) if *last_key == key => lines.clone(),
        _ => {
            let lines = detail(entry);
            *last = Some((key, lines.clone()));
            lines
        }
    }
}

/// The first lines of the pane: who asks and where, why, and the title.
fn head(entry: &Entry) -> Vec<Line<'static>> {
    let mut lines = vec![
        format!(
            "user {}   host {}   mode {}",
            entry.user_text(),
            visible_line(&entry.host),
            visible_line(&entry.mode),
        ),
        format!("session {}", visible_line(&entry.session)),
        format!("folder {}", visible_line(&entry.folder)),
    ];
    if entry.list_only {
        lines.push(LIST_ONLY.to_owned());
    }
    if let Some(reason) = &entry.reason {
        push_field(&mut lines, "reason", reason);
    }
    let mut lines: Vec<Line<'static>> = lines.into_iter().map(Line::from).collect();
    lines.push(Line::default());
    let title = title(entry);
    lines.push(Line::from(Span::styled(
        title.clone(),
        Style::new().add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::default());
    lines
}

/// The lines of the pane for the questions of a prompt: the current
/// question with its options by number, and the answers so far.
fn question_lines(draft: &QuestionDraft) -> Vec<Line<'static>> {
    let question = draft.current();
    let (number, count) = draft.position();
    let mut lines = vec![
        Line::from(if question.header.is_empty() {
            format!("Question {number} of {count}")
        } else {
            format!(
                "Question {number} of {count}: {}",
                visible_line(&question.header)
            )
        }),
        Line::from(Span::styled(
            visible_line(&question.question),
            Style::new().add_modifier(Modifier::BOLD),
        )),
        Line::default(),
    ];
    for (index, choice) in question.options.iter().enumerate() {
        let mark = match (question.multi_select, draft.is_picked(index)) {
            (false, false) => "( ) ",
            (false, true) => "(x) ",
            (true, false) => "[ ] ",
            (true, true) => "[x] ",
        };
        let description = if choice.description.is_empty() {
            String::new()
        } else {
            format!(": {}", visible_line(&choice.description))
        };
        lines.push(Line::from(format!(
            "  {}  {mark}{}{description}",
            index + 1,
            visible_line(&choice.label)
        )));
    }
    lines.push(Line::from(match draft.own_answer() {
        Some(own) => format!("  t  own answer: {}", visible_line(own)),
        None => "  t  an own answer as text".to_owned(),
    }));
    if draft.answered().next().is_some() {
        lines.push(Line::default());
        lines.push(Line::from("Answers so far, Esc goes back:"));
        for (question, answer) in draft.answered() {
            lines.push(Line::from(format!(
                "  {}: {}",
                visible_line(question),
                visible_line(&answer)
            )));
        }
    }
    lines
}

fn detail(entry: &Entry) -> Vec<Line<'static>> {
    let mut lines = head(entry);
    let title = title(entry);
    for (key, value) in &entry.input {
        let name = visible_line(key);
        match value {
            // The title above already shows this text
            Value::String(text) if key == "description" && visible_line(text) == title => {}
            Value::String(text) => match code(entry, key, text) {
                Some(Code::Lines(name, code)) => push_code(&mut lines, name, code),
                Some(Code::InOtherField) => {}
                None => {
                    let mut field = Vec::new();
                    push_field(&mut field, &name, text);
                    lines.extend(field.into_iter().map(Line::from));
                }
            },
            other => lines.push(Line::from(format!(
                "{name}: {}",
                visible_line(&other.to_string())
            ))),
        }
    }
    lines
}

/// An input text that the pane shows as code.
enum Code {
    /// The name of the field and its colored lines.
    Lines(&'static str, Vec<Line<'static>>),
    /// The lines of another field of the input show this text.
    InOtherField,
}

/// The code of an input field: the command of a Bash call in the colors of
/// a shell script, the content of a Write call in those of the language of
/// its file, and both texts of an Edit call as one diff.
fn code(entry: &Entry, key: &str, text: &str) -> Option<Code> {
    let other = |key: &str| entry.input.get(key).and_then(Value::as_str);
    // The diff shows under a name that the input does not use
    let has_diff = !entry.input.contains_key(DIFF_NAME);
    match (entry.tool.as_str(), key) {
        ("Bash", "command") => Some(Code::Lines(
            "command",
            highlight::code_lines(text, Language::Shell),
        )),
        ("Write", "content") => Some(Code::Lines(
            "content",
            highlight::code_lines(
                text,
                Language::OfPath(other("file_path").unwrap_or_default()),
            ),
        )),
        ("Edit", "old_string") if has_diff => {
            other("new_string").map(|new| Code::Lines(DIFF_NAME, highlight::diff_lines(text, new)))
        }
        ("Edit", "new_string") if has_diff => other("old_string").map(|_| Code::InOtherField),
        _ => None,
    }
}

/// One line for one line of code, else the name and the code below it.
fn push_code(lines: &mut Vec<Line<'static>>, name: &str, mut code: Vec<Line<'static>>) {
    if code.len() == 1 {
        let mut spans = vec![Span::raw(format!("{name}: "))];
        spans.append(&mut code[0].spans);
        lines.push(Line::from(spans));
    } else {
        lines.push(Line::from(format!("{name}:")));
        lines.extend(code.into_iter().map(|mut line| {
            line.spans.insert(0, Span::raw("  "));
            line
        }));
    }
}

/// One line for a short text, else the name and the text below it.
fn push_field(lines: &mut Vec<String>, name: &str, text: &str) {
    let text_lines = visible_lines(text);
    if let [line] = text_lines.as_slice() {
        lines.push(format!("{name}: {line}"));
    } else {
        lines.push(format!("{name}:"));
        lines.extend(text_lines.into_iter().map(|line| format!("  {line}")));
    }
}

fn age(seconds: u64) -> String {
    match seconds {
        0..60 => format!("{seconds}s"),
        60..3600 => format!("{}m", seconds / 60),
        3600..86_400 => format!("{}h", seconds / 3600),
        _ => format!("{}d", seconds / 86_400),
    }
}
