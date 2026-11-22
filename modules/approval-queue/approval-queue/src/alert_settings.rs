//! The settings of the tray menu and their file.

use std::{fs, path::Path, time::Duration};

use anyhow::{Context, Result};
use ping_throttle::Cadence;
use serde::{Deserialize, Serialize};

use crate::ping_watch::CADENCE;

/// How often pings come: a named set of cadence values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CadencePreset {
    Often,
    #[default]
    Normal,
    Seldom,
}

impl CadencePreset {
    pub const ALL: [Self; 3] = [Self::Often, Self::Normal, Self::Seldom];

    #[must_use]
    pub fn cadence(self) -> Cadence {
        match self {
            Self::Often => Cadence {
                quiet_time: Duration::from_secs(10),
                batch_size: 2,
                longest_wait: Duration::from_secs(30),
            },
            Self::Normal => CADENCE,
            Self::Seldom => Cadence {
                quiet_time: Duration::from_mins(2),
                batch_size: 8,
                longest_wait: Duration::from_mins(10),
            },
        }
    }

    /// The text of the preset in a menu.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Often => "Often: 10 s quiet, then 2 prompts or 30 s",
            Self::Normal => "Normal: 30 s quiet, then 4 prompts or 2 min",
            Self::Seldom => "Seldom: 2 min quiet, then 8 prompts or 10 min",
        }
    }
}

/// What a ping does and how often pings come. The owner changes it in the
/// menu of the tray icon.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AlertSettings {
    /// A ping also brings the list to view. Without it, a ping only changes
    /// the tray icon, while one shows.
    pub high_alert: bool,
    pub cadence: CadencePreset,
}

impl AlertSettings {
    /// The stored settings. The defaults when the file is missing or has
    /// another form.
    #[must_use]
    pub fn load(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn store(&self, path: &Path) -> Result<()> {
        let temporary = path.with_extension("tmp");
        let stored = fs::write(&temporary, serde_json::to_vec(self)?)
            .and_then(|()| fs::rename(&temporary, path));
        if stored.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        stored.with_context(|| format!("Settings {} not stored", path.display()))
    }
}
