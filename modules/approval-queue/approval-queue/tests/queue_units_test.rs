use std::{
    fs,
    os::unix::net::UnixListener,
    path::{Path, PathBuf},
};

use approval_queue::queue_units::{REQUEST_SOCKET, START_COMMAND, UNITS, is_stopped_at};

fn module_file(path: &str) -> String {
    fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(path)).unwrap()
}

fn unit(name: &str) -> String {
    module_file(&format!(
        "XDG_CONFIG_HOME.approval-queue/systemd/user/{name}"
    ))
}

fn socket_path(folder: &tempfile::TempDir) -> PathBuf {
    folder.path().join("request.sock")
}

#[test]
fn should_count_the_queue_as_stopped_while_no_socket_is_at_the_path() {
    let folder = tempfile::tempdir().unwrap();
    let socket = socket_path(&folder);
    assert!(is_stopped_at(&socket));
    let listener = UnixListener::bind(&socket).unwrap();
    assert!(!is_stopped_at(&socket));
    drop(listener);
    fs::remove_file(&socket).unwrap();
    assert!(is_stopped_at(&socket));
}

#[test]
fn should_count_a_regular_file_at_the_path_as_stopped() {
    let folder = tempfile::tempdir().unwrap();
    let file = socket_path(&folder);
    fs::write(&file, "").unwrap();
    assert!(is_stopped_at(&file));
}

#[test]
fn should_look_for_the_socket_where_the_socket_unit_listens() {
    let socket = unit("approval-queue.socket");
    assert!(
        socket
            .lines()
            .any(|line| line == format!("ListenStream={REQUEST_SOCKET}"))
    );
    assert!(socket.lines().any(|line| line == "RemoveOnStop=yes"));
}

#[test]
fn should_start_the_socket_and_the_tray_program_with_the_command_of_the_readme() {
    assert_eq!(
        START_COMMAND,
        format!("systemctl --user start {} {}", UNITS[0], UNITS[2])
    );
    assert!(
        module_file("readme.md")
            .lines()
            .any(|line| line == START_COMMAND)
    );
}
