use std::path::Path;

use approval_queue::list_window::{FOCUS_SCRIPT, TITLE, is_loaded, terminal_command};

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
            "--title=approval-queue",
            "-e",
            "/data/approval-queue/bin/approval-queue",
        ]
    );
    assert!(command.contains(&format!("--title={TITLE}").into()));
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
