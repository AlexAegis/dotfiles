use std::{fs, path::Path};

use approval_queue::{
    alert_settings::AlertSettings,
    tray_icons::{self, ICONS},
    tray_view::TrayView,
};
use tempfile::TempDir;

#[test]
fn should_write_each_icon_as_an_svg_file_of_the_theme_folder() {
    let runtime_folder = TempDir::new().unwrap();
    let theme_path = tray_icons::write(runtime_folder.path()).unwrap();
    assert_eq!(
        theme_path,
        runtime_folder.path().join("approval-queue/icons")
    );
    for (name, text) in ICONS {
        let path = theme_path.join(format!("hicolor/scalable/status/{name}.svg"));
        assert_eq!(fs::read_to_string(path).unwrap(), text);
        assert!(text.starts_with("<svg "));
    }
}

#[test]
fn should_have_an_icon_for_each_name_that_the_tray_shows() {
    let views = [
        TrayView::default(),
        TrayView {
            questions: 1,
            ..TrayView::default()
        },
        TrayView {
            permissions: 1,
            ..TrayView::default()
        },
        TrayView {
            is_flashing: true,
            settings: AlertSettings::default(),
            ..TrayView::default()
        },
        TrayView {
            idle_sessions: 1,
            ..TrayView::default()
        },
    ];
    for view in views {
        assert!(ICONS.iter().any(|(name, _)| *name == view.icon_name()));
    }
}

fn icon_names_below(folder: &Path, names: &mut Vec<String>) {
    for entry in fs::read_dir(folder).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            icon_names_below(&entry.path(), names);
        } else if let Some(stem) = entry.path().file_stem() {
            names.push(stem.to_string_lossy().into_owned());
        }
    }
}

fn names_with_cut_ends(name: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = name;
    loop {
        names.push(rest.to_owned());
        names.push(format!("{rest}-symbolic"));
        match rest.rfind('-').filter(|index| *index > 1) {
            Some(index) => rest = &rest[..index],
            None => return names,
        }
    }
}

#[test]
fn should_install_no_icon_that_the_desktop_finds_for_a_tray_name_with_a_cut_end() {
    let mut installed = Vec::new();
    icon_names_below(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../XDG_DATA_HOME.approval-queue/icons"),
        &mut installed,
    );
    assert_ne!(installed, Vec::<String>::new());
    for (name, _) in ICONS {
        for found in names_with_cut_ends(name) {
            assert!(!installed.contains(&found), "{found}");
        }
    }
}

#[test]
fn should_count_each_name_cut_at_a_dash_also_with_the_symbolic_end() {
    assert_eq!(
        names_with_cut_ends("approval-queue-flash"),
        [
            "approval-queue-flash",
            "approval-queue-flash-symbolic",
            "approval-queue",
            "approval-queue-symbolic",
            "approval",
            "approval-symbolic",
        ]
    );
}
