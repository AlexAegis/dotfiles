use approval_queue::highlight::{Language, code_lines, diff_lines};
use ratatui::{
    style::{Color, Modifier, Style},
    text::Line,
};

fn parts(line: &Line) -> Vec<(String, Option<Color>)> {
    line.spans
        .iter()
        .map(|span| (span.content.to_string(), span.style.fg))
        .collect()
}

fn text(line: &Line) -> String {
    line.spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect()
}

fn texts(lines: &[Line]) -> Vec<String> {
    lines.iter().map(text).collect()
}

fn style_of(line: &Line, part: &str) -> Style {
    line.spans
        .iter()
        .find(|span| span.content.contains(part))
        .unwrap_or_else(|| panic!("{part:?} is no part of {:?}", text(line)))
        .style
}

fn color_of(line: &Line, part: &str) -> Option<Color> {
    style_of(line, part).fg
}

#[test]
fn should_color_the_parts_of_a_shell_command_by_their_kind() {
    let lines = code_lines("if true; then echo \"a $HOME\" # note\nfi", Language::Shell);
    assert_eq!(
        texts(&lines),
        ["if true; then echo \"a $HOME\" # note", "fi"]
    );
    assert_eq!(color_of(&lines[0], "if"), Some(Color::Blue));
    assert_eq!(color_of(&lines[0], "echo"), Some(Color::Cyan));
    assert_eq!(color_of(&lines[0], "a "), Some(Color::Green));
    assert_eq!(color_of(&lines[1], "fi"), Some(Color::Blue));
}

#[test]
fn should_show_a_comment_dim_and_without_a_color() {
    let lines = code_lines("ls # note", Language::Shell);
    assert_eq!(
        style_of(&lines[0], "note"),
        Style::new().add_modifier(Modifier::DIM)
    );
    assert_eq!(style_of(&lines[0], "ls").add_modifier, Modifier::empty());
}

#[test]
fn should_take_the_paint_of_the_innermost_kind() {
    let shell = code_lines("echo \"a $HOME\"", Language::Shell);
    assert_eq!(color_of(&shell[0], "a "), Some(Color::Green));
    assert_eq!(color_of(&shell[0], "HOME"), None);
    let rust = code_lines("let a = \"b\\n\";", Language::OfPath("a.rs"));
    assert_eq!(color_of(&rust[0], "b"), Some(Color::Green));
    assert_eq!(color_of(&rust[0], "\\n"), Some(Color::Cyan));
}

#[test]
fn should_keep_the_state_of_the_syntax_over_the_lines() {
    let lines = code_lines("echo 'one\ntwo'\nls", Language::Shell);
    assert_eq!(color_of(&lines[1], "two"), Some(Color::Green));
    assert_ne!(color_of(&lines[2], "ls"), Some(Color::Green));
}

#[test]
fn should_take_the_language_from_the_extension_of_the_file() {
    let rust = code_lines("fn main() {} // x", Language::OfPath("/a/b.c/main.rs"));
    assert_eq!(color_of(&rust[0], "fn"), Some(Color::Blue));
    for path in ["a.ts", "a.tsx", "a.mts", "a.cts", "a.js"] {
        let script = code_lines("const a: number = 'b';", Language::OfPath(path));
        assert_eq!(color_of(&script[0], "const"), Some(Color::Blue), "{path}");
        assert_eq!(text(&script[0]), "const a: number = 'b';");
    }
    for path in ["/a/notes.unknown-kind", "/a.rs/notes", "", "/a/.rs/"] {
        let plain = code_lines("fn main() {} // x", Language::OfPath(path));
        assert_eq!(
            parts(&plain[0]),
            [("fn main() {} // x".to_owned(), None)],
            "{path}"
        );
    }
}

#[test]
fn should_show_control_characters_and_tabs_of_code_as_escapes() {
    let lines = code_lines("echo '\u{1b}[2J'\t\u{202e}x\r", Language::Shell);
    assert_eq!(text(&lines[0]), "echo '\\u{1b}[2J'\\t\\u{202e}x\\r");
    let diff = diff_lines("a\u{1b}", "\tb\u{7}");
    assert_eq!(texts(&diff), ["-a\\u{1b}", "+\\tb\\u{7}"]);
}

