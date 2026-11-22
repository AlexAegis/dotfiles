use std::time::Duration;

use crate::answer::Answer;

/// What the list does after a key press.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Answer {
        id: String,
        answer: Answer,
    },
    /// No ping comes for this long.
    MutePings(Duration),
    UnmutePings,
    Quit,
}
