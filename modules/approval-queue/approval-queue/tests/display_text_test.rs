use approval_queue::display_text::{visible_line, visible_lines};

#[test]
fn should_keep_text_that_shows_as_written() {
    for text in [
        "git push origin 1a2b:master",
        "",
        "árvíztűrő 日本語",
        "a  b",
    ] {
        assert_eq!(visible_line(text), text);
    }
}

#[test]
fn should_show_control_characters_as_escapes() {
    for (text, shown) in [
        ("\u{1b}[2Jrm -rf x", "\\u{1b}[2Jrm -rf x"),
        ("a\nb", "a\\nb"),
        ("a\r", "a\\r"),
        ("a\tb", "a\\tb"),
        ("\u{7}", "\\u{7}"),
        ("\u{7f}", "\\u{7f}"),
        ("\u{9b}31m", "\\u{9b}31m"),
        ("\u{0}", "\\u{0}"),
    ] {
        assert_eq!(visible_line(text), shown);
    }
}

#[test]
fn should_show_invisible_formatting_characters_as_escapes() {
    for (text, shown) in [
        ("a\u{202e}b", "a\\u{202e}b"),
        ("\u{2066}", "\\u{2066}"),
        ("\u{200b}", "\\u{200b}"),
        ("\u{200f}", "\\u{200f}"),
        ("\u{2028}", "\\u{2028}"),
        ("\u{feff}", "\\u{feff}"),
        ("\u{ad}", "\\u{ad}"),
        ("\u{e0041}", "\\u{e0041}"),
        ("\u{180e}", "\\u{180e}"),
        ("\u{fe0f}", "\\u{fe0f}"),
        ("\u{1bca0}", "\\u{1bca0}"),
        ("\u{1d173}", "\\u{1d173}"),
        ("\u{e0100}", "\\u{e0100}"),
    ] {
        assert_eq!(visible_line(text), shown);
    }
}

#[test]
fn should_split_a_text_at_its_line_breaks() {
    assert_eq!(visible_lines("echo a\necho b"), ["echo a", "echo b"]);
    assert_eq!(visible_lines("a\r\nb\u{1b}"), ["a\\r", "b\\u{1b}"]);
    assert_eq!(visible_lines("one"), ["one"]);
    assert_eq!(visible_lines("end\n"), ["end", ""]);
}

#[test]
fn should_show_blank_characters_other_than_the_space_as_escapes() {
    for (text, shown) in [
        ("alex\u{a0}", "alex\\u{a0}"),
        ("\u{2003}", "\\u{2003}"),
        ("\u{202f}", "\\u{202f}"),
        ("\u{3000}", "\\u{3000}"),
        ("\u{3164}", "\\u{3164}"),
        ("\u{115f}", "\\u{115f}"),
        ("\u{ffa0}", "\\u{ffa0}"),
        ("\u{2800}", "\\u{2800}"),
    ] {
        assert_eq!(visible_line(text), shown);
    }
}
