/// A key press of the list, without the terminal library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// A key that the list does not use, such as Tab or Ctrl-D.
    Other,
}
