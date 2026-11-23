use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex, OnceLock},
    thread,
    time::{Duration, Instant, SystemTime},
};

use approval_queue::{
    alert_settings::{AlertSettings, CadencePreset},
    entry::Entry,
    queue_folder::QueueFolder,
};
use serde_json::{Map, Value};
use tempfile::TempDir;

const ITEM: &str = "org.kde.StatusNotifierItem";
const MENU: &str = "com.canonical.dbusmenu";

const WATCHER: &str = "org.kde.StatusNotifierWatcher";

struct Watcher {
    items: Arc<Mutex<Vec<String>>>,
}

#[zbus::interface(name = "org.kde.StatusNotifierWatcher")]
impl Watcher {
    fn register_status_notifier_item(&self, service: &str) {
        self.items.lock().unwrap().push(service.to_owned());
    }
}

struct Tray {
    notify: Child,
    bus: Child,
    folder: TempDir,
    watcher: Option<zbus::blocking::Connection>,
    items: Arc<Mutex<Vec<String>>>,
    name: OnceLock<String>,
}

impl Drop for Tray {
    fn drop(&mut self) {
        for process in [&mut self.notify, &mut self.bus] {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}

fn until<T>(mut found: impl FnMut() -> Option<T>) -> T {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(value) = found() {
            return value;
        }
        assert!(Instant::now() < deadline, "Waited 20 seconds");
        thread::sleep(Duration::from_millis(5));
    }
}

fn write_program(path: &Path, body: &str) {
    fs::write(path, format!("#!/bin/sh\n{body}")).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

fn entry(tool: &str) -> Entry {
    Entry {
        received: 1_795_000_000,
        user: "agent".to_owned(),
        user_is_verified: false,
        host: "claude-code".to_owned(),
        session: "0a1b".to_owned(),
        folder: "/home/agent/git/pont".to_owned(),
        mode: "auto".to_owned(),
        tool: tool.to_owned(),
        title: None,
        risk: None,
        input: Map::new(),
        reason: None,
        list_only: false,
    }
}

fn properties_of_label(layout: &Value, label: &str) -> Option<Value> {
    match layout {
        Value::Array(items) => {
            if let [Value::Number(_), Value::Object(properties), Value::Array(_)] = &items[..]
                && properties.get("label").and_then(|text| text.get("data")) == Some(&label.into())
            {
                return Some(Value::Object(properties.clone()));
            }
            items
                .iter()
                .find_map(|item| properties_of_label(item, label))
        }
        Value::Object(fields) => fields
            .values()
            .find_map(|field| properties_of_label(field, label)),
        _ => None,
    }
}

fn id_of_label(layout: &Value, label: &str) -> Option<i64> {
    match layout {
        Value::Array(items) => {
            if let [
                Value::Number(id),
                Value::Object(properties),
                Value::Array(_),
            ] = &items[..]
                && properties.get("label").and_then(|text| text.get("data")) == Some(&label.into())
            {
                return id.as_i64();
            }
            items.iter().find_map(|item| id_of_label(item, label))
        }
        Value::Object(fields) => fields.values().find_map(|field| id_of_label(field, label)),
        _ => None,
    }
}

fn start_bus(folder: &Path) -> Child {
    let socket = folder.join("bus");
    let bus = Command::new("dbus-daemon")
        .args(["--session", "--nofork", "--nopidfile"])
        .arg(format!("--address=unix:path={}", socket.display()))
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !socket.exists() {
        assert!(
            Instant::now() < deadline,
            "dbus-daemon made no socket. These tests need unix sockets."
        );
        thread::sleep(Duration::from_millis(10));
    }
    bus
}

fn connect_watcher(folder: &Path, items: &Arc<Mutex<Vec<String>>>) -> zbus::blocking::Connection {
    let address = format!("unix:path={}", folder.join("bus").display());
    zbus::blocking::connection::Builder::address(address.as_str())
        .unwrap()
        .name(WATCHER)
        .unwrap()
        .serve_at(
            "/StatusNotifierWatcher",
            Watcher {
                items: Arc::clone(items),
            },
        )
        .unwrap()
        .build()
        .unwrap()
}

fn start_notify(folder: &Path, state: &[(&str, &str)]) -> Child {
    let programs = folder.join("programs");
    fs::create_dir(&programs).unwrap();
    for program in ["busctl", "systemd-run", "systemctl"] {
        write_program(
            &programs.join(program),
            &format!("echo \"{program} $*\" >>\"$CALLS\"\necho 'i 3'\n"),
        );
    }
    fs::create_dir(folder.join("state")).unwrap();
    Command::new(env!("CARGO_BIN_EXE_approval-queue-notify"))
        .current_dir(folder)
        .env_clear()
        .env("XDG_RUNTIME_DIR", folder)
        .envs(state.iter().copied())
        .env(
            "DBUS_SESSION_BUS_ADDRESS",
            format!("unix:path={}", folder.join("bus").display()),
        )
        .env("PATH", &programs)
        .env("CALLS", folder.join("calls"))
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap()
}

impl Tray {
    fn start() -> Self {
        let folder = TempDir::new().unwrap();
        let bus = start_bus(folder.path());
        let items = Arc::default();
        let watcher = connect_watcher(folder.path(), &items);
        let state = folder.path().join("state");
        let notify = start_notify(
            folder.path(),
            &[("STATE_DIRECTORY", state.to_str().unwrap())],
        );
        let tray = Self {
            notify,
            bus,
            folder,
            watcher: Some(watcher),
            items,
            name: OnceLock::new(),
        };
        tray.wait_for_registration();
        tray
    }

    fn start_without_watcher(folder: TempDir) -> Self {
        let state = folder.path().join("state");
        let state = state.to_str().unwrap().to_owned();
        Self::start_with_state(folder, &[("STATE_DIRECTORY", &state)])
    }

    fn start_with_state(folder: TempDir, state: &[(&str, &str)]) -> Self {
        let bus = start_bus(folder.path());
        let notify = start_notify(folder.path(), state);
        Self {
            notify,
            bus,
            folder,
            watcher: None,
            items: Arc::default(),
            name: OnceLock::new(),
        }
    }

    fn start_watcher(&mut self) {
        self.watcher = Some(connect_watcher(self.folder.path(), &self.items));
        self.wait_for_registration();
    }

    fn wait_for_registration(&self) {
        until(|| (*self.items.lock().unwrap() == [self.name()]).then_some(()));
    }

    fn busctl(&self, arguments: &[&str]) -> Option<String> {
        let output = Command::new("busctl")
            .arg(format!(
                "--address=unix:path={}",
                self.folder.path().join("bus").display()
            ))
            .args(arguments)
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .unwrap();
        output
            .status
            .success()
            .then(|| String::from_utf8(output.stdout).unwrap())
    }

    fn item_names(&self) -> Option<Vec<String>> {
        Some(
            self.busctl(&["list", "--no-legend"])?
                .lines()
                .filter_map(|line| line.split_whitespace().next())
                .filter(|name| name.starts_with("org.kde.StatusNotifierItem-"))
                .map(str::to_owned)
                .collect(),
        )
    }

    fn name(&self) -> String {
        self.name
            .get_or_init(|| until(|| self.item_names()?.pop()))
            .clone()
    }

    fn menu_item(&self, label: &str) -> Option<Value> {
        let name = self.name();
        let layout = self.busctl(&[
            "--json=short",
            "call",
            "--",
            &name,
            "/MenuBar",
            MENU,
            "GetLayout",
            "iias",
            "0",
            "-1",
            "0",
        ])?;
        properties_of_label(&serde_json::from_str(&layout).unwrap(), label)
    }

    fn wait_for_menu_item(&self, label: &str, property: &str, value: &Value) {
        until(|| (self.menu_item(label)?[property]["data"] == *value).then_some(()));
    }

    fn property(&self, property: &str) -> Value {
        let name = self.name();
        let reply = until(|| {
            self.busctl(&[
                "--json=short",
                "get-property",
                &name,
                "/StatusNotifierItem",
                ITEM,
                property,
            ])
        });
        serde_json::from_str::<Value>(&reply).unwrap()["data"].take()
    }

    fn wait_for_icon(&self, icon: &str) {
        until(|| (self.property("IconName") == icon).then_some(()));
    }

    fn click(&self, label: &str) {
        let name = self.name();
        let id = until(|| {
            let layout = self.busctl(&[
                "--json=short",
                "call",
                "--",
                &name,
                "/MenuBar",
                MENU,
                "GetLayout",
                "iias",
                "0",
                "-1",
                "0",
            ])?;
            id_of_label(&serde_json::from_str(&layout).unwrap(), label)
        });
        self.busctl(&[
            "call",
            &name,
            "/MenuBar",
            MENU,
            "Event",
            "isvu",
            &id.to_string(),
            "clicked",
            "s",
            "",
            "0",
        ])
        .expect("busctl sends the click");
    }

    fn queue(&self) -> QueueFolder {
        let queue = QueueFolder::in_runtime_folder(self.folder.path());
        queue.create().unwrap();
        queue
    }

    fn settings_path(&self) -> PathBuf {
        self.folder.path().join("state/settings.json")
    }

    fn calls(&self) -> Vec<String> {
        fs::read_to_string(self.folder.path().join("calls"))
            .unwrap_or_default()
            .lines()
            .map(str::to_owned)
            .collect()
    }
}

#[test]
fn should_show_the_icon_of_what_waits() {
    let tray = Tray::start();
    let queue = tray.queue();
    tray.wait_for_icon("approval-queue-empty");
    assert_eq!(
        tray.property("IconThemePath"),
        tray.folder
            .path()
            .join("approval-queue/icons")
            .to_str()
            .unwrap()
    );
    assert_eq!(tray.property("Status"), "Active");
    let question = queue.add(&entry("AskUserQuestion")).unwrap();
    tray.wait_for_icon("approval-queue-question");
    assert_eq!(tray.property("Status"), "NeedsAttention");
    assert_eq!(
        tray.property("AttentionIconName"),
        "approval-queue-question"
    );
    let permission = queue.add(&entry("Bash")).unwrap();
    tray.wait_for_icon("approval-queue-permission");
    drop(permission);
    tray.wait_for_icon("approval-queue-question");
    drop(question);
    tray.wait_for_icon("approval-queue-empty");
    assert_eq!(tray.property("Status"), "Active");
    assert_eq!(tray.calls(), Vec::<String>::new());
}

#[test]
fn should_flash_at_a_ping_before_it_shows_what_waits() {
    let tray = Tray::start();
    let queue = tray.queue();
    tray.wait_for_icon("approval-queue-empty");
    let _waiting = queue.add(&entry("Bash")).unwrap();
    tray.wait_for_icon("approval-queue-flash");
    tray.wait_for_icon("approval-queue-permission");
}

#[test]
fn should_mute_and_unmute_the_pings_from_the_menu() {
    let tray = Tray::start();
    let queue = tray.queue();
    tray.click("For 1 hour");
    let until_time = until(|| queue.pings_muted_until());
    let left = until_time.duration_since(SystemTime::now()).unwrap();
    assert!(left > Duration::from_mins(59) && left <= Duration::from_hours(1));
    tray.click("For 10 minutes");
    until(|| {
        let left = queue
            .pings_muted_until()?
            .duration_since(SystemTime::now())
            .ok()?;
        (left <= Duration::from_mins(10)).then_some(())
    });
    tray.click("Unmute");
    until(|| queue.pings_muted_until().is_none().then_some(()));
}

#[test]
fn should_store_the_high_alert_of_the_menu_and_then_show_the_list_at_a_ping() {
    let tray = Tray::start();
    let queue = tray.queue();
    tray.click(HIGH_ALERT);
    until(|| {
        AlertSettings::load(&tray.settings_path())
            .high_alert
            .then_some(())
    });
    assert_eq!(tray.calls(), Vec::<String>::new());
    let _waiting = queue.add(&entry("Bash")).unwrap();
    let calls = until(|| Some(tray.calls()).filter(|calls| calls.len() >= 4));
    assert!(calls[1].contains(" loadScript "));
    assert!(calls[3].starts_with("systemd-run "));
}

#[test]
fn should_store_the_cadence_preset_of_the_menu() {
    let tray = Tray::start();
    tray.click(SELDOM);
    until(|| {
        (AlertSettings::load(&tray.settings_path()).cadence == CadencePreset::Seldom).then_some(())
    });
}

#[test]
fn should_show_the_list_at_a_click_on_the_icon_and_at_the_menu_item() {
    let tray = Tray::start();
    let name = tray.name();
    tray.busctl(&[
        "call",
        &name,
        "/StatusNotifierItem",
        ITEM,
        "Activate",
        "ii",
        "0",
        "0",
    ])
    .unwrap();
    let calls = until(|| Some(tray.calls()).filter(|calls| calls.len() >= 4));
    assert!(calls[3].starts_with("systemd-run "));
    tray.click("Open the list");
    until(|| Some(tray.calls()).filter(|calls| calls.len() >= 8));
}

#[test]
fn should_stop_the_units_of_the_queue_at_the_quit_item_of_the_menu() {
    let tray = Tray::start();
    tray.click("Quit: stop the queue");
    let calls = until(|| Some(tray.calls()).filter(|calls| !calls.is_empty()));
    assert_eq!(
        calls,
        [
            "systemctl --user --no-block stop approval-queue.socket approval-queue@*.service approval-queue-notify.service"
        ]
    );
}

const HIGH_ALERT: &str = "High alert: a ping shows the list";
const SELDOM: &str = "Seldom: 2 min quiet, then 8 prompts or 10 min";
const NORMAL: &str = "Normal: 30 s quiet, then 4 prompts or 2 min";

#[test]
fn should_show_in_the_menu_what_is_set() {
    let tray = Tray::start();
    let (off, on) = (Value::from(0), Value::from(1));
    tray.wait_for_menu_item(HIGH_ALERT, "toggle-state", &off);
    tray.wait_for_menu_item(NORMAL, "toggle-state", &on);
    tray.wait_for_menu_item(SELDOM, "toggle-state", &off);
    tray.wait_for_menu_item("Unmute", "enabled", &Value::from(false));
    tray.click(HIGH_ALERT);
    tray.wait_for_menu_item(HIGH_ALERT, "toggle-state", &on);
    tray.click(SELDOM);
    tray.wait_for_menu_item(SELDOM, "toggle-state", &on);
    tray.wait_for_menu_item(NORMAL, "toggle-state", &off);
    tray.click("For 10 minutes");
    tray.wait_for_menu_item("Unmute", "enabled", &Value::Null);
}

#[test]
fn should_bring_the_list_to_view_at_a_ping_while_no_service_takes_the_icon() {
    let tray = Tray::start_without_watcher(TempDir::new().unwrap());
    let queue = tray.queue();
    let _waiting = queue.add(&entry("Bash")).unwrap();
    let calls = until(|| Some(tray.calls()).filter(|calls| calls.len() >= 4));
    assert!(calls[3].starts_with("systemd-run "));
}

#[test]
fn should_only_show_the_icon_at_a_ping_after_a_service_came_that_takes_it() {
    let mut tray = Tray::start_without_watcher(TempDir::new().unwrap());
    let queue = tray.queue();
    tray.wait_for_icon("approval-queue-empty");
    tray.start_watcher();
    let _waiting = queue.add(&entry("Bash")).unwrap();
    tray.wait_for_icon("approval-queue-permission");
    thread::sleep(Duration::from_millis(600));
    assert_eq!(tray.calls(), Vec::<String>::new());
}

#[test]
fn should_bring_the_list_to_view_at_a_ping_when_it_cannot_write_the_icons() {
    let folder = TempDir::new().unwrap();
    fs::create_dir(folder.path().join("approval-queue")).unwrap();
    fs::write(folder.path().join("approval-queue/icons"), "").unwrap();
    let tray = Tray::start_without_watcher(folder);
    let queue = tray.queue();
    let _waiting = queue.add(&entry("Bash")).unwrap();
    let calls = until(|| Some(tray.calls()).filter(|calls| calls.len() >= 4));
    assert!(calls[3].starts_with("systemd-run "));
    assert_eq!(tray.item_names(), Some(Vec::new()));
}

#[test]
fn should_bring_the_list_to_view_at_a_ping_after_the_service_that_takes_the_icon_left() {
    let mut tray = Tray::start();
    let queue = tray.queue();
    tray.wait_for_icon("approval-queue-empty");
    tray.watcher = None;
    until(|| tray.busctl(&["status", WATCHER]).is_none().then_some(()));
    thread::sleep(Duration::from_millis(600));
    let _waiting = queue.add(&entry("Bash")).unwrap();
    let calls = until(|| Some(tray.calls()).filter(|calls| calls.len() >= 4));
    assert!(calls[3].starts_with("systemd-run "));
}

#[test]
fn should_keep_the_settings_in_the_folder_of_the_queue_without_a_state_folder() {
    let mut tray = Tray::start_with_state(TempDir::new().unwrap(), &[("HOME", "")]);
    tray.start_watcher();
    tray.click(HIGH_ALERT);
    let settings = tray.folder.path().join("approval-queue/settings.json");
    until(|| AlertSettings::load(&settings).high_alert.then_some(()));
    let left: Vec<_> = fs::read_dir(tray.folder.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| name.starts_with('.'))
        .collect();
    assert_eq!(left, Vec::<String>::new());
}
