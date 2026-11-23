use std::{
    ffi::OsStr,
    fs::{self, DirBuilder, File, OpenOptions, TryLockError},
    io::{self, BufReader, ErrorKind, Write},
    os::unix::fs::{DirBuilderExt, OpenOptionsExt},
    path::{Path, PathBuf},
    process,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail, ensure};

use crate::{
    answer::Answer,
    entry::Entry,
    listed_entry::ListedEntry,
    open_list::OpenList,
    process_tree::process_folder,
    request::{Risk, check_title},
    session_state::{
        Change, ListedSession, SessionReport, SessionState, is_session_id, latest_sent_at,
    },
    waiting_entry::WaitingEntry,
};

/// The most sessions of one user that have a state.
pub const MAX_SESSIONS_OF_USER: usize = 64;

/// A temporary file that no process locks is a leftover after this age.
const LEFTOVER_AGE: Duration = Duration::from_secs(60);

/// The folders through which the service processes and the lists share the
/// waiting prompts: one file for each entry, one for each answer, one for
/// each later title, one for each open list and one for each session that
/// reports its state.
#[derive(Debug, Clone)]
pub struct QueueFolder {
    entries: PathBuf,
    answers: PathBuf,
    titles: PathBuf,
    risks: PathBuf,
    lists: PathBuf,
    sessions: PathBuf,
    mute: PathBuf,
}

impl QueueFolder {
    /// The queue of a user, in the runtime folder of that user.
    #[must_use]
    pub fn in_runtime_folder(runtime_folder: &Path) -> Self {
        let root = runtime_folder.join("approval-queue");
        Self {
            entries: root.join("entries"),
            answers: root.join("answers"),
            titles: root.join("titles"),
            risks: root.join("risks"),
            lists: root.join("lists"),
            sessions: root.join("sessions"),
            mute: root.join("pings-muted-until"),
        }
    }

    /// Creates the folders, for their owner only.
    pub fn create(&self) -> Result<()> {
        for folder in [
            &self.entries,
            &self.answers,
            &self.titles,
            &self.risks,
            &self.lists,
            &self.sessions,
        ] {
            DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(folder)
                .with_context(|| format!("Folder {} not created", folder.display()))?;
        }
        Ok(())
    }

    /// Stores the entry of the calling service process. The entry leaves the
    /// queue when the returned value drops, or when the process ends.
    pub fn add(&self, entry: &Entry) -> Result<WaitingEntry> {
        let id = new_id()?;
        let path = self.entries.join(format!("{id}.json"));
        let lock = store_locked(&self.entries, &id, &path, &serde_json::to_vec(entry)?)
            .with_context(|| format!("Entry {} not stored", path.display()))?;
        Ok(WaitingEntry {
            answer_path: self.answers.join(format!("{id}.json")),
            title_path: self.titles.join(&id),
            risk_path: self.risks.join(&id),
            id,
            path,
            _lock: lock,
        })
    }

    /// Marks a list as open until the returned value drops, or until the
    /// process of the list ends.
    pub fn open_list(&self) -> Result<OpenList> {
        let id = new_id()?;
        let path = self.lists.join(&id);
        let lock = store_locked(&self.lists, &id, &path, &[])
            .with_context(|| format!("Mark {} of the list not stored", path.display()))?;
        Ok(OpenList { path, _lock: lock })
    }

    /// True while a list is open. Removes the mark of each list whose process
    /// ended without removing it.
    pub fn has_open_list(&self) -> Result<bool> {
        Ok(files_of(&self.lists)?
            .iter()
            .filter_map(|path| locked(path))
            .count()
            > 0)
    }

    /// The waiting entries, oldest first. Removes each entry whose service
    /// process ended without removing it.
    pub fn waiting(&self) -> Result<Vec<ListedEntry>> {
        let mut listed = Vec::new();
        for path in files_of(&self.entries)? {
            let Some(file) = locked(&path) else {
                continue;
            };
            let id = entry_id(&path);
            let entry: serde_json::Result<Entry> = serde_json::from_reader(BufReader::new(&file));
            if let (Some(id), Ok(mut entry)) = (id, entry) {
                if let Some(title) = self.later_title(id) {
                    entry.title = Some(title);
                }
                entry.risk = self.risk_of(id);
                listed.push(ListedEntry {
                    id: id.to_owned(),
                    entry,
                });
            }
        }
        listed.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(listed)
    }

