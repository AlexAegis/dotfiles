use std::path::Path;

use approval_queue::list_window::{
    APPLICATION_ID, FOCUS_SCRIPT, TITLE, is_loaded, terminal_command,
};
use approval_queue::tray_icons::ICONS;

#[test]
fn should_open_the_list_in_a_terminal_unit_with_the_title_of_the_list() {
    let command = terminal_command(Path::new("/data/approval-queue/bin/approval-queue"));
    assert_eq!(
        command,
        [
            "systemd-run",
            "--user",
            "--collect",
            "--quiet",
            "--description=List of the waiting permission prompts",
            "ghostty",
            "--class=io.github.alexaegis.approval-queue",
            "--title=approval-queue",
            "-e",
            "/data/approval-queue/bin/approval-queue",
        ]
    );
    assert!(command.contains(&format!("--title={TITLE}").into()));
    assert!(command.contains(&format!("--class={APPLICATION_ID}").into()));
}

#[test]
fn should_name_the_window_of_the_list_and_its_icon_in_the_desktop_file() {
    let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("../XDG_DATA_HOME.approval-queue");
    let entry = std::fs::read_to_string(
        data.join("applications")
            .join(format!("{APPLICATION_ID}.desktop")),
    )
    .unwrap();
    assert!(entry.contains("\nIcon=approval-queue\n"));
    assert!(entry.contains(&format!("\nStartupWMClass={APPLICATION_ID}\n")));
    assert!(entry.contains(&format!(
        "\nExec=ghostty --class={APPLICATION_ID} --title={TITLE} -e "
    )));
}

#[test]
fn should_draw_the_window_icon_as_the_empty_queue_in_fixed_colors() {
    let icon = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../XDG_DATA_HOME.approval-queue/icons/hicolor/scalable/apps/approval-queue.svg"),
    )
    .unwrap();
    let empty = ICONS
        .iter()
        .find(|(name, _)| *name == "approval-queue-empty")
        .unwrap()
        .1;
    let shapes: Vec<&str> = empty
        .split(" d=\"")
        .skip(1)
        .map(|rest| rest.split('"').next().unwrap())
        .collect();
    assert_eq!(shapes.len(), 2);
    for shape in shapes {
        assert!(icon.contains(&format!(" d=\"{shape}\"")));
    }
    assert_eq!(icon.matches("<path ").count(), 2);
    assert!(!icon.contains("currentColor"));
}

#[test]
fn should_search_the_window_by_a_caption_that_is_the_title_of_the_list() {
    assert!(FOCUS_SCRIPT.contains(&format!("var listCaption = /^{TITLE}( <[0-9]+>)?$/;")));
}

#[test]
fn should_read_a_load_reply_with_an_id_as_loaded() {
    assert!(is_loaded("i 0\n").unwrap());
    assert!(is_loaded("i 7\n").unwrap());
}

#[test]
fn should_read_a_load_reply_with_a_negative_id_as_not_loaded() {
    assert!(!is_loaded("i -1\n").unwrap());
}

#[test]
fn should_refuse_a_load_reply_of_another_form() {
    assert!(is_loaded("s \"x\"\n").is_err());
}
