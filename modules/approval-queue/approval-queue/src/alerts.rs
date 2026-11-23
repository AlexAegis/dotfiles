//! The waiting prompts with their pings and the settings of the owner.

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::Result;

use crate::{
    alert_settings::{AlertSettings, CadencePreset},
    ping_watch::PingWatch,
    queue_folder::{self, QueueFolder},
    session_state::Activity,
    tray_view::{EntryKind, TrayView},
};

/// How long the icon of a ping shows before the icon of what waits.
pub const FLASH_TIME: Duration = Duration::from_millis(400);

/// A session pings and counts for the tray after this time without a turn
/// and with nothing in flight. A session that its user answers sooner stays
/// quiet.
pub const IDLE_PING_AFTER: Duration = Duration::from_mins(1);

/// What the owner chose in the tray.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayCommand {
    OpenList,
    ToggleHighAlert,
    Mute(Duration),
    Unmute,
    SetCadence(CadencePreset),
    /// Stop the whole queue. The tray program carries it out.
    Quit,
}

/// The result of one read of the queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Refreshed {
    /// What the tray shows.
    pub view: TrayView,
    /// The count of the prompts and idle sessions of a ping that came.
    pub pinged: Option<usize>,
    /// True when the ping also brings the list to view.
    pub shows_list: bool,
}

#[derive(Debug)]
pub struct Alerts {
    queue: QueueFolder,
    settings: AlertSettings,
    settings_path: PathBuf,
    watch: PingWatch,
    pings: Receiver<usize>,
    kinds: HashMap<String, EntryKind>,
    flash_until: Option<Instant>,
}

impl Alerts {
    /// Loads the settings from the path. Stores each change of them there.
    #[must_use]
    pub fn new(queue: QueueFolder, settings_path: PathBuf) -> Self {
        let settings = AlertSettings::load(&settings_path);
        let (ping_sender, pings) = mpsc::channel();
        Self {
            queue,
            settings,
            settings_path,
            watch: PingWatch::new(settings.cadence.cadence(), move |ping| {
                let _ = ping_sender.send(ping.keys.len());
            }),
            pings,
            kinds: HashMap::new(),
            flash_until: None,
        }
    }

    #[must_use]
    pub fn queue(&self) -> &QueueFolder {
        &self.queue
    }

    #[must_use]
    pub fn settings(&self) -> AlertSettings {
        self.settings
    }

    /// Carries out a command of the tray. `Ok(true)` when it asks for the
    /// list. A setting that could not be stored still applies until the
    /// program ends.
    pub fn carry_out(&mut self, command: TrayCommand, now: SystemTime) -> Result<bool> {
        match command {
            TrayCommand::OpenList => return Ok(true),
            TrayCommand::Quit => {}
            TrayCommand::ToggleHighAlert => {
                self.settings.high_alert = !self.settings.high_alert;
                self.settings.store(&self.settings_path)?;
            }
            TrayCommand::SetCadence(preset) => {
                self.settings.cadence = preset;
                self.watch.set_cadence(preset.cadence());
                self.settings.store(&self.settings_path)?;
            }
            TrayCommand::Mute(time) => self.queue.mute_pings_until(now + time)?,
            TrayCommand::Unmute => self.queue.unmute_pings()?,
        }
        Ok(false)
    }

    /// Reads the queue and pings when the cadence allows it, for each new
    /// prompt and for each session that is idle for `IDLE_PING_AFTER`. A
    /// session pings one time for each time that it goes idle. A ping brings
    /// the list to view with high alert, and also while no tray icon shows
    /// the ping.
    pub fn refresh(
        &mut self,
        now: Instant,
        clock: SystemTime,
        has_tray: bool,
    ) -> Result<Refreshed> {
        let muted_minutes = self
            .queue
            .pings_muted_until()
            .and_then(|until| queue_folder::muted_minutes(until, clock));
        let waiting = self.queue.waiting_ids()?;
        self.kinds.retain(|id, _| waiting.contains(id));
        for id in &waiting {
            if !self.kinds.contains_key(id) {
                let kind = self
                    .queue
                    .tool_of(id)
                    .map_or(EntryKind::Permission, |tool| EntryKind::of_tool(&tool));
                self.kinds.insert(id.clone(), kind);
            }
        }
        let epoch = clock
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_secs());
        let idle: Vec<_> = self
            .queue
            .sessions()?
            .into_iter()
            .filter(|listed| {
                listed.state.activity == Activity::Idle
                    && epoch.saturating_sub(listed.state.since) >= IDLE_PING_AFTER.as_secs()
            })
            .collect();
        let pinging = idle
            .iter()
            .map(|listed| format!("session-{}-{}", listed.session, listed.state.since));
        self.watch.refresh(
            waiting.into_iter().chain(pinging).collect(),
            now,
            muted_minutes.is_some(),
        );
        let pinged = self.pings.try_iter().reduce(|sum, count| sum + count);
        if pinged.is_some() {
            self.flash_until = Some(now + FLASH_TIME);
        }
        Ok(Refreshed {
            view: TrayView {
                permissions: self.count(EntryKind::Permission),
                questions: self.count(EntryKind::Question),
                idle_sessions: idle.len(),
                is_flashing: self.flash_left(now).is_some(),
                muted_minutes,
                settings: self.settings,
            },
            pinged,
            shows_list: pinged.is_some() && (self.settings.high_alert || !has_tray),
        })
    }

    /// The time for which the icon of the last ping still shows at `now`.
    #[must_use]
    pub fn flash_left(&self, now: Instant) -> Option<Duration> {
        self.flash_until?
            .checked_duration_since(now)
            .filter(|left| !left.is_zero())
    }

    fn count(&self, kind: EntryKind) -> usize {
        self.kinds.values().filter(|found| **found == kind).count()
    }
}
