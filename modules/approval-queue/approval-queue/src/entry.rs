use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{display_text::visible_line, request::Request};

/// One waiting prompt, as the service stores it for the list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    /// Seconds since the Unix epoch at which the service got the request.
    pub received: u64,
    pub user: String,
    /// True when the service compared the name with the sender of the
    /// request, which it can only for the owner of the queue.
    pub user_is_verified: bool,
    pub host: String,
    pub session: String,
    pub folder: String,
    pub mode: String,
    pub tool: String,
    /// The title that the request had.
    #[serde(default)]
    pub title: Option<String>,
    pub input: Map<String, Value>,
    pub reason: Option<String>,
    pub list_only: bool,
}

impl Entry {
    #[must_use]
    pub fn of(request: Request, received: u64, user_is_verified: bool) -> Self {
        Self {
            received,
            user: request.user,
            user_is_verified,
            host: request.host,
            session: request.session,
            folder: request.folder,
            mode: request.mode,
            tool: request.tool,
            title: request.title,
            input: request.input,
            reason: request.reason,
            list_only: request.list_only,
        }
    }

    /// The name of the user on one line, after `?` when the service could not
    /// compare it with the sender of the request. The mark is first, so a
    /// short column keeps it.
    #[must_use]
    pub fn user_text(&self) -> String {
        let name = visible_line(&self.user);
        if self.user_is_verified {
            name
        } else {
            format!("?{name}")
        }
    }
}
