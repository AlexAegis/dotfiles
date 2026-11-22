use std::fs;

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
    ];
    for view in views {
        assert!(ICONS.iter().any(|(name, _)| *name == view.icon_name()));
    }
}
