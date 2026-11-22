use std::{
    fs::{self, File},
    os::unix::fs::PermissionsExt,
    path::Path,
    time::{Duration, SystemTime},
};

use approval_queue::{answer::Answer, entry::Entry, queue_folder::QueueFolder};
use serde_json::{Map, json};
use tempfile::TempDir;

fn entry(tool: &str) -> Entry {
    let mut input = Map::new();
    input.insert("command".to_owned(), json!("ls"));
    Entry {
        received: 1_795_000_000,
        user: "agent".to_owned(),
        user_is_verified: false,
        host: "claude-code".to_owned(),
        session: "0a1b".to_owned(),
        folder: "/home/agent/git/pont".to_owned(),
        mode: "auto".to_owned(),
        tool: tool.to_owned(),
        input,
        reason: None,
        list_only: false,
    }
}

fn queue() -> (TempDir, QueueFolder) {
    let runtime_folder = TempDir::new().unwrap();
    let queue = QueueFolder::in_runtime_folder(runtime_folder.path());
    queue.create().unwrap();
    (runtime_folder, queue)
}

fn entries(runtime_folder: &TempDir) -> std::path::PathBuf {
    runtime_folder.path().join("approval-queue/entries")
}

fn answers(runtime_folder: &TempDir) -> std::path::PathBuf {
    runtime_folder.path().join("approval-queue/answers")
}

fn lists(runtime_folder: &TempDir) -> std::path::PathBuf {
    runtime_folder.path().join("approval-queue/lists")
}

fn mode(path: &Path) -> u32 {
    fs::metadata(path).unwrap().permissions().mode() & 0o777
}

