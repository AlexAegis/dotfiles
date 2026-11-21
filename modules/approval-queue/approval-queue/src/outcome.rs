use std::fmt::{self, Display, Formatter};

use crate::answer::Answer;

/// How the wait for one prompt ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Answered(Answer),
    /// The hook closed the connection: the prompt got its answer in the
    /// terminal, or its session ended.
    ClientLeft,
    /// Nobody answered within the longest wait.
    Expired,
    /// No list was open for a prompt that only the list can answer, so the
    /// prompt went back to the terminal of its session.
    NoList,
}

impl Display for Outcome {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Answered(Answer::Allow) => "allowed",
            Self::Answered(Answer::Deny { .. }) => "denied",
            Self::ClientLeft => "left the queue",
            Self::Expired => "expired",
            Self::NoList => "went back to its terminal, no list is open",
        })
    }
}
