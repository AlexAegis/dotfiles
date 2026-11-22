use std::fs;

use approval_queue::{
    alert_settings::{AlertSettings, CadencePreset},
    ping_watch::CADENCE,
};
use tempfile::TempDir;

#[test]
fn should_load_the_defaults_when_no_settings_are_stored() {
    let folder = TempDir::new().unwrap();
    let settings = AlertSettings::load(&folder.path().join("settings.json"));
    assert_eq!(
        settings,
        AlertSettings {
            high_alert: false,
            cadence: CadencePreset::Normal,
        }
    );
}

#[test]
fn should_load_the_settings_that_it_stored() {
    let folder = TempDir::new().unwrap();
    let path = folder.path().join("settings.json");
    let settings = AlertSettings {
        high_alert: true,
        cadence: CadencePreset::Seldom,
    };
    settings.store(&path).unwrap();
    assert_eq!(AlertSettings::load(&path), settings);
    assert_eq!(fs::read_dir(folder.path()).unwrap().count(), 1);
}

#[test]
fn should_load_the_defaults_for_settings_of_another_form() {
    let folder = TempDir::new().unwrap();
    let path = folder.path().join("settings.json");
    fs::write(&path, r#"{"cadence":"hourly"}"#).unwrap();
    assert_eq!(AlertSettings::load(&path), AlertSettings::default());
}

#[test]
fn should_load_the_default_of_a_setting_that_the_file_does_not_have() {
    let folder = TempDir::new().unwrap();
    let path = folder.path().join("settings.json");
    fs::write(&path, r#"{"cadence":"often"}"#).unwrap();
    assert_eq!(
        AlertSettings::load(&path),
        AlertSettings {
            high_alert: false,
            cadence: CadencePreset::Often,
        }
    );
}

#[test]
fn should_report_settings_that_it_cannot_store() {
    let folder = TempDir::new().unwrap();
    let path = folder.path().join("missing/settings.json");
    assert!(AlertSettings::default().store(&path).is_err());
}

#[test]
fn should_have_the_cadence_values_that_the_label_of_each_preset_names() {
    let values = |preset: CadencePreset| {
        let cadence = preset.cadence();
        (
            cadence.quiet_time.as_secs(),
            cadence.batch_size,
            cadence.longest_wait.as_secs(),
            preset.label(),
        )
    };
    assert_eq!(
        CadencePreset::ALL.map(values),
        [
            (10, 2, 30, "Often: 10 s quiet, then 2 prompts or 30 s"),
            (30, 4, 120, "Normal: 30 s quiet, then 4 prompts or 2 min"),
            (120, 8, 600, "Seldom: 2 min quiet, then 8 prompts or 10 min"),
        ]
    );
    assert_eq!(CadencePreset::Normal.cadence(), CADENCE);
}
