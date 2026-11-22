use std::{
    fs,
    time::{Duration, Instant, SystemTime},
};

use approval_queue::{
    alert_settings::{AlertSettings, CadencePreset},
    alerts::{Alerts, FLASH_TIME, TrayCommand},
    entry::Entry,
    queue_folder::QueueFolder,
};
use serde_json::Map;
use tempfile::TempDir;

struct Fixture {
    folder: TempDir,
    queue: QueueFolder,
    alerts: Alerts,
    start: Instant,
    clock: SystemTime,
}

impl Fixture {
    fn with_settings(settings: &str) -> Self {
        let folder = TempDir::new().unwrap();
        let queue = QueueFolder::in_runtime_folder(folder.path());
        queue.create().unwrap();
        fs::write(folder.path().join("settings.json"), settings).unwrap();
        let alerts = Alerts::new(
            QueueFolder::in_runtime_folder(folder.path()),
            folder.path().join("settings.json"),
        );
        Self {
            folder,
            queue,
            alerts,
            start: Instant::now(),
            clock: SystemTime::now(),
        }
    }

    fn stored(&self) -> AlertSettings {
        AlertSettings::load(&self.folder.path().join("settings.json"))
    }
}

fn seconds(count: u64) -> Duration {
    Duration::from_secs(count)
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
        input: Map::new(),
        reason: None,
        list_only: false,
    }
}

#[test]
fn should_count_the_waiting_prompts_by_their_kind() {
    let mut fixture = Fixture::with_settings("{}");
    let _question = fixture.queue.add(&entry("AskUserQuestion")).unwrap();
    let permission = fixture.queue.add(&entry("Bash")).unwrap();
    let view = fixture
        .alerts
        .refresh(fixture.start, fixture.clock, true)
        .unwrap()
        .view;
    assert_eq!((view.permissions, view.questions), (1, 1));
    drop(permission);
    let view = fixture
        .alerts
        .refresh(fixture.start + seconds(1), fixture.clock, true)
        .unwrap()
        .view;
    assert_eq!((view.permissions, view.questions), (0, 1));
}

#[test]
fn should_count_an_entry_of_another_form_as_a_permission_prompt() {
    let mut fixture = Fixture::with_settings("{}");
    let waiting = fixture.queue.add(&entry("AskUserQuestion")).unwrap();
    let path = fixture
        .folder
        .path()
        .join(format!("approval-queue/entries/{}.json", waiting.id()));
    fs::write(path, "[").unwrap();
    let view = fixture
        .alerts
        .refresh(fixture.start, fixture.clock, true)
        .unwrap()
        .view;
    assert_eq!((view.permissions, view.questions), (1, 0));
}

#[test]
fn should_flash_for_the_flash_time_after_a_ping() {
    let mut fixture = Fixture::with_settings("{}");
    let _waiting = fixture.queue.add(&entry("Bash")).unwrap();
    let refreshed = fixture
        .alerts
        .refresh(fixture.start, fixture.clock, true)
        .unwrap();
    assert_eq!(refreshed.pinged_prompts, Some(1));
    assert!(refreshed.view.is_flashing);
    assert_eq!(FLASH_TIME, Duration::from_millis(400));
    assert_eq!(fixture.alerts.flash_left(fixture.start), Some(FLASH_TIME));
    let end = fixture.start + FLASH_TIME;
    assert_eq!(fixture.alerts.flash_left(end), None);
    let refreshed = fixture.alerts.refresh(end, fixture.clock, true).unwrap();
    assert_eq!(refreshed.pinged_prompts, None);
    assert!(!refreshed.view.is_flashing);
}

