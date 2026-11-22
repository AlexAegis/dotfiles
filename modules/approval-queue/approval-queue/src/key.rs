/// A key press of the list, without the terminal library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    PageUp,
    PageDown,
    Enter,
    Escape,
    Backspace,
    /// Ctrl-C.
    Interrupt,
    Character(char),
    /// A key that the list does not use, with its name for a notice, such as
    /// `Tab` or `Ctrl+d`.
    Other(String),
}
