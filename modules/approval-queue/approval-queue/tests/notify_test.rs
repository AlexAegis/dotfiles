use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime},
};

use approval_queue::{entry::Entry, queue_folder::QueueFolder};
use serde_json::Map;
use tempfile::TempDir;

struct Notify {
    process: Child,
    folder: TempDir,
}

impl Drop for Notify {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

impl Notify {
    fn start(busctl_reply: &str) -> Self {
        Self::start_with_settings(busctl_reply, r#"{"high_alert":true}"#)
    }

    fn start_with_settings(busctl_reply: &str, settings: &str) -> Self {
        Self::start_in(TempDir::new().unwrap(), busctl_reply, settings)
    }

    fn start_in(folder: TempDir, busctl_reply: &str, settings: &str) -> Self {
        let state = folder.path().join("state");
        fs::create_dir(&state).unwrap();
        fs::write(state.join("settings.json"), settings).unwrap();
        let state = state.to_str().unwrap().to_owned();
        Self::start_with(folder, busctl_reply, &[("STATE_DIRECTORY", &state)])
    }

    fn start_with_state(folder: TempDir, state: &[(&str, &str)]) -> Self {
        Self::start_with(folder, "i 3", state)
    }

    fn start_with(folder: TempDir, busctl_reply: &str, state: &[(&str, &str)]) -> Self {
        let programs = folder.path().join("programs");
        fs::create_dir(&programs).unwrap();
        write_program(
            &programs.join("busctl"),
            &format!(
                "echo \"busctl $*\" >>\"$CALLS\"\ncase \"$*\" in *loadScript*) echo '{busctl_reply}' ;; esac\n"
            ),
        );
        write_program(
            &programs.join("systemd-run"),
            "echo \"systemd-run $*\" >>\"$CALLS\"\n",
        );
        let process = Command::new(env!("CARGO_BIN_EXE_approval-queue-notify"))
            .current_dir(folder.path())
            .env_clear()
            .env("XDG_RUNTIME_DIR", folder.path())
            .envs(state.iter().copied())
            .env("PATH", &programs)
            .env("CALLS", folder.path().join("calls"))
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        Self { process, folder }
    }

    fn queue(&self) -> QueueFolder {
        let queue = QueueFolder::in_runtime_folder(self.folder.path());
        queue.create().unwrap();
        queue
    }

    fn calls_after(&self, line_count: usize) -> Vec<String> {
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            let calls = fs::read_to_string(self.folder.path().join("calls")).unwrap_or_default();
            let lines: Vec<String> = calls.lines().map(str::to_owned).collect();
            if lines.len() >= line_count || Instant::now() > deadline {
                return lines;
            }
            thread::sleep(Duration::from_millis(20));
        }
    }
}

fn write_program(path: &Path, body: &str) {
    fs::write(path, format!("#!/bin/sh\n{body}")).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

fn entry() -> Entry {
    Entry {
        received: 1_795_000_000,
        user: "agent".to_owned(),
        user_is_verified: false,
        host: "claude-code".to_owned(),
        session: "0a1b".to_owned(),
        folder: "/home/agent/git/pont".to_owned(),
        mode: "auto".to_owned(),
        tool: "Bash".to_owned(),
        input: Map::new(),
        reason: None,
        list_only: false,
    }
}

#[test]
fn should_load_the_focus_script_and_open_a_terminal_for_the_first_prompt() {
    let notify = Notify::start("i 3");
    let queue = notify.queue();
    let _waiting = queue.add(&entry()).unwrap();
    let script = notify.folder.path().join("approval-queue/focus-list.js");
    let list_program =
        Path::new(env!("CARGO_BIN_EXE_approval-queue-notify")).with_file_name("approval-queue");
    let scripting = "busctl --user --timeout=2 call org.kde.KWin /Scripting org.kde.kwin.Scripting";
    assert_eq!(
        notify.calls_after(4),
        [
            format!("{scripting} unloadScript s approval-queue-focus-list"),
            format!(
                "{scripting} loadScript ss {} approval-queue-focus-list",
                script.display()
            ),
            format!("{scripting} start"),
            format!(
                "systemd-run --user --collect --quiet --description=List of the waiting permission prompts ghostty --title=approval-queue -e {}",
                list_program.display()
            ),
        ]
    );
    assert!(
        fs::read_to_string(script)
            .unwrap()
            .contains("approval-queue")
    );
}

#[test]
fn should_open_no_terminal_while_a_list_is_open() {
    let notify = Notify::start("i 3");
    let queue = notify.queue();
    let _open_list = queue.open_list().unwrap();
    let _waiting = queue.add(&entry()).unwrap();
    assert_eq!(notify.calls_after(3).len(), 3);
    thread::sleep(Duration::from_millis(300));
    let calls = notify.calls_after(0);
    assert_eq!(calls.len(), 3);
    assert!(calls.iter().all(|call| call.starts_with("busctl ")));
}

#[test]
fn should_open_the_terminal_when_kwin_loads_no_script() {
    let notify = Notify::start("i -1");
    let queue = notify.queue();
    let _waiting = queue.add(&entry()).unwrap();
    let calls = notify.calls_after(3);
    assert_eq!(calls.len(), 3);
    assert!(calls[1].contains(" loadScript "));
    assert!(calls[2].starts_with("systemd-run "));
}

#[test]
fn should_not_ping_while_no_prompt_waits() {
    let mut notify = Notify::start("i 3");
    let _queue = notify.queue();
    thread::sleep(Duration::from_millis(1200));
    assert_eq!(notify.calls_after(0), Vec::<String>::new());
    assert!(notify.process.try_wait().unwrap().is_none());
}

#[test]
fn should_unload_the_focus_script_some_time_after_the_ping() {
    let notify = Notify::start("i 3");
    let queue = notify.queue();
    let _open_list = queue.open_list().unwrap();
    let _waiting = queue.add(&entry()).unwrap();
    let calls = notify.calls_after(4);
    assert_eq!(calls.len(), 4);
    assert!(calls[3].contains(" unloadScript "));
}

#[test]
fn should_ping_for_an_entry_whose_content_is_no_request() {
    let notify = Notify::start("i 3");
    let queue = notify.queue();
    let _waiting = queue.add(&entry()).unwrap();
    let entries = notify.folder.path().join("approval-queue/entries");
    let path = fs::read_dir(&entries)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(path, "[").unwrap();
    assert_eq!(notify.calls_after(4).len(), 4);
}

#[test]
fn should_not_ping_while_the_pings_are_muted_and_ping_after_the_unmute() {
    let notify = Notify::start("i 3");
    let queue = notify.queue();
    queue
        .mute_pings_until(SystemTime::now() + Duration::from_secs(600))
        .unwrap();
    let _waiting = queue.add(&entry()).unwrap();
    thread::sleep(Duration::from_millis(1500));
    assert_eq!(notify.calls_after(0), Vec::<String>::new());
    queue.unmute_pings().unwrap();
    assert_eq!(notify.calls_after(4).len(), 4);
}

#[test]
fn should_ping_when_the_time_of_the_mute_is_over() {
    let notify = Notify::start("i 3");
    let queue = notify.queue();
    queue
        .mute_pings_until(SystemTime::now() - Duration::from_secs(1))
        .unwrap();
    let _waiting = queue.add(&entry()).unwrap();
    assert_eq!(notify.calls_after(4).len(), 4);
}

#[test]
fn should_bring_the_list_to_view_at_a_ping_without_high_alert_when_it_has_no_tray() {
    let notify = Notify::start_with_settings("i 3", "{}");
    let queue = notify.queue();
    let _waiting = queue.add(&entry()).unwrap();
    assert_eq!(notify.calls_after(4).len(), 4);
}

fn folder_mode(path: &Path) -> u32 {
    fs::metadata(path).unwrap().permissions().mode() & 0o777
}

#[test]
fn should_make_the_state_folder_of_the_user_for_the_owner_only() {
    let folder = TempDir::new().unwrap();
    let state_home = folder.path().join("state-home");
    let notify = Notify::start_with_state(
        folder,
        &[
            ("STATE_DIRECTORY", ""),
            ("XDG_STATE_HOME", state_home.to_str().unwrap()),
        ],
    );
    let queue = notify.queue();
    let _waiting = queue.add(&entry()).unwrap();
    assert_eq!(notify.calls_after(4).len(), 4);
    assert_eq!(folder_mode(&state_home.join("approval-queue")), 0o700);
}

#[test]
fn should_take_the_state_folder_below_the_home_folder_for_a_relative_state_home() {
    let folder = TempDir::new().unwrap();
    let home = folder.path().join("home");
    let notify = Notify::start_with_state(
        folder,
        &[
            ("XDG_STATE_HOME", "relative"),
            ("HOME", home.to_str().unwrap()),
        ],
    );
    let queue = notify.queue();
    let _waiting = queue.add(&entry()).unwrap();
    assert_eq!(notify.calls_after(4).len(), 4);
    assert_eq!(
        folder_mode(&home.join(".local/state/approval-queue")),
        0o700
    );
}

#[test]
fn should_ping_without_a_state_folder() {
    let notify = Notify::start_with_state(TempDir::new().unwrap(), &[("HOME", "")]);
    let queue = notify.queue();
    let _waiting = queue.add(&entry()).unwrap();
    assert_eq!(notify.calls_after(4).len(), 4);
}