#[test]
fn should_bring_the_list_to_view_at_a_ping_only_with_high_alert_or_without_a_tray() {
    for (settings, has_tray, shows_list) in [
        ("{}", true, false),
        ("{}", false, true),
        (r#"{"high_alert":true}"#, true, true),
        (r#"{"high_alert":true}"#, false, true),
    ] {
        let mut fixture = Fixture::with_settings(settings);
        let refreshed = fixture
            .alerts
            .refresh(fixture.start, fixture.clock, has_tray)
            .unwrap();
        assert!(!refreshed.shows_list);
        let _waiting = fixture.queue.add(&entry("Bash")).unwrap();
        let refreshed = fixture
            .alerts
            .refresh(fixture.start + seconds(1), fixture.clock, has_tray)
            .unwrap();
        assert_eq!(refreshed.shows_list, shows_list);
        let refreshed = fixture
            .alerts
            .refresh(fixture.start + seconds(2), fixture.clock, has_tray)
            .unwrap();
        assert!(!refreshed.shows_list);
    }
}

#[test]
fn should_not_ping_for_a_new_cadence_alone() {
    let mut fixture = Fixture::with_settings("{}");
    let _first = fixture.queue.add(&entry("Bash")).unwrap();
    let start = fixture.start;
    let clock = fixture.clock;
    assert_eq!(
        fixture
            .alerts
            .refresh(start, clock, true)
            .unwrap()
            .pinged_prompts,
        Some(1)
    );
    let _second = fixture.queue.add(&entry("Bash")).unwrap();
    fixture
        .alerts
        .refresh(start + seconds(5), clock, true)
        .unwrap();
    assert!(
        !fixture
            .alerts
            .carry_out(TrayCommand::SetCadence(CadencePreset::Seldom), clock)
            .unwrap()
    );
    assert_eq!(fixture.stored().cadence, CadencePreset::Seldom);
    for time in [6, 125, 604] {
        let refreshed = fixture
            .alerts
            .refresh(start + seconds(time), clock, true)
            .unwrap();
        assert_eq!(refreshed.pinged_prompts, None);
        assert_eq!(refreshed.view.settings.cadence, CadencePreset::Seldom);
    }
    assert_eq!(
        fixture
            .alerts
            .refresh(start + seconds(605), clock, true)
            .unwrap()
            .pinged_prompts,
        Some(1)
    );
}

#[test]
fn should_ping_at_the_stored_cadence_from_the_start() {
    let mut fixture = Fixture::with_settings(r#"{"cadence":"often"}"#);
    let (start, clock) = (fixture.start, fixture.clock);
    let _first = fixture.queue.add(&entry("Bash")).unwrap();
    fixture.alerts.refresh(start, clock, true).unwrap();
    let _second = fixture.queue.add(&entry("Bash")).unwrap();
    let _third = fixture.queue.add(&entry("Bash")).unwrap();
    let pinged = |alerts: &mut Alerts, time| {
        alerts
            .refresh(start + seconds(time), clock, true)
            .unwrap()
            .pinged_prompts
    };
    assert_eq!(pinged(&mut fixture.alerts, 1), None);
    assert_eq!(pinged(&mut fixture.alerts, 9), None);
    assert_eq!(pinged(&mut fixture.alerts, 10), Some(2));
}

#[test]
fn should_store_the_high_alert_at_each_toggle() {
    let mut fixture = Fixture::with_settings("{}");
    let clock = fixture.clock;
    fixture
        .alerts
        .carry_out(TrayCommand::ToggleHighAlert, clock)
        .unwrap();
    assert!(fixture.stored().high_alert);
    assert!(fixture.alerts.settings().high_alert);
    fixture
        .alerts
        .carry_out(TrayCommand::ToggleHighAlert, clock)
        .unwrap();
    assert!(!fixture.stored().high_alert);
}

#[test]
fn should_keep_a_setting_that_it_cannot_store_and_report_it() {
    let mut fixture = Fixture::with_settings("{}");
    let clock = fixture.clock;
    let settings = fixture.folder.path().join("settings.json");
    fs::remove_file(&settings).unwrap();
    fs::create_dir(&settings).unwrap();
    assert!(
        fixture
            .alerts
            .carry_out(TrayCommand::ToggleHighAlert, clock)
            .is_err()
    );
    assert!(fixture.alerts.settings().high_alert);
}

#[test]
fn should_mute_and_unmute_the_pings_and_show_the_minutes_left() {
    let mut fixture = Fixture::with_settings("{}");
    let clock = fixture.clock;
    let start = fixture.start;
    fixture
        .alerts
        .carry_out(TrayCommand::Mute(seconds(600)), clock)
        .unwrap();
    let _waiting = fixture.queue.add(&entry("Bash")).unwrap();
    let refreshed = fixture
        .alerts
        .refresh(start, clock + seconds(60), true)
        .unwrap();
    assert_eq!(refreshed.view.muted_minutes, Some(9));
    assert_eq!(refreshed.pinged_prompts, None);
    assert_eq!(refreshed.view.permissions, 1);
    fixture
        .alerts
        .carry_out(TrayCommand::Unmute, clock)
        .unwrap();
    let refreshed = fixture
        .alerts
        .refresh(start + seconds(1), clock + seconds(61), true)
        .unwrap();
    assert_eq!(refreshed.view.muted_minutes, None);
    assert_eq!(refreshed.pinged_prompts, Some(1));
}

#[test]
fn should_ask_for_the_list_only_at_the_open_command() {
    let mut fixture = Fixture::with_settings("{}");
    let clock = fixture.clock;
    assert!(
        fixture
            .alerts
            .carry_out(TrayCommand::OpenList, clock)
            .unwrap()
    );
    assert!(
        !fixture
            .alerts
            .carry_out(TrayCommand::Unmute, clock)
            .unwrap()
    );
}
