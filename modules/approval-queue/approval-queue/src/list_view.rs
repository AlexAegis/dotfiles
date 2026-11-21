use std::time::{SystemTime, UNIX_EPOCH};

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Paragraph, Row, Table, TableState, Wrap},
};
use serde_json::Value;

use crate::{
    answer::Answer,
    display_text::{visible_line, visible_lines},
    entry::Entry,
    list_state::ListState,
    listed_entry::ListedEntry,
};

const KEYS: &str =
    "a allow   d deny   m deny with a message   j/k select   PgUp/PgDn scroll   q quit";
const MESSAGE_KEYS: &str = "Enter send   Esc cancel";
const LIST_ONLY: &str = "only in this list: its terminal shows the prompt when no list is open";
/// The width of the session column, which shows the start of the ID.
const SESSION_START: u16 = 8;

/// Draws the list: the waiting prompts, the selected one in full and the keys.
/// Returns how far the pane of the selected prompt scrolls: the count of its
/// lines below the pane.
pub fn draw(frame: &mut Frame, state: &ListState, now: SystemTime) -> u16 {
    let now = now
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    let count = state.entries().len();
    let table_height = u16::try_from(count)
        .unwrap_or(u16::MAX)
        .saturating_add(3)
        .min(frame.area().height / 2);
    let [table_area, detail_area, footer_area] = Layout::vertical([
        Constraint::Length(table_height),
        Constraint::Min(3),
        Constraint::Length(2),
    ])
    .areas(frame.area());

    let rows = state.entries().iter().map(|listed| row(listed, state, now));
    let table = Table::new(
        rows,
        [
            Constraint::Length(7),
            Constraint::Length(12),
            Constraint::Length(SESSION_START),
            Constraint::Length(12),
            Constraint::Percentage(30),
            Constraint::Fill(1),
        ],
    )
    .header(Row::new([
        "waits", "user", "session", "tool", "folder", "prompt",
    ]))
    .row_highlight_style(Style::new().add_modifier(Modifier::REVERSED))
    .block(Block::bordered().title(format!(" approval-queue: {count} waiting ")));
    let mut table_state = TableState::new().with_selected(state.selected_index());
    frame.render_stateful_widget(table, table_area, &mut table_state);

    let detail = state.selected().map_or_else(
        || vec![Line::from("No prompt waits")],
        |listed| detail(&listed.entry),
    );
    let detail = Paragraph::new(detail).wrap(Wrap { trim: false });
    let mut pane = Block::bordered();
    let inner = pane.inner(detail_area);
    let scroll_limit = u16::try_from(detail.line_count(inner.width))
        .unwrap_or(u16::MAX)
        .saturating_sub(inner.height);
    let scroll = state.scroll().min(scroll_limit);
    if scroll > 0 {
        pane = pane.title_top(format!(" lines above: {scroll}, PgUp "));
    }
    if scroll < scroll_limit {
        pane = pane.title_bottom(format!(" lines below: {}, PgDn ", scroll_limit - scroll));
    }
    frame.render_widget(detail.scroll((scroll, 0)).block(pane), detail_area);

    let notice = state.notice().map(visible_line);
    let (status, keys) = match state.message() {
        Some(message) => (
            format!("Message of the denial: {}_", visible_line(message)),
            notice.unwrap_or_else(|| MESSAGE_KEYS.to_owned()),
        ),
        None => (notice.unwrap_or_default(), KEYS.to_owned()),
    };
    frame.render_widget(
        Paragraph::new(vec![Line::from(status), Line::from(keys)]),
        footer_area,
    );
    scroll_limit
}

fn row<'a>(listed: &ListedEntry, state: &ListState, now: u64) -> Row<'a> {
    let entry = &listed.entry;
    let waits = match state.answer_of(&listed.id) {
        Some(Answer::Allow) => "allowed".to_owned(),
        Some(Answer::Deny { .. }) => "denied".to_owned(),
        None => age(now.saturating_sub(entry.received)),
    };
    Row::new([
        waits,
        entry.user_text(),
        visible_line(&entry.session),
        visible_line(&entry.tool),
        visible_line(&entry.folder),
        summary(entry),
    ])
}

/// The part of the input that names the call: the command, else the file,
/// else the whole input.
fn summary(entry: &Entry) -> String {
    ["command", "file_path"]
        .into_iter()
        .find_map(|key| entry.input.get(key).and_then(Value::as_str))
        .map_or_else(
            || visible_line(&Value::Object(entry.input.clone()).to_string()),
            visible_line,
        )
}

fn detail<'a>(entry: &Entry) -> Vec<Line<'a>> {
    let mut lines = vec![
        format!(
            "user {}   host {}   mode {}   session {}",
            entry.user_text(),
            visible_line(&entry.host),
            visible_line(&entry.mode),
            visible_line(&entry.session),
        ),
        format!("folder {}", visible_line(&entry.folder)),
    ];
    if entry.list_only {
        lines.push(LIST_ONLY.to_owned());
    }
    if let Some(reason) = &entry.reason {
        push_field(&mut lines, "reason", reason);
    }
    lines.push(format!("tool {}", visible_line(&entry.tool)));
    for (key, value) in &entry.input {
        match value {
            Value::String(text) => push_field(&mut lines, &visible_line(key), text),
            other => lines.push(format!(
                "{}: {}",
                visible_line(key),
                visible_line(&other.to_string())
            )),
        }
    }
    lines.into_iter().map(Line::from).collect()
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
