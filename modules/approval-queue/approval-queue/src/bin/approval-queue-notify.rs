use std::{
    env,
    fs::DirBuilder,
    os::unix::fs::DirBuilderExt,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, RecvTimeoutError, Sender},
    },
    thread,
    time::{Duration, Instant, SystemTime},
};

use anyhow::{Context, Result};
use approval_queue::{
    alert_settings::CadencePreset,
    alerts::{Alerts, TrayCommand},
    list_window,
    queue_folder::QueueFolder,
    tray_icons,
    tray_view::TrayView,
};
use ksni::{
    MenuItem, OfflineReason, Status, ToolTip, Tray,
    blocking::TrayMethods,
    menu::{CheckmarkItem, RadioGroup, RadioItem, StandardItem, SubMenu},
};

const REFRESH_INTERVAL: Duration = Duration::from_millis(500);
/// Long enough for the terminal of a ping to open its window.
const SCRIPT_KEEP_TIME: Duration = Duration::from_secs(10);
const SHORT_MUTE: Duration = Duration::from_mins(10);
const LONG_MUTE: Duration = Duration::from_hours(1);

struct QueueTray {
    view: TrayView,
    icon_theme_path: String,
    commands: Sender<TrayCommand>,
    /// False while no desktop service takes tray icons.
    has_host: Arc<AtomicBool>,
}

impl QueueTray {
    fn send(&self, command: TrayCommand) {
        let _ = self.commands.send(command);
    }

    fn item(label: &str, command: TrayCommand) -> MenuItem<Self> {
        StandardItem {
            label: label.to_owned(),
            activate: Box::new(move |tray: &mut Self| tray.send(command)),
            ..Default::default()
        }
        .into()
    }
}

impl Tray for QueueTray {
    fn id(&self) -> String {
        "approval-queue".to_owned()
    }

    fn title(&self) -> String {
        "Approval queue".to_owned()
    }

    fn icon_theme_path(&self) -> String {
        self.icon_theme_path.clone()
    }

    fn watcher_online(&self) {
        self.has_host.store(true, Ordering::Relaxed);
    }

    fn watcher_offline(&self, reason: OfflineReason) -> bool {
        eprintln!("No desktop service takes the tray icon: {reason:?}");
        self.has_host.store(false, Ordering::Relaxed);
        true
    }

    fn icon_name(&self) -> String {
        self.view.icon_name().to_owned()
    }

    fn attention_icon_name(&self) -> String {
        self.icon_name()
    }

    fn status(&self) -> Status {
        if self.view.needs_attention() {
            Status::NeedsAttention
        } else {
            Status::Active
        }
    }

    fn tool_tip(&self) -> ToolTip {
        ToolTip {
            title: "Approval queue".to_owned(),
            description: self.view.summary(),
            ..Default::default()
        }
    }

    fn activate(&mut self, _x: i32, _y: i32) {
        self.send(TrayCommand::OpenList);
    }