fn names(folder: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .unwrap()
        .map(|item| item.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

#[test]
fn should_create_folders_and_entry_files_for_their_owner_only() {
    let (runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    assert_eq!(mode(&entries(&runtime_folder)), 0o700);
    assert_eq!(mode(&answers(&runtime_folder)), 0o700);
    assert_eq!(
        mode(&entries(&runtime_folder).join(format!("{}.json", waiting.id()))),
        0o600
    );
}

#[test]
fn should_list_a_waiting_entry_with_its_id() {
    let (_runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    let listed = queue.waiting().unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].id, waiting.id());
    assert_eq!(listed[0].entry, entry("Bash"));
}

#[test]
fn should_list_the_oldest_entry_first() {
    let (_runtime_folder, queue) = queue();
    let _first = queue.add(&entry("Bash")).unwrap();
    let _second = queue.add(&entry("Write")).unwrap();
    let _third = queue.add(&entry("Edit")).unwrap();
    let tools: Vec<String> = queue
        .waiting()
        .unwrap()
        .into_iter()
        .map(|listed| listed.entry.tool)
        .collect();
    assert_eq!(tools, ["Bash", "Write", "Edit"]);
}

#[test]
fn should_remove_the_entry_when_its_process_drops_it() {
    let (runtime_folder, queue) = queue();
    drop(queue.add(&entry("Bash")).unwrap());
    assert_eq!(queue.waiting().unwrap(), []);
    assert_eq!(names(&entries(&runtime_folder)), [] as [&str; 0]);
}

#[test]
fn should_remove_an_entry_that_no_process_locks() {
    let (runtime_folder, queue) = queue();
    let leftover = entries(&runtime_folder).join("00000000001795000000000000000-41.json");
    fs::write(&leftover, serde_json::to_vec(&entry("Bash")).unwrap()).unwrap();
    assert_eq!(queue.waiting().unwrap(), []);
    assert!(!leftover.exists());
}

#[test]
fn should_keep_a_locked_file_of_another_form_out_of_the_list() {
    let (runtime_folder, queue) = queue();
    for (name, content) in [
        ("00000000001795000000000000000-41.json", "not json"),
        ("../outside.json", "{}"),
        ("name.json", "{}"),
        ("00000000001795000000000000000-42.txt", "{}"),
    ] {
        let path = entries(&runtime_folder).join(name);
        fs::write(&path, content).unwrap();
        let file = File::open(&path).unwrap();
        file.lock().unwrap();
        assert_eq!(queue.waiting().unwrap(), [], "{name}");
        assert!(path.exists(), "{name}");
    }
}

#[test]
fn should_remove_an_old_temporary_file_and_keep_a_new_one() {
    let (runtime_folder, queue) = queue();
    let new = entries(&runtime_folder).join(".new.tmp");
    let old = entries(&runtime_folder).join(".old.tmp");
    fs::write(&new, "").unwrap();
    fs::write(&old, "").unwrap();
    File::options()
        .write(true)
        .open(&old)
        .unwrap()
        .set_modified(SystemTime::now() - Duration::from_secs(61))
        .unwrap();
    assert_eq!(queue.waiting().unwrap(), []);
    assert_eq!(names(&entries(&runtime_folder)), [".new.tmp"]);
}

#[test]
fn should_hand_an_answer_to_the_service_and_remove_it_with_its_entry() {
    let (runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    assert_eq!(queue.answer_for(waiting.id()).unwrap(), None);
    let denial = Answer::Deny {
        message: Some("Use a branch".to_owned()),
    };
    queue.answer(waiting.id(), &denial).unwrap();
    assert_eq!(
        mode(&answers(&runtime_folder).join(format!("{}.json", waiting.id()))),
        0o600
    );
    assert_eq!(queue.answer_for(waiting.id()).unwrap(), Some(denial));
    drop(waiting);
    assert_eq!(names(&answers(&runtime_folder)), [] as [&str; 0]);
    assert_eq!(names(&entries(&runtime_folder)), [] as [&str; 0]);
}

#[test]
fn should_refuse_an_answer_after_the_service_read_one() {
    let (_runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    let denial = Answer::Deny { message: None };
    queue.answer(waiting.id(), &denial).unwrap();
    assert_eq!(queue.answer_for(waiting.id()).unwrap(), Some(denial));
    assert_eq!(
        queue
            .answer(waiting.id(), &Answer::Allow)
            .unwrap_err()
            .to_string(),
        "Prompt has its answer"
    );
}

#[test]
fn should_refuse_a_second_answer_and_keep_the_first() {
    let (runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    queue.answer(waiting.id(), &Answer::Allow).unwrap();
    assert_eq!(
        queue
            .answer(waiting.id(), &Answer::Deny { message: None })
            .unwrap_err()
            .to_string(),
        "Prompt has its answer"
    );
    assert_eq!(
        names(&answers(&runtime_folder)),
        [format!("{}.json", waiting.id())]
    );
    assert_eq!(queue.answer_for(waiting.id()).unwrap(), Some(Answer::Allow));
}

#[test]
fn should_refuse_an_answer_for_an_entry_that_left() {
    let (runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    let id = waiting.id().to_owned();
    drop(waiting);
    assert_eq!(
        queue.answer(&id, &Answer::Allow).unwrap_err().to_string(),
        "Prompt left the queue"
    );
    assert_eq!(names(&answers(&runtime_folder)), [] as [&str; 0]);
}

#[test]
fn should_refuse_an_answer_for_an_id_of_another_form() {
    let (runtime_folder, queue) = queue();
    fs::write(
        runtime_folder.path().join("approval-queue/outside.json"),
        "{}",
    )
    .unwrap();
    for id in ["", "../outside", "a", "1/2"] {
        assert_eq!(
            queue.answer(id, &Answer::Allow).unwrap_err().to_string(),
            format!("Entry ID {id:?} has another form")
        );
    }
    assert_eq!(names(&answers(&runtime_folder)), [] as [&str; 0]);
}

#[test]
fn should_refuse_a_message_that_does_not_fit_one_line() {
    let (_runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    for (message, reason) in [
        (
            "two\nlines".to_owned(),
            "Message contains a control character",
        ),
        ("a".repeat(501), "Message longer than 500 characters"),
    ] {
        let denial = Answer::Deny {
            message: Some(message),
        };
        assert_eq!(
            queue.answer(waiting.id(), &denial).unwrap_err().to_string(),
            reason
        );
    }
    let longest = Answer::Deny {
        message: Some("a".repeat(500)),
    };
    queue.answer(waiting.id(), &longest).unwrap();
}

#[test]
fn should_refuse_an_answer_file_of_another_form() {
    let (runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    let path = answers(&runtime_folder).join(format!("{}.json", waiting.id()));
    for (content, reason) in [
        ("yes", "Answer has another form"),
        (r#"{"behavior":"ask"}"#, "Answer has another form"),
        (r#"{"behavior":"deny","more":1}"#, "Answer has another form"),
        (
            r#"{"behavior":"deny","message":"two\nlines"}"#,
            "Message contains a control character",
        ),
    ] {
        fs::write(&path, content).unwrap();
        assert_eq!(
            queue.answer_for(waiting.id()).unwrap_err().to_string(),
            reason,
            "{content}"
        );
    }
    drop(waiting);
    assert!(!path.exists());
}

#[test]
fn should_remove_only_the_answers_whose_entry_left() {
    let (runtime_folder, queue) = queue();
    let stays = queue.add(&entry("Bash")).unwrap();
    let leaves = queue.add(&entry("Write")).unwrap();
    queue.answer(stays.id(), &Answer::Allow).unwrap();
    queue.answer(leaves.id(), &Answer::Allow).unwrap();
    drop(leaves);
    queue.remove_answers_without_entry().unwrap();
    assert_eq!(
        names(&answers(&runtime_folder)),
        [format!("{}.json", stays.id())]
    );
}

#[test]
fn should_keep_a_new_temporary_answer_and_remove_an_old_one() {
    let (runtime_folder, queue) = queue();
    let waiting = queue.add(&entry("Bash")).unwrap();
    let new = format!(".{}.41.tmp", waiting.id());
    let old = format!(".{}.42.tmp", waiting.id());
    fs::write(answers(&runtime_folder).join(&new), "").unwrap();
    fs::write(answers(&runtime_folder).join(&old), "").unwrap();
    File::options()
        .write(true)
        .open(answers(&runtime_folder).join(&old))
        .unwrap()
        .set_modified(SystemTime::now() - Duration::from_secs(61))
        .unwrap();
    queue.remove_answers_without_entry().unwrap();
    assert_eq!(names(&answers(&runtime_folder)), [new]);
}

#[test]
fn should_see_an_open_list_until_it_closes() {
    let (runtime_folder, queue) = queue();
    assert!(!queue.has_open_list().unwrap());
    let first = queue.open_list().unwrap();
    let second = queue.open_list().unwrap();
    assert_eq!(mode(&lists(&runtime_folder)), 0o700);
    assert_eq!(names(&lists(&runtime_folder)).len(), 2);
    assert!(queue.has_open_list().unwrap());
    drop(first);
    assert!(queue.has_open_list().unwrap());
    drop(second);
    assert!(!queue.has_open_list().unwrap());
    assert_eq!(names(&lists(&runtime_folder)), [] as [&str; 0]);
}

#[test]
fn should_remove_the_mark_of_a_list_that_no_process_locks() {
    let (runtime_folder, queue) = queue();
    fs::write(lists(&runtime_folder).join("1-1"), "").unwrap();
    fs::write(lists(&runtime_folder).join(".2-1.tmp"), "").unwrap();
    assert!(!queue.has_open_list().unwrap());
    assert_eq!(names(&lists(&runtime_folder)), [".2-1.tmp"]);
}

#[test]
fn should_keep_the_entries_apart_from_the_marks_of_the_lists() {
    let (_runtime_folder, queue) = queue();
    let _open_list = queue.open_list().unwrap();
    assert_eq!(queue.waiting().unwrap(), []);
    let _waiting = queue.add(&entry("Bash")).unwrap();
    assert_eq!(queue.waiting().unwrap().len(), 1);
    assert!(queue.has_open_list().unwrap());
}

#[test]
fn should_list_the_ids_of_the_waiting_entries_without_their_content() {
    let (runtime_folder, queue) = queue();
    let first = queue.add(&entry("Bash")).unwrap();
    let second = queue.add(&entry("Write")).unwrap();
    for path in fs::read_dir(entries(&runtime_folder)).unwrap() {
        fs::write(path.unwrap().path(), "[").unwrap();
    }
    File::create(entries(&runtime_folder).join("00000000000000000001-1.json")).unwrap();
    assert_eq!(
        queue.waiting_ids().unwrap(),
        vec![first.id().to_owned(), second.id().to_owned()]
    );
    assert_eq!(fs::read_dir(entries(&runtime_folder)).unwrap().count(), 2);
}
