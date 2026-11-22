use std::collections::BTreeMap;

use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

/// Longest message of an answer and longest answer to one question, in
/// characters.
pub const MAX_MESSAGE_CHARS: usize = 500;
/// The most questions that one prompt asks.
pub const MAX_QUESTIONS: usize = 4;

/// The owner's answer to one prompt. Its JSON form is the answer file of the
/// list and the reply line to the hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "behavior", rename_all = "lowercase", deny_unknown_fields)]
pub enum Answer {
    Allow {
        /// For a prompt that asks questions: the answer to each question, by
        /// the text of the question.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        answers: Option<BTreeMap<String, String>>,
        /// What the owner tells the agent with the approval.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
    Deny {
        /// What the agent reads as the result of the denied call.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
}

impl Answer {
    /// The answer that allows a call as it is.
    pub const ALLOW: Self = Self::Allow {
        answers: None,
        message: None,
    };

    /// Refuses a message or answers that do not fit one reply line.
    pub fn check(&self) -> Result<()> {
        let (Self::Allow { message, .. } | Self::Deny { message }) = self;
        if let Some(message) = message {
            ensure!(
                !message.chars().any(char::is_control),
                "Message contains a control character"
            );
            ensure!(
                message.chars().count() <= MAX_MESSAGE_CHARS,
                "Message longer than {MAX_MESSAGE_CHARS} characters"
            );
        }
        if let Self::Allow {
            answers: Some(answers),
            ..
        } = self
        {
            ensure!(
                (1..=MAX_QUESTIONS).contains(&answers.len()),
                "Answers for {} questions, not for 1 to {MAX_QUESTIONS}",
                answers.len()
            );
            for answer in answers.values() {
                ensure!(
                    !answer.chars().any(char::is_control),
                    "Answer contains a control character"
                );
                ensure!(
                    (1..=MAX_MESSAGE_CHARS).contains(&answer.chars().count()),
                    "Answer is empty or longer than {MAX_MESSAGE_CHARS} characters"
                );
            }
        }
        Ok(())
    }
}
