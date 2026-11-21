use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Deserializer};
use serde_json::{Map, Value};

/// Longest request line, the newline included.
pub const MAX_REQUEST_BYTES: usize = 65_536;
pub const VERSION: u32 = 1;

/// One permission prompt of a harness session, as the hook of the harness
/// sends it.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub version: u32,
    /// The harness, such as `claude-code`.
    pub host: String,
    /// The user that the hook runs as. The service compares it with the sender.
    pub user: String,
    pub session: String,
    /// The working folder of the session.
    pub folder: String,
    /// The permission mode of the session.
    pub mode: String,
    pub tool: String,
    pub input: Map<String, Value>,
    /// Why a guard hook asked, or `null`.
    #[serde(deserialize_with = "required_or_null")]
    pub reason: Option<String>,
    /// True when the terminal of the session shows the prompt only after the
    /// hook ends, so only the list can answer it while it waits here.
    pub list_only: bool,
}

/// A field with its own reader has no default, so a request without the key
/// is refused.
fn required_or_null<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    Option::deserialize(deserializer)
}

impl Request {
    /// Parses one request line without its newline.
    pub fn parse(line: &str) -> Result<Self> {
        let request: Self = serde_json::from_str(line).context("Request has another form")?;
        ensure!(
            request.version == VERSION,
            "Request version {} is not {VERSION}",
            request.version
        );
        for (name, value, limit) in [
            ("host", &request.host, 32),
            ("user", &request.user, 64),
            ("session", &request.session, 128),
            ("folder", &request.folder, 4096),
            ("mode", &request.mode, 32),
            ("tool", &request.tool, 256),
        ] {
            ensure!(!value.is_empty(), "Request field {name} is empty");
            ensure!(
                value.len() <= limit,
                "Request field {name} is longer than {limit} bytes"
            );
        }
        ensure!(
            is_account_name(&request.user),
            "Request field user is no account name"
        );
        Ok(request)
    }
}

/// The form of a Linux account name. A name in this form cannot only look
/// like another name through spaces or characters of other scripts.
fn is_account_name(text: &str) -> bool {
    let mut bytes = text.bytes();
    bytes
        .next()
        .is_some_and(|first| first.is_ascii_lowercase() || first == b'_')
        && bytes.all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
        })
}
