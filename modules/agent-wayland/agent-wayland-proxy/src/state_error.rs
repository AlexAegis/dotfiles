use std::error::Error;

use anyhow::anyhow;

/// Converts an error that is not `Send`, such as a wl-proxy `StateError`, which holds
/// `Rc` values. Keeps the messages of its source chain.
pub fn describe(error: &dyn Error) -> anyhow::Error {
    let mut message = error.to_string();
    let mut source = error.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    anyhow!(message)
}
