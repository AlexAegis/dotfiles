//! What the tray icon shows, from the waiting prompts and the settings.

use crate::{alert_settings::AlertSettings, question::QUESTION_TOOL};

/// What a waiting entry asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Permission,
    Question,
}

impl EntryKind {
    #[must_use]
    pub fn of_tool(tool: &str) -> Self {
        if tool == QUESTION_TOOL {
            Self::Question
        } else {
            Self::Permission
        }
    }
}

/// What the tray icon and its menu show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TrayView {
    pub permissions: usize,
    pub questions: usize,
    /// The sessions that are idle for the time after which they ping.
    pub idle_sessions: usize,
    /// True for the short time after a ping.
    pub is_flashing: bool,
    /// The minutes that a mute still lasts.
    pub muted_minutes: Option<u64>,
    pub settings: AlertSettings,
}

impl TrayView {
    /// The name of the icon in the icon theme. A permission prompt shows
    /// before a question, and a question before an idle session.
    #[must_use]
    pub fn icon_name(&self) -> &'static str {
        if self.is_flashing {
            "approval-queue-flash"
        } else if self.permissions > 0 {
            "approval-queue-permission"
        } else if self.questions > 0 {
            "approval-queue-question"
        } else if self.idle_sessions > 0 {
            "approval-queue-idle"
        } else {
            "approval-queue-empty"
        }
    }

    /// True while a prompt or an idle session waits or a ping shows. The
    /// desktop then shows the icon also when it hides idle icons.
    #[must_use]
    pub fn needs_attention(&self) -> bool {
        self.is_flashing || self.permissions > 0 || self.questions > 0 || self.idle_sessions > 0
    }

    /// One line on what waits, for the tooltip.
    #[must_use]
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();
        if self.permissions > 0 {
            parts.push(format!("permission prompts: {}", self.permissions));
        }
        if self.questions > 0 {
            parts.push(format!("questions: {}", self.questions));
        }
        if self.idle_sessions > 0 {
            parts.push(format!("idle sessions: {}", self.idle_sessions));
        }
        if parts.is_empty() {
            parts.push("nothing waits".to_owned());
        }
        if let Some(minutes) = self.muted_minutes {
            parts.push(format!("pings muted for {minutes} min"));
        }
        if self.settings.high_alert {
            parts.push("high alert".to_owned());
        }
        parts.join(", ")
    }
}
