use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

/// Longest request line, the newline included.
pub const MAX_REQUEST_BYTES: usize = 65_536;
pub const VERSION: u32 = 1;
/// Longest title of a request, in characters.
pub const MAX_TITLE_CHARACTERS: usize = 144;

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
    /// What the call is for, in one line that the sender wrote. A request
    /// may leave it out.
    #[serde(default)]
    pub title: Option<String>,
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
        if let Some(title) = &request.title {
            check_title(title)?;
        }
        ensure!(
            is_account_name(&request.user),
            "Request field user is no account name"
        );
        Ok(request)
    }
}

/// How risky a call looks to the program that wrote its title. It is a
/// hint for the owner and no check: the program reads text that the session
/// wrote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Risk {
    Ok,
    Suspicious,
    Dangerous,
}

impl Risk {
    /// The word of the risk, as an update line writes it.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Suspicious => "suspicious",
            Self::Dangerous => "dangerous",
        }
    }

    /// The risk that the word names.
    #[must_use]
    pub fn of_name(name: &str) -> Option<Self> {
        [Self::Ok, Self::Suspicious, Self::Dangerous]
            .into_iter()
            .find(|risk| risk.name() == name)
    }
}

/// A later line of the hook: a new title for its waiting prompt, such as a
/// summary that took time to write, and how risky the call looks.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TitleUpdate {
    pub title: String,
    #[serde(default)]
    pub risk: Option<Risk>,
}

impl TitleUpdate {
    /// Parses one update line without its newline.
    pub fn parse(line: &str) -> Result<Self> {
        let update: Self = serde_json::from_str(line).context("Update has another form")?;
        check_title(&update.title)?;
        Ok(update)
    }
}

/// Refuses a title that is no line of text of at most
/// `MAX_TITLE_CHARACTERS` characters.
pub fn check_title(title: &str) -> Result<()> {
    ensure!(!title.is_empty(), "Request field title is empty");
    ensure!(
        title.chars().count() <= MAX_TITLE_CHARACTERS,
        "Request field title is longer than {MAX_TITLE_CHARACTERS} characters"
    );
    ensure!(
        !title.chars().any(char::is_control),
        "Request field title has a control character"
    );
    Ok(())
}

/// The form of a Linux account name. A name in this form cannot only look
/// like another name through spaces or characters of other scripts.
pub(crate) fn is_account_name(text: &str) -> bool {
    let mut bytes = text.bytes();
    bytes
        .next()
        .is_some_and(|first| first.is_ascii_lowercase() || first == b'_')
        && bytes.all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_' || byte == b'-'
        })
}
