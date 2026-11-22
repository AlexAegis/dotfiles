use crate::answer::MAX_MESSAGE_CHARS;

/// The message for the next answer to an entry while the owner types it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MessageDraft {
    pub(crate) id: String,
    pub(crate) text: String,
}

impl MessageDraft {
    /// Adds the character when the message takes it: no control character,
    /// and no character past the longest message.
    pub(crate) fn push(&mut self, character: char) {
        if !character.is_control() && self.text.chars().count() < MAX_MESSAGE_CHARS {
            self.text.push(character);
        }
    }
}