    fn menu(&self) -> Vec<MenuItem<Self>> {
        let presets = CadencePreset::ALL;
        vec![
            Self::item("Open the list", TrayCommand::OpenList),
            MenuItem::Separator,
            CheckmarkItem {
                label: "High alert: a ping shows the list".to_owned(),
                checked: self.view.settings.high_alert,
                activate: Box::new(|tray: &mut Self| tray.send(TrayCommand::ToggleHighAlert)),
                ..Default::default()
            }
            .into(),
            SubMenu {
                label: "Mute the pings".to_owned(),
                submenu: vec![
                    Self::item("For 10 minutes", TrayCommand::Mute(SHORT_MUTE)),
                    Self::item("For 1 hour", TrayCommand::Mute(LONG_MUTE)),
                    StandardItem {
                        label: "Unmute".to_owned(),
                        enabled: self.view.muted_minutes.is_some(),
                        activate: Box::new(|tray: &mut Self| tray.send(TrayCommand::Unmute)),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            SubMenu {
                label: "Cadence of the pings".to_owned(),
                submenu: vec![
                    RadioGroup {
                        selected: presets
                            .iter()
                            .position(|preset| *preset == self.view.settings.cadence)
                            .unwrap_or_default(),
                        select: Box::new(move |tray: &mut Self, index| {
                            if let Some(preset) = presets.get(index) {
                                tray.send(TrayCommand::SetCadence(*preset));
                            }
                        }),
                        options: presets
                            .iter()
                            .map(|preset| RadioItem {
                                label: preset.label().to_owned(),
                                ..Default::default()
                            })
                            .collect(),
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
        ]
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let runtime_folder =
        PathBuf::from(env::var_os("XDG_RUNTIME_DIR").context("XDG_RUNTIME_DIR is not set")?);
    let settings_path = state_folder()
        .unwrap_or_else(|error| {
            let folder = runtime_folder.join("approval-queue");
            eprintln!(
                "{error:#}. Settings stay in {} until the user logs out.",
                folder.display()
            );
            folder
        })
        .join("settings.json");
    let list_program = env::current_exe()
        .context("Own path unknown")?
        .with_file_name("approval-queue");
    let queue = QueueFolder::in_runtime_folder(&runtime_folder);
    queue.create()?;
    let mut alerts = Alerts::new(queue, settings_path);
    let (command_sender, commands) = mpsc::channel();
    let has_host = Arc::new(AtomicBool::new(true));
    // Without a tray icon, each ping brings the list to view
    let handle = tray_icons::write(&runtime_folder)
        .and_then(|icon_theme_path| {
            let tray = QueueTray {
                view: TrayView {
                    settings: alerts.settings(),
                    ..TrayView::default()
                },
                icon_theme_path: icon_theme_path.to_string_lossy().into_owned(),
                commands: command_sender,
                has_host: Arc::clone(&has_host),
            };
            Ok(tray.assume_sni_available(true).spawn()?)
        })
        .inspect_err(|error| eprintln!("Tray icon not shown: {error:#}"))
        .ok();
    let mut shown = None;
    let mut forget_script_at = None;
    let mut command = None;
    loop {
        let now = Instant::now();
        let mut shows_list = false;
        for command in command.take().into_iter().chain(commands.try_iter()) {
            match alerts.carry_out(command, SystemTime::now()) {
                Ok(asks_for_list) => shows_list |= asks_for_list,
                Err(error) => eprintln!("{error:#}"),
            }
        }
        let has_tray = handle.as_ref().is_some_and(|handle| !handle.is_closed())
            && has_host.load(Ordering::Relaxed);
        match alerts.refresh(now, SystemTime::now(), has_tray) {
            Ok(refreshed) => {
                if let Some(count) = refreshed.pinged_prompts {
                    eprintln!("Ping for {count} new prompts");
                }
                shows_list |= refreshed.shows_list;
                if shown != Some(refreshed.view)
                    && let Some(handle) = &handle
                {
                    handle.update(|tray| tray.view = refreshed.view);
                    shown = Some(refreshed.view);
                }
            }
            Err(error) => eprintln!("Queue unreadable: {error:#}"),
        }
        if shows_list {
            if let Err(error) = show_list(alerts.queue(), &runtime_folder, &list_program) {
                eprintln!("List not shown: {error:#}");
            }
            forget_script_at = Some(now + SCRIPT_KEEP_TIME);
        }
        if forget_script_at.is_some_and(|at| now >= at) {
            let _ = list_window::forget_script();
            forget_script_at = None;
        }
        let wait = alerts
            .flash_left(Instant::now())
            .map_or(REFRESH_INTERVAL, |left| left.min(REFRESH_INTERVAL));
        // A command of the tray ends the wait. Without a tray, no command
        // comes and the sender is gone.
        command = match commands.recv_timeout(wait) {
            Ok(command) => Some(command),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => {
                thread::sleep(wait);
                None
            }
        };
    }
}

/// The folder of the settings: the state folder that systemd gives the unit,
/// else the state folder of the user.
fn state_folder() -> Result<PathBuf> {
    if let Some(folder) = env::var_os("STATE_DIRECTORY").filter(|folder| !folder.is_empty()) {
        return Ok(PathBuf::from(folder));
    }
    let folder = env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .filter(|folder| folder.is_absolute())
        .map_or_else(
            || {
                env::var_os("HOME")
                    .map(PathBuf::from)
                    .filter(|home| home.is_absolute())
                    .map(|home| home.join(".local/state"))
                    .context("No state folder: HOME is not an absolute path")
            },
            Ok,
        )?
        .join("approval-queue");
    DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&folder)
        .with_context(|| format!("Folder {} not created", folder.display()))?;
    Ok(folder)
}

/// Brings the window of the list to view. Opens a terminal with the list
/// first when no list is open.
fn show_list(queue: &QueueFolder, runtime_folder: &Path, list_program: &Path) -> Result<()> {
    if let Err(error) = list_window::bring_to_view(runtime_folder) {
        eprintln!("Window of the list not focused: {error:#}");
    }
    if !queue.has_open_list()? {
        list_window::open_terminal(list_program)?;
    }
    Ok(())
}
