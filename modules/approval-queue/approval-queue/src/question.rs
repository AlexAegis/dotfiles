//! The questions of a prompt that asks the owner, as the `AskUserQuestion`
//! tool of Claude Code sends them.

use serde::Deserialize;
use serde_json::Value;

use crate::{
    answer::{MAX_MESSAGE_CHARS, MAX_QUESTIONS},
    entry::Entry,
};

/// The tool whose prompt is a question dialog.
pub const QUESTION_TOOL: &str = "AskUserQuestion";
/// The most options of one question that a key of the list can pick.
pub const MAX_OPTIONS: usize = 9;

/// One question of a prompt.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Question {
    /// The text of the question, which also names its answer.
    pub question: String,
    #[serde(default)]
    pub header: String,
    #[serde(default)]
    pub options: Vec<Choice>,
    /// True when the answer can have more than one option.
    #[serde(default, rename = "multiSelect")]
    pub multi_select: bool,
}

/// One option of a question.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Choice {
    pub label: String,
    #[serde(default)]
    pub description: String,
}

/// The questions of the entry. `None` for an entry of another tool, and for
/// questions that the list cannot answer: none, more than `MAX_QUESTIONS`, a
/// question with more than `MAX_OPTIONS` options, an empty or repeated
/// question text, an option that is no answer the queue takes, or another
/// form. The list then shows the entry as a prompt without questions.
///
/// An option is such an answer when its label has text, no control character
/// and, together with the other labels of its question and their separators,
/// at most `MAX_MESSAGE_CHARS` characters.
#[must_use]
pub fn questions_of(entry: &Entry) -> Option<Vec<Question>> {
    if entry.tool != QUESTION_TOOL {
        return None;
    }
    let questions: Vec<Question> =
        Vec::deserialize(entry.input.get("questions").unwrap_or(&Value::Null)).ok()?;
    let is_answerable = (1..=MAX_QUESTIONS).contains(&questions.len())
        && questions.iter().enumerate().all(|(index, question)| {
            !question.question.is_empty()
                && question.options.len() <= MAX_OPTIONS
                && question.options.iter().all(|option| {
                    !option.label.is_empty() && !option.label.chars().any(char::is_control)
                })
                && joined_length(question) <= MAX_MESSAGE_CHARS
                && questions[..index]
                    .iter()
                    .all(|earlier| earlier.question != question.question)
        });
    is_answerable.then_some(questions)
}

/// The length of the longest answer from the options of the question: one
/// label, or each label with a comma and a blank between two.
fn joined_length(question: &Question) -> usize {
    let lengths = question
        .options
        .iter()
        .map(|option| option.label.chars().count());
    if question.multi_select {
        lengths.sum::<usize>() + 2 * question.options.len().saturating_sub(1)
    } else {
        lengths.max().unwrap_or(0)
    }
}