    /// The IDs of the waiting entries, oldest first, without a read of their
    /// content. Removes each entry whose service process ended without
    /// removing it.
    pub fn waiting_ids(&self) -> Result<Vec<String>> {
        let mut ids: Vec<String> = files_of(&self.entries)?
            .iter()
            .filter(|path| locked(path).is_some())
            .filter_map(|path| entry_id(path).map(str::to_owned))
            .collect();
        ids.sort();
        Ok(ids)
    }

    /// The tool of a waiting entry. Keeps no other field. `None` when the
    /// entry left or has another form.
    #[must_use]
    pub fn tool_of(&self, id: &str) -> Option<String> {
        #[derive(serde::Deserialize)]
        struct Tool {
            tool: String,
        }
        if !is_id(id) {
            return None;
        }
        let file = File::open(self.entries.join(format!("{id}.json"))).ok()?;
        let entry: Tool = serde_json::from_reader(BufReader::new(file)).ok()?;
        Some(entry.tool)
    }

    /// Mutes the pings until the time. A later call replaces the time.
    pub fn mute_pings_until(&self, until: SystemTime) -> Result<()> {
        let seconds = until
            .duration_since(UNIX_EPOCH)
            .context("Time is before 1970")?
            .as_secs();
        let temporary = self.mute.with_extension(format!("{}.tmp", process::id()));
        let stored = fs::write(&temporary, seconds.to_string())
            .and_then(|()| fs::rename(&temporary, &self.mute));
        if stored.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        stored.with_context(|| format!("Mute {} not stored", self.mute.display()))
    }

    pub fn unmute_pings(&self) -> Result<()> {
        match fs::remove_file(&self.mute) {
            Err(error) if error.kind() != ErrorKind::NotFound => {
                Err(error).with_context(|| format!("Mute {} not removed", self.mute.display()))
            }
            _ => Ok(()),
        }
    }

    /// The time until which the pings are muted. `None` without a mute, and
    /// for a stored time that is not a count of seconds.
    #[must_use]
    pub fn pings_muted_until(&self) -> Option<SystemTime> {
        let seconds = fs::read_to_string(&self.mute).ok()?.trim().parse().ok()?;
        UNIX_EPOCH.checked_add(Duration::from_secs(seconds))
    }

    /// Stores a later title of the waiting entry, in place of an earlier
    /// one. The lists show it in place of the title of the request.
    pub fn set_title(&self, id: &str, title: &str) -> Result<()> {
        ensure!(is_id(id), "Entry ID {id:?} has another form");
        check_title(title)?;
        let temporary = self.titles.join(format!(".{id}.{}.tmp", process::id()));
        let path = self.titles.join(id);
        let stored =
            store(&temporary, title.as_bytes()).and_then(|_| Ok(fs::rename(&temporary, &path)?));
        if stored.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        stored.with_context(|| format!("Title {} not stored", path.display()))
    }

    /// Stores how risky the call of the waiting entry looks, in place of an
    /// earlier risk. The lists show it with the entry.
    pub fn set_risk(&self, id: &str, risk: Risk) -> Result<()> {
        ensure!(is_id(id), "Entry ID {id:?} has another form");
        let temporary = self.risks.join(format!(".{id}.{}.tmp", process::id()));
        let path = self.risks.join(id);
        let stored = store(&temporary, risk.name().as_bytes())
            .and_then(|_| Ok(fs::rename(&temporary, &path)?));
        if stored.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        stored.with_context(|| format!("Risk {} not stored", path.display()))
    }

    /// The risk of the entry, when its service process stored one.
    fn risk_of(&self, id: &str) -> Option<Risk> {
        Risk::of_name(&fs::read_to_string(self.risks.join(id)).ok()?)
    }

    /// The later title of the entry, when its service process stored one
    /// that is a title.
    fn later_title(&self, id: &str) -> Option<String> {
        let title = fs::read_to_string(self.titles.join(id)).ok()?;
        check_title(&title).ok()?;
        Some(title)
    }

