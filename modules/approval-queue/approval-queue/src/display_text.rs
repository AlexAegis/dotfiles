//! Text of a request as a terminal can show it without acting on it.

/// The text on one line. Each control character, a line break included, and
/// each character that [`is_invisible`] names becomes its escape, such as
/// `\u{1b}`.
#[must_use]
pub fn visible_line(text: &str) -> String {
    text.chars().fold(String::new(), |mut line, character| {
        push_visible(&mut line, character);
        line
    })
}

/// The lines of the text, each as [`visible_line`] shows it.
#[must_use]
pub fn visible_lines(text: &str) -> Vec<String> {
    text.split('\n').map(visible_line).collect()
}

fn push_visible(line: &mut String, character: char) {
    if character.is_control() {
        line.extend(character.escape_default());
    } else if is_invisible(character) {
        line.extend(character.escape_unicode());
    } else {
        line.push(character);
    }
}

/// The characters that show as nothing or as a blank, or that change how the
/// text around them shows, such as the direction overrides: every white space
/// other than the space, and the listed formatting, filler and selector
/// characters. Not every such character of Unicode: the standard library has
/// no table of them.
fn is_invisible(character: char) -> bool {
    (character.is_whitespace() && character != ' ')
        || matches!(
            character,
            '\u{ad}'
                | '\u{34f}'
                | '\u{61c}'
                | '\u{115f}'..='\u{1160}'
                | '\u{17b4}'..='\u{17b5}'
                | '\u{180b}'..='\u{180f}'
                | '\u{200b}'..='\u{200f}'
                | '\u{202a}'..='\u{202e}'
                | '\u{2060}'..='\u{206f}'
                | '\u{2800}'
                | '\u{3164}'
                | '\u{fe00}'..='\u{fe0f}'
                | '\u{feff}'
                | '\u{ffa0}'
                | '\u{fff9}'..='\u{fffb}'
                | '\u{1bca0}'..='\u{1bca3}'
                | '\u{1d173}'..='\u{1d17a}'
                | '\u{e0000}'..='\u{e007f}'
                | '\u{e0100}'..='\u{e01ef}'
        )
}

/// The name of a user on one line, after `?` when the service could not
/// compare it with the sender of its line. The mark is first, so a short
/// column keeps it.
#[must_use]
pub fn user_text(name: &str, is_verified: bool) -> String {
    let name = visible_line(name);
    if is_verified {
        name
    } else {
        format!("?{name}")
    }
}
