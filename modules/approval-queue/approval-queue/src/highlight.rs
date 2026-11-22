//! Code of a request as colored lines: a syntax by the language, and a line
//! diff of two texts.

use std::{
    sync::OnceLock,
    time::{Duration, Instant},
};

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
use syntect::parsing::{ParseState, Scope, ScopeStack, SyntaxReference, SyntaxSet};

use crate::display_text::visible_line;

/// Code with a longer line shows without colors: some syntaxes take
/// seconds for one line of the longest request.
const LONGEST_COLORED_LINE: usize = 4096;
/// The longest time that the colors of one piece of code take. The lines
/// after it show without colors.
const LONGEST_COLOR_TIME: Duration = Duration::from_millis(250);
/// Above this product of the line counts of both texts without their common
/// first and last lines, a diff shows those old lines and then those new
/// ones, without a search for their common lines.
const LONGEST_DIFF_TABLE: usize = 4_000_000;
/// The paint of each kind of code. The innermost scope that a prefix fits
/// sets the paint, by the first such prefix in this list. The colors are
/// those of the terminal. A comment is dim and has no color: the dark gray
/// of some color schemes is their background.
const PAINTS: [(&str, Paint); 13] = [
    ("comment", Paint::Dim),
    ("string", Paint::Color(Color::Green)),
    ("constant.character.escape", Paint::Color(Color::Cyan)),
    ("constant", Paint::Color(Color::Magenta)),
    ("keyword.operator", Paint::Plain),
    ("keyword", Paint::Color(Color::Blue)),
    ("storage", Paint::Color(Color::Blue)),
    ("entity.name.function", Paint::Color(Color::Cyan)),
    ("support.function", Paint::Color(Color::Cyan)),
    ("variable.function", Paint::Color(Color::Cyan)),
    ("entity.name", Paint::Color(Color::Yellow)),
    ("support.type", Paint::Color(Color::Yellow)),
    ("variable", Paint::Plain),
];

#[derive(Debug, Clone, Copy)]
enum Paint {
    Plain,
    Dim,
    Color(Color),
}

impl Paint {
    fn style(self) -> Style {
        match self {
            Self::Plain => Style::new(),
            Self::Dim => Style::new().add_modifier(Modifier::DIM),
            Self::Color(color) => Style::new().fg(color),
        }
    }
}

/// The syntax set has no TypeScript. Its files get the colors of JavaScript,
/// which fit most of their text.
const TYPESCRIPT_EXTENSIONS: [&str; 4] = ["ts", "tsx", "mts", "cts"];

/// The language of a piece of code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language<'a> {
    Shell,
    /// The language that the name of the file tells.
    OfPath(&'a str),
}

struct Syntaxes {
    set: SyntaxSet,
    paints: Vec<(Scope, Paint)>,
}

fn syntaxes() -> &'static Syntaxes {
    static SYNTAXES: OnceLock<Syntaxes> = OnceLock::new();
    SYNTAXES.get_or_init(|| Syntaxes {
        set: SyntaxSet::load_defaults_newlines(),
        paints: PAINTS
            .iter()
            .filter_map(|(prefix, paint)| Some((Scope::new(prefix).ok()?, *paint)))
            .collect(),
    })
}

impl Syntaxes {
    fn of(&self, language: Language) -> &SyntaxReference {
        match language {
            Language::Shell => self.set.find_syntax_by_token("bash"),
            Language::OfPath(path) => path
                .rsplit('/')
                .next()
                .and_then(|name| name.rsplit_once('.'))
                .and_then(|(_, extension)| {
                    let extension = if TYPESCRIPT_EXTENSIONS.contains(&extension) {
                        "js"
                    } else {
                        extension
                    };
                    self.set.find_syntax_by_extension(extension)
                }),
        }
        .unwrap_or_else(|| self.set.find_syntax_plain_text())
    }

    fn style(&self, scopes: &ScopeStack) -> Style {
        scopes
            .as_slice()
            .iter()
            .rev()
            .find_map(|scope| {
                self.paints
                    .iter()
                    .find(|(prefix, _)| prefix.is_prefix_of(*scope))
                    .map(|(_, paint)| paint.style())
            })
            .unwrap_or_default()
    }
}