#[test]
fn should_show_a_tab_and_blanks_in_different_ways() {
    let tabs = ": <<-EOF\n\tbody\n\tEOF\np after";
    let blanks = tabs.replace('\t', "    ");
    assert_ne!(
        texts(&code_lines(tabs, Language::Shell)),
        texts(&code_lines(&blanks, Language::Shell))
    );
    assert_eq!(texts(&diff_lines("\tx", "    x")), ["-\\tx", "+    x"]);
}

#[test]
fn should_show_each_line_of_the_code_also_an_empty_one() {
    let lines = code_lines("a\n\nb\n", Language::Shell);
    assert_eq!(texts(&lines), ["a", "", "b", ""]);
}

#[test]
fn should_show_code_with_a_very_long_line_without_colors() {
    let colored = format!("echo 'a'\n# {}", "a".repeat(4094));
    assert_eq!(
        color_of(&code_lines(&colored, Language::Shell)[0], "a"),
        Some(Color::Green)
    );
    let long = format!("echo 'a'\n# {}", "a".repeat(4095));
    let lines = code_lines(&long, Language::Shell);
    assert_eq!(parts(&lines[0]), [("echo 'a'".to_owned(), None)]);
    assert_eq!(lines[1].spans.len(), 1);
    assert_eq!(lines[1].spans[0].style, Style::new());
    assert_eq!(text(&lines[1]).len(), 4097);
}

#[test]
fn should_mark_the_removed_the_added_and_the_kept_lines_of_a_diff() {
    let lines = diff_lines("one\ntwo\nthree\nfour", "one\n2\nthree\nfour\nfive");
    let shown: Vec<(String, Option<Color>)> = lines.iter().flat_map(parts).collect();
    assert_eq!(
        shown,
        [
            (" one".to_owned(), None),
            ("-two".to_owned(), Some(Color::Red)),
            ("+2".to_owned(), Some(Color::Green)),
            (" three".to_owned(), None),
            (" four".to_owned(), None),
            ("+five".to_owned(), Some(Color::Green)),
        ]
    );
}

#[test]
fn should_keep_the_common_lines_between_the_changes_of_a_diff() {
    let lines = diff_lines("x\na\nb\nc\ny", "p\na\nq\nc\nr");
    assert_eq!(
        texts(&lines),
        ["-x", "+p", " a", "-b", "+q", " c", "-y", "+r"]
    );
}

#[test]
fn should_show_a_diff_of_equal_texts_and_of_texts_without_a_common_line() {
    assert_eq!(texts(&diff_lines("a\nb", "a\nb")), [" a", " b"]);
    assert_eq!(texts(&diff_lines("a", "b\nc")), ["-a", "+b", "+c"]);
    assert_eq!(texts(&diff_lines("a\n", "a")), [" a", "-"]);
    assert_eq!(texts(&diff_lines("a\na", "a")), [" a", "-a"]);
}

#[test]
fn should_keep_the_common_first_and_last_lines_of_very_long_texts() {
    let old = format!("{}x\nsame", "a\n".repeat(2500));
    let new = format!("{}y\nsame", "a\n".repeat(2500));
    let lines = texts(&diff_lines(&old, &new));
    assert_eq!(lines.len(), 2503);
    assert_eq!(lines[2499..], [" a", "-x", "+y", " same"]);
}

#[test]
fn should_show_very_long_texts_that_differ_as_all_removed_then_all_added() {
    let old = format!("x\n{}y", "same\n".repeat(2500));
    let new = format!("p\n{}q", "same\n".repeat(2500));
    let lines = texts(&diff_lines(&old, &new));
    assert_eq!(lines.len(), 5004);
    assert_eq!(lines[..2], ["-x", "-same"]);
    assert_eq!(lines[2501..2504], ["-y", "+p", "+same"]);
    assert_eq!(lines[5003], "+q");
}

#[test]
fn should_show_the_lines_of_slow_code_without_colors_after_some_time() {
    let line = ":a".repeat(2048);
    let code = vec![line.as_str(); 15].join("\n");
    let lines = code_lines(&code, Language::OfPath("notes.rst"));
    assert_eq!(texts(&lines), vec![line.clone(); 15]);
    assert_eq!(parts(&lines[14]), [(line, None)]);
}
