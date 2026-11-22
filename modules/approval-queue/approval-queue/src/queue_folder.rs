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
    answer::Answer, entry::Entry, listed_entry::ListedEntry, open_list::OpenList,
    waiting_entry::WaitingEntry,
};

/// A temporary file that no process locks is a leftover after this age.
const LEFTOVER_AGE: Duration = Duration::from_secs(60);

/// The folders through which the service processes and the lists share the
/// waiting prompts: one file for each entry, one for each answer and one for
/// each open list.
#[derive(Debug, Clone)]
pub struct QueueFolder {
    entries: PathBuf,
    answers: PathBuf,
    lists: PathBuf,
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
            lists: root.join("lists"),
            mute: root.join("pings-muted-until"),
        }
    }

    /// Creates the folders, for their owner only.
    pub fn create(&self) -> Result<()> {
        for folder in [&self.entries, &self.answers, &self.lists] {
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
            let entry = serde_json::from_reader(BufReader::new(&file));
            if let (Some(id), Ok(entry)) = (id, entry) {
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

    /// Removes each answer whose entry left the queue before its service
    /// process read the answer, and each old temporary file.
    pub fn remove_answers_without_entry(&self) -> Result<()> {
        for path in files_of(&self.answers)? {
            let name = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
            let is_leftover = match name.strip_suffix(".json") {
                Some(id) if !name.starts_with('.') => {
                    !self.entries.join(format!("{id}.json")).exists()
                }
                _ => File::open(&path).is_ok_and(|file| is_older_than(&file, LEFTOVER_AGE)),
            };
            if is_leftover {
                let _ = fs::remove_file(&path);
            }
        }
        Ok(())
    }
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