/// The lines of the code, each part in the paint of its kind. A control
/// character shows as its escape, a tab too. Code in a language that the
/// syntax set does not have shows without colors, and so does code with a
/// line above `LONGEST_COLORED_LINE` bytes. The lines that follow after
/// `LONGEST_COLOR_TIME` show without colors too.
#[must_use]
pub fn code_lines(code: &str, language: Language) -> Vec<Line<'static>> {
    let syntaxes = syntaxes();
    let syntax = if code
        .split('\n')
        .any(|line| line.len() > LONGEST_COLORED_LINE)
    {
        syntaxes.set.find_syntax_plain_text()
    } else {
        syntaxes.of(language)
    };
    let mut state = ParseState::new(syntax);
    let mut scopes = ScopeStack::new();
    let end = Instant::now() + LONGEST_COLOR_TIME;
    code.split('\n')
        .map(|line| {
            if Instant::now() > end {
                return Line::from(painted(line, Style::new()));
            }
            let line = format!("{line}\n");
            let mut spans = Vec::new();
            let mut start = 0;
            let mut push = |end: usize, scopes: &ScopeStack| {
                if let Some(part) = line.get(start..end) {
                    let part = part.trim_end_matches('\n');
                    if !part.is_empty() {
                        spans.push(painted(part, syntaxes.style(scopes)));
                    }
                    start = end;
                }
            };
            for (offset, change) in state.parse_line(&line, &syntaxes.set).unwrap_or_default() {
                push(offset, &scopes);
                let _ = scopes.apply(&change);
            }
            push(line.len(), &scopes);
            Line::from(spans)
        })
        .collect()
}

/// The lines of both texts as a diff: a line of only the old text after `-`
/// in red, a line of only the new text after `+` in green, a common line
/// after a blank.
#[must_use]
pub fn diff_lines(old: &str, new: &str) -> Vec<Line<'static>> {
    let old: Vec<&str> = old.split('\n').collect();
    let new: Vec<&str> = new.split('\n').collect();
    let start = old
        .iter()
        .zip(&new)
        .take_while(|(old, new)| old == new)
        .count();
    let end = old[start..]
        .iter()
        .rev()
        .zip(new[start..].iter().rev())
        .take_while(|(old, new)| old == new)
        .count();
    let kept = |lines: &[&str]| -> Vec<Line<'static>> {
        lines
            .iter()
            .map(|text| Line::from(painted(&format!(" {text}"), Style::new())))
            .collect()
    };
    let mut lines = kept(&old[..start]);
    for change in changes(&old[start..old.len() - end], &new[start..new.len() - end]) {
        let (mark, text, style) = match change {
            Change::Removed(text) => ('-', text, Style::new().fg(Color::Red)),
            Change::Added(text) => ('+', text, Style::new().fg(Color::Green)),
            Change::Kept(text) => (' ', text, Style::new()),
        };
        lines.push(Line::from(painted(&format!("{mark}{text}"), style)));
    }
    lines.extend(kept(&old[old.len() - end..]));
    lines
}

fn painted(text: &str, style: Style) -> Span<'static> {
    Span::styled(visible_line(text), style)
}

enum Change<'a> {
    Removed(&'a str),
    Added(&'a str),
    Kept(&'a str),
}

/// The changes from the old lines to the new ones, with the longest common
/// run of lines kept.
fn changes<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<Change<'a>> {
    if old.len().saturating_mul(new.len()) > LONGEST_DIFF_TABLE {
        return old
            .iter()
            .map(|line| Change::Removed(line))
            .chain(new.iter().map(|line| Change::Added(line)))
            .collect();
    }
    let width = new.len() + 1;
    let mut common = vec![0_u32; (old.len() + 1) * width];
    for (i, old_line) in old.iter().enumerate().rev() {
        for (j, new_line) in new.iter().enumerate().rev() {
            common[i * width + j] = if old_line == new_line {
                common[(i + 1) * width + j + 1] + 1
            } else {
                common[(i + 1) * width + j].max(common[i * width + j + 1])
            };
        }
    }
    let mut changes = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < old.len() || j < new.len() {
        if i < old.len() && j < new.len() && old[i] == new[j] {
            changes.push(Change::Kept(old[i]));
            i += 1;
            j += 1;
        } else if j == new.len()
            || (i < old.len() && common[(i + 1) * width + j] >= common[i * width + j + 1])
        {
            changes.push(Change::Removed(old[i]));
            i += 1;
        } else {
            changes.push(Change::Added(new[j]));
            j += 1;
        }
    }
    changes
}
