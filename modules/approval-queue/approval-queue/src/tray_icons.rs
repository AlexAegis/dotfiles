//! The icons of the tray, as an icon theme folder that the program writes.

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

/// Each icon: its name in the theme and its SVG text.
pub const ICONS: [(&str, &str); 4] = [
    (
        "approval-queue-empty",
        include_str!("icons/approval-queue-empty.svg"),
    ),
    (
        "approval-queue-flash",
        include_str!("icons/approval-queue-flash.svg"),
    ),
    (
        "approval-queue-permission",
        include_str!("icons/approval-queue-permission.svg"),
    ),
    (
        "approval-queue-question",
        include_str!("icons/approval-queue-question.svg"),
    ),
];

/// Writes the icons below the folder of the queue and returns the folder
/// that the tray gives the desktop as its icon theme path.
pub fn write(runtime_folder: &Path) -> Result<PathBuf> {
    let theme_path = runtime_folder.join("approval-queue").join("icons");
    let folder = theme_path.join("hicolor/scalable/status");
    fs::create_dir_all(&folder)
        .with_context(|| format!("Folder {} not created", folder.display()))?;
    for (name, text) in ICONS {
        let path = folder.join(format!("{name}.svg"));
        fs::write(&path, text).with_context(|| format!("Icon {} not written", path.display()))?;
    }
    Ok(theme_path)
}