    /// Applies the report to the state of its session: stores the new
    /// state in place of the one before, removes the state of a session that
    /// ended, or keeps the state that a later report gave. Refuses a report
    /// for the session of another user, a report that was sent after `now`
    /// and a new session of a user that has `MAX_SESSIONS_OF_USER`. Returns
    /// the state that the session has after it.
    pub fn report_session(
        &self,
        report: &SessionReport,
        now: u64,
        user_is_verified: bool,
    ) -> Result<Option<SessionState>> {
        let path = self.session_path(&report.session)?;
        let before = read_session(&path);
        if let Some(before) = &before {
            ensure!(
                before.user == report.user && before.user_is_verified == user_is_verified,
                "Session belongs to another user"
            );
        }
        ensure!(
            report.sent_at <= latest_sent_at(now),
            "Report field sent_at is after the time of the service"
        );
        let stored = match SessionState::after(before.as_ref(), report, now, user_is_verified) {
            Change::Keep => return Ok(before),
            Change::Remove => {
                return match fs::remove_file(&path) {
                    Err(error) if error.kind() != ErrorKind::NotFound => {
                        Err(error).with_context(|| format!("State {} not removed", path.display()))
                    }
                    _ => Ok(None),
                };
            }
            Change::Store(stored) => stored,
        };
        if before.is_none() {
            let of_user = self
                .sessions()?
                .iter()
                .filter(|listed| listed.state.user == report.user)
                .count();
            ensure!(
                of_user < MAX_SESSIONS_OF_USER,
                "User has {MAX_SESSIONS_OF_USER} sessions with a state"
            );
        }
        let temporary = self
            .sessions
            .join(format!(".{}.{}.tmp", report.session, process::id()));
        let written = store(&temporary, &serde_json::to_vec(&stored)?)
            .and_then(|_| Ok(fs::rename(&temporary, &path)?));
        if written.is_err() {
            let _ = fs::remove_file(&temporary);
        }
        written.with_context(|| format!("State {} not stored", path.display()))?;
        Ok(Some(stored))
    }

    /// The sessions that reported a state, in the order of their IDs.
    /// Removes the state of a session whose process is gone, a file of
    /// another form and each old temporary file.
    pub fn sessions(&self) -> Result<Vec<ListedSession>> {
        let mut listed = Vec::new();
        for path in files_of(&self.sessions)? {
            let name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
            let session = name
                .strip_suffix(".json")
                .filter(|session| is_session_id(session));
            let Some(session) = session else {
                let is_new_temporary = name.starts_with('.')
                    && !File::open(&path).is_ok_and(|file| is_older_than(&file, LEFTOVER_AGE));
                if !is_new_temporary {
                    let _ = fs::remove_file(&path);
                }
                continue;
            };
            match read_session(&path) {
                Some(state) if process_folder(state.pid).exists() => listed.push(ListedSession {
                    session: session.to_owned(),
                    state,
                }),
                read => self.forget_session(session, read.as_ref()),
            }
        }
        listed.sort_by(|one, other| one.session.cmp(&other.session));
        Ok(listed)
    }

    /// Removes the state file of the session when it still reads as `read`.
    /// A report can store a new state after the read. That state stays.
    pub fn forget_session(&self, session: &str, read: Option<&SessionState>) {
        if let Ok(path) = self.session_path(session)
            && read_session(&path).as_ref() == read
        {
            let _ = fs::remove_file(&path);
        }
    }

    fn session_path(&self, session: &str) -> Result<PathBuf> {
        ensure!(
            is_session_id(session),
            "Session ID {session:?} has another form"
        );
        Ok(self.sessions.join(format!("{session}.json")))
    }

    /// Stores the owner's answer for the service process of the entry.
    pub fn answer(&self, id: &str, answer: &Answer) -> Result<()> {
        ensure!(is_id(id), "Entry ID {id:?} has another form");
        answer.check()?;
        ensure!(
            self.entries.join(format!("{id}.json")).exists(),
            "Prompt left the queue"
        );
        let temporary = self.answers.join(format!(".{id}.{}.tmp", process::id()));
        let path = self.answers.join(format!("{id}.json"));
        // A link fails when another list stored an answer first
        let stored = store(&temporary, &serde_json::to_vec(answer)?)
            .and_then(|_| Ok(fs::hard_link(&temporary, &path)?));
        let _ = fs::remove_file(&temporary);
        match stored {
            Err(error)
                if error
                    .downcast_ref::<io::Error>()
                    .is_some_and(|error| error.kind() == ErrorKind::AlreadyExists) =>
            {
                bail!("Prompt has its answer")
            }
            stored => stored.with_context(|| format!("Answer {} not stored", path.display())),
        }
    }

