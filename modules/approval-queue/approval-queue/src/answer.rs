use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};

/// Longest message of a denial, in characters.
pub const MAX_MESSAGE_CHARS: usize = 500;

/// The owner's answer to one prompt. Its JSON form is the answer file of the
/// list and the reply line to the hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "behavior", rename_all = "lowercase", deny_unknown_fields)]
pub enum Answer {
    Allow,
    Deny {
        /// What the agent reads as the result of the denied call.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
}

impl Answer {
    /// Refuses a message that does not fit one reply line.
    pub fn check(&self) -> Result<()> {
        if let Self::Deny {
            message: Some(message),
        } = self
        {
            ensure!(
                !message.chars().any(char::is_control),
                "Message contains a control character"
            );
            ensure!(
                message.chars().count() <= MAX_MESSAGE_CHARS,
                "Message longer than {MAX_MESSAGE_CHARS} characters"
            );
        }
        Ok(())
    }
}
