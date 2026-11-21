use crate::entry::Entry;

/// A waiting prompt with the ID that an answer names.
#[derive(Debug, Clone, PartialEq)]
pub struct ListedEntry {
    pub id: String,
    pub entry: Entry,
}
