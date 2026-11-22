//! The state of one harness session, as the hook of the harness reports it.

use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    display_text::user_text,
    request::{VERSION, is_account_name},
};

/// The most things that a waiting session names.
pub const MAX_WAITS: usize = 16;
/// Longest name of one thing that a session waits for, in characters.
pub const MAX_WAIT_CHARACTERS: usize = 144;

/// The longest time for which a report can say that its state already
/// lasts.
pub const MAX_SECONDS_AGO: u64 = 86_400;

/// A report can name a send time that is this long after the time of the
/// service, which reads its clock in whole seconds.
const MAX_SECONDS_AHEAD: u64 = 5;

/// The latest `sent_at` of a report that comes at `now`, in seconds since
/// the Unix epoch.
#[must_use]
pub fn latest_sent_at(now: u64) -> u64 {
    now.saturating_add(MAX_SECONDS_AHEAD).saturating_mul(1000)
}

/// What a session does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Activity {
    /// A turn runs.
    Active,
    /// No turn runs, and something that the session started still runs,
    /// such as a build or a wait for a CI run.
    Waiting,
    /// The turn ended with nothing in flight: the session needs its user.
    Idle,
}

/// The state that a report names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportedState {
    Active,
    Waiting,
    Idle,
    /// The session closed.
    Ended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Kind {
    Session,
}

/// One line of the hook of a harness on the state of its session.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionReport {
    pub version: u32,
    kind: Kind,
    pub host: String,
    /// The user that the hook runs as. The service compares it with the
    /// sender.
    pub user: String,
    pub session: String,
    /// The working folder of the session.
    pub folder: String,
    /// The process of the session. The state leaves with it.
    pub pid: u32,
    pub state: ReportedState,
    /// What a waiting session waits for, one line each.
    #[serde(default)]
    pub waits_for: Vec<String>,
    /// The seconds for which the session already had the state when the
    /// hook learned of it.
    #[serde(default)]
    pub seconds_ago: u64,
    /// True for an idle report that does not replace a waiting state: a
    /// signal that the session is idle, which also comes while the session
    /// waits.
    #[serde(default)]
    pub unless_waiting: bool,
    /// Milliseconds since the Unix epoch at which the hook wrote the
    /// report. The reports of one session apply in this order.
    pub sent_at: u64,
}

impl Activity {
    /// The name of the activity, as a report writes it.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Waiting => "waiting",
            Self::Idle => "idle",
        }
    }
}

impl SessionReport {
    /// True for a line that names its kind, which no request does.
    #[must_use]
    pub fn is_named_in(line: &str) -> bool {
        serde_json::from_str::<Value>(line).is_ok_and(|value| value.get("kind").is_some())
    }

    /// Parses one report line without its newline.
    pub fn parse(line: &str) -> Result<Self> {
        let report: Self = serde_json::from_str(line).context("Report has another form")?;
        ensure!(
            report.version == VERSION,
            "Report version {} is not {VERSION}",
            report.version
        );
        for (name, value, limit) in [
            ("host", &report.host, 32),
            ("user", &report.user, 64),
            ("folder", &report.folder, 4096),
        ] {
            ensure!(!value.is_empty(), "Report field {name} is empty");
            ensure!(
                value.len() <= limit,
                "Report field {name} is longer than {limit} bytes"
            );
        }
        ensure!(
            is_account_name(&report.user),
            "Report field user is no account name"
        );
        ensure!(
            report.seconds_ago <= MAX_SECONDS_AGO,
            "Report field seconds_ago is more than {MAX_SECONDS_AGO}"
        );
        ensure!(
            is_session_id(&report.session),
            "Report field session is no session ID"
        );
        ensure!(report.pid > 0, "Report field pid is 0");
        ensure!(
            !report.unless_waiting || report.state == ReportedState::Idle,
            "Report field unless_waiting is only for the state idle"
        );
        ensure!(
            report.waits_for.len() <= MAX_WAITS,
            "Report field waits_for has more than {MAX_WAITS} items"
        );
        ensure!(
            report
                .waits_for
                .iter()
                .all(|wait| wait.chars().count() <= MAX_WAIT_CHARACTERS),
            "Report field waits_for has an item longer than {MAX_WAIT_CHARACTERS} characters"
        );
        Ok(report)
    }
}

/// The form of a session ID that can name a file.
#[must_use]
pub fn is_session_id(text: &str) -> bool {
    !text.is_empty()
        && text.len() <= 128
        && text
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

/// The state of one session, as the service stores it for the list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionState {
    pub user: String,
    /// True when the service compared the name with the sender of the
    /// report, which it can only for the owner of the queue.
    pub user_is_verified: bool,
    pub host: String,
    pub folder: String,
    pub pid: u32,
    pub activity: Activity,
    /// Seconds since the Unix epoch at which the session got the activity.
    pub since: u64,
    pub waits_for: Vec<String>,
    /// The `sent_at` of the report that gave the state.
    pub sent_at: u64,
}

/// What a report does to the stored state of its session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    /// The session has this state now.
    Store(SessionState),
    /// The session ended.
    Remove,
    /// The state before stays: a later report gave it, or the session
    /// waits and the report does not replace a waiting state.
    Keep,
}

impl SessionState {
    /// What the report that came at `now` does to the state before. A
    /// session that keeps its activity keeps the time of it. A state before
    /// whose send time is after `now`, as after a step back of the clock,
    /// does not stay for that time.
    #[must_use]
    pub fn after(
        before: Option<&Self>,
        report: &SessionReport,
        now: u64,
        user_is_verified: bool,
    ) -> Change {
        if before.is_some_and(|before| {
            (before.sent_at > report.sent_at && before.sent_at <= latest_sent_at(now))
                || (report.unless_waiting && before.activity == Activity::Waiting)
        }) {
            return Change::Keep;
        }
        let activity = match report.state {
            ReportedState::Active => Activity::Active,
            ReportedState::Waiting => Activity::Waiting,
            ReportedState::Idle => Activity::Idle,
            ReportedState::Ended => return Change::Remove,
        };
        let since = before
            .filter(|before| before.activity == activity)
            .map_or_else(
                || now.saturating_sub(report.seconds_ago),
                |before| before.since,
            );
        Change::Store(Self {
            user: report.user.clone(),
            user_is_verified,
            host: report.host.clone(),
            folder: report.folder.clone(),
            pid: report.pid,
            activity,
            since,
            waits_for: if activity == Activity::Waiting {
                report.waits_for.clone()
            } else {
                Vec::new()
            },
            sent_at: report.sent_at,
        })
    }

    /// The name of the user on one line, after `?` when the service could
    /// not compare it with the sender of the report.
    #[must_use]
    pub fn user_text(&self) -> String {
        user_text(&self.user, self.user_is_verified)
    }
}

/// The state of a session with the ID of the session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListedSession {
    pub session: String,
    pub state: SessionState,
}
