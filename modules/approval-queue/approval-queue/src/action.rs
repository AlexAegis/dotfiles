use crate::answer::Answer;

/// What the list does after a key press.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Answer { id: String, answer: Answer },
    Quit,
}