    /// The answer for the entry, when a list stored one. The file stays until
    /// the entry leaves the queue, so it refuses a second answer.
    pub fn answer_for(&self, id: &str) -> Result<Option<Answer>> {
        let path = self.answers.join(format!("{id}.json"));
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(error).with_context(|| format!("Answer {} unreadable", path.display()));
            }
        };
        let answer: Answer = serde_json::from_str(&text).context("Answer has another form")?;
        answer.check()?;
        Ok(Some(answer))
    }

    /// Removes each answer, later title and risk whose entry left the queue
    /// without the end of its service process removing them, and each old
    /// temporary file.
    pub fn remove_leftovers(&self) -> Result<()> {
        for path in files_of(&self.answers)?
            .into_iter()
            .chain(files_of(&self.titles)?)
            .chain(files_of(&self.risks)?)
        {
            let name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
            let id = name.strip_suffix(".json").unwrap_or(name);
            let is_leftover = if name.starts_with('.') {
                File::open(&path).is_ok_and(|file| is_older_than(&file, LEFTOVER_AGE))
            } else {
                !self.entries.join(format!("{id}.json")).exists()
            };
            if is_leftover {
                let _ = fs::remove_file(&path);
            }
        }
        Ok(())
    }
}

fn read_session(path: &Path) -> Option<SessionState> {
    serde_json::from_reader(BufReader::new(File::open(path).ok()?)).ok()
}

/// An ID that sorts by its time and names its process.
fn new_id() -> Result<String> {
    let nanoseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Clock is before 1970")?
        .as_nanos();
    Ok(format!("{nanoseconds:020}-{}", process::id()))
}

/// Stores the content at the path and returns the file with an exclusive
/// lock. The file gets its name only with its lock and its content, so no
/// reader takes it for the leftover of a process that ended.
fn store_locked(folder: &Path, id: &str, path: &Path, content: &[u8]) -> Result<File> {
    let temporary = folder.join(format!(".{id}.tmp"));
    let stored = store(&temporary, content).and_then(|file| {
        fs::rename(&temporary, path)?;
        Ok(file)
    });
    if stored.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    stored
}

/// The file when a process holds its lock. Removes a file without a lock: the
/// leftover of a process that ended. A temporary file goes only after
/// `LEFTOVER_AGE`, because its process locks it right after it creates it.
fn locked(path: &Path) -> Option<File> {
    let name = path.file_name().and_then(OsStr::to_str)?;
    let file = File::open(path).ok()?;
    match file.try_lock_shared() {
        Ok(()) => {
            if !name.starts_with('.') || is_older_than(&file, LEFTOVER_AGE) {
                let _ = fs::remove_file(path);
            }
            None
        }
        Err(TryLockError::WouldBlock) => Some(file),
        Err(TryLockError::Error(_)) => None,
    }
}

/// Writes a new file for its owner only and returns it with an exclusive lock.
fn store(path: &Path, content: &[u8]) -> Result<File> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    file.lock()?;
    file.write_all(content)?;
    Ok(file)
}

fn files_of(folder: &Path) -> Result<Vec<PathBuf>> {
    fs::read_dir(folder)
        .with_context(|| format!("Folder {} unreadable", folder.display()))?
        .map(|item| Ok(item?.path()))
        .collect()
}

fn is_older_than(file: &File, age: Duration) -> bool {
    file.metadata()
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .is_some_and(|elapsed| elapsed > age)
}

fn entry_id(path: &Path) -> Option<&str> {
    path.file_name()
        .and_then(OsStr::to_str)
        .and_then(|name| name.strip_suffix(".json"))
        .filter(|id| is_id(id))
}

fn is_id(text: &str) -> bool {
    !text.is_empty()
        && text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'-')
}

/// The minutes that a mute until `until` still lasts at `now`, a part of a
/// minute as a whole one. `None` when the mute is over.
#[must_use]
pub fn muted_minutes(until: SystemTime, now: SystemTime) -> Option<u64> {
    let left = until
        .duration_since(now)
        .ok()
        .filter(|left| !left.is_zero())?;
    Some(u64::try_from(left.as_millis().div_ceil(60_000)).unwrap_or(u64::MAX))
}
