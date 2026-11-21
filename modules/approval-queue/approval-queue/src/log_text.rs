//! The lines that the service writes to the journal.

use anyhow::Error;

use crate::{display_text::visible_line, entry::Entry, outcome::Outcome};

/// How one prompt ended. The text of the request shows as the list shows it,
/// so it cannot start a line of its own.
#[must_use]
pub fn served(entry: &Entry, outcome: &Outcome) -> String {
    format!(
        "Prompt of {} for {} in session {}: {outcome}",
        entry.user_text(),
        visible_line(&entry.tool),
        visible_line(&entry.session),
    )
}

/// Why the service ended without an outcome, with every inner reason.
#[must_use]
pub fn failed(error: &Error) -> String {
    visible_line(&format!("{error:#}"))
}
