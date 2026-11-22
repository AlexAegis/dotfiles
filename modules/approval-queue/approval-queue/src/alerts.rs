//! The waiting prompts with their pings and the settings of the owner.

use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant, SystemTime},
};

use anyhow::Result;

use crate::{
    alert_settings::{AlertSettings, CadencePreset},
    ping_watch::PingWatch,
    queue_folder::{self, QueueFolder},
    tray_view::{EntryKind, TrayView},
};

/// How long the icon of a ping shows before the icon of what waits.
pub const FLASH_TIME: Duration = Duration::from_millis(400);

/// What the owner chose in the tray.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayCommand {
    OpenList,
    ToggleHighAlert,
    Mute(Duration),
    Unmute,
    SetCadence(CadencePreset),
}

/// The result of one read of the queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Refreshed {
    /// What the tray shows.
    pub view: TrayView,
    /// The count of the prompts of a ping that came.
    pub pinged_prompts: Option<usize>,
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

    /// Reads the queue and pings when the cadence allows it. A ping brings
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
        self.watch.refresh(waiting, now, muted_minutes.is_some());
        let pinged_prompts = self.pings.try_iter().reduce(|sum, count| sum + count);
        if pinged_prompts.is_some() {
            self.flash_until = Some(now + FLASH_TIME);
        }
        Ok(Refreshed {
            view: TrayView {
                permissions: self.count(EntryKind::Permission),
                questions: self.count(EntryKind::Question),
                is_flashing: self.flash_left(now).is_some(),
                muted_minutes,
                settings: self.settings,
            },
            pinged_prompts,
            shows_list: pinged_prompts.is_some() && (self.settings.high_alert || !has_tray),
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
