use std::{
    fs,
    io::{ErrorKind, Read, Write},
    os::unix::{fs::MetadataExt, net::UnixStream},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail, ensure};
use serde_json::json;

use crate::{
    entry::Entry,
    log_text,
    outcome::Outcome,
    process_tree::{is_at_or_above, process_folder},
    queue_folder::QueueFolder,
    request::{MAX_REQUEST_BYTES, Request, TitleUpdate},
    session_state::{SessionReport, SessionState},
};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
/// Longest later line of a hook, the newline included: above a title of the
/// longest size in JSON with each character as an escape, and its key.
const MAX_UPDATE_BYTES: usize = 2048;

/// What one connection of a hook was.
#[derive(Debug, Clone, PartialEq)]
pub enum Served {
    /// A prompt, with how its wait ended.
    Prompt(Entry, Outcome),
    /// A report on the state of a session, with its ID and the state that
    /// the queue now has for it. `None` for a session that ended.
    Session(String, Option<SessionState>),
}

/// The process that opened a connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sender {
    /// The user ID of the process, as the service sees it.
    pub uid: u32,
    pub pid: u32,
}

/// Serves the connection of one hook. A request line in, then lines that
/// give the waiting prompt a new title, and at most one reply line out. Or
/// one report line on the state of a session in, and no reply.
#[derive(Debug)]
pub struct Service<'a> {
    pub queue: &'a QueueFolder,
    /// The name of the user that owns the queue.
    pub own_user: &'a str,
    /// The user ID of that user.
    pub own_uid: u32,
    /// How often the service looks for the answer and for the end of the
    /// connection.
    pub poll_interval: Duration,
    /// The service ends without a reply after this time.
    pub longest_wait: Duration,
    /// A prompt that only the list can answer goes back to its terminal after
    /// this time without an open list.
    pub no_list_grace: Duration,
}

impl Service<'_> {
    /// Puts the request into the queue and waits. Replies the owner's answer
    /// as one JSON line. A refused request gets `{"error": <reason>}` with the
    /// outer reason only, and so does a prompt that only the list can answer
    /// while no list is open. Returns the entry and how its wait ended, for
    /// the log. A report line on the state of a session changes that state
    /// and gets a reply only when it is refused.
    pub fn serve(&self, stream: &mut UnixStream, sender: Sender) -> Result<Served> {
        let sender_is_own_user = sender.uid == self.own_uid;
        let read = read_line(stream);
        let (line, mut later_bytes) = replied(stream, read)?;
        if SessionReport::is_named_in(&line) {
            let reported = self.take_report(&line, sender);
            return replied(stream, reported);
        }
        let accepted = self
            .accept(&line, sender_is_own_user)
            .and_then(|entry| Ok((self.queue.add(&entry)?, entry)));
        let (waiting, entry) = replied(stream, accepted)?;
        let deadline = Instant::now() + self.longest_wait;
        let mut list_seen = Instant::now();
        self.take_titles(waiting.id(), &mut later_bytes)?;
        stream.set_read_timeout(Some(self.poll_interval))?;
        loop {
            if let Some(answer) = self.queue.answer_for(waiting.id())? {
                reply(stream, &serde_json::to_value(&answer)?);
                return Ok(Served::Prompt(entry, Outcome::Answered(answer)));
            }
            let now = Instant::now();
            if now >= deadline {
                return Ok(Served::Prompt(entry, Outcome::Expired));
            }
            if entry.list_only {
                if self.queue.has_open_list()? {
                    list_seen = now;
                } else if now.duration_since(list_seen) >= self.no_list_grace {
                    reply(stream, &json!({ "error": "No list is open" }));
                    return Ok(Served::Prompt(entry, Outcome::NoList));
                }
            }
            let mut chunk = [0; 512];
            match stream.read(&mut chunk) {
                Ok(0) => return Ok(Served::Prompt(entry, Outcome::ClientLeft)),
                Ok(count) => {
                    later_bytes.extend_from_slice(&chunk[..count]);
                    self.take_titles(waiting.id(), &mut later_bytes)?;
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::Interrupted
                    ) => {}
                Err(error) if error.kind() == ErrorKind::ConnectionReset => {
                    return Ok(Served::Prompt(entry, Outcome::ClientLeft));
                }
                Err(error) => return Err(error).context("Connection failed"),
            }
        }
    }

    /// Takes each whole line out of the bytes that the hook sent after its
    /// request, and stores the title of each line that gives one. A line of
    /// another form and a title that the queue cannot store change nothing:
    /// the prompt keeps its place and its title. Fails for a line above
    /// `MAX_UPDATE_BYTES`.
    fn take_titles(&self, id: &str, later_bytes: &mut Vec<u8>) -> Result<()> {
        while let Some(end) = later_bytes.iter().position(|byte| *byte == b'\n') {
            let line: Vec<u8> = later_bytes.drain(..=end).collect();
            ensure!(line.len() <= MAX_UPDATE_BYTES, line_too_long());
            let update = std::str::from_utf8(&line[..end])
                .context("Update not UTF-8")
                .and_then(TitleUpdate::parse);
            if let Ok(update) = update
                && let Err(error) = self.queue.set_title(id, &update.title)
            {
                eprintln!("{}", log_text::failed(&error));
            }
        }
        ensure!(later_bytes.len() < MAX_UPDATE_BYTES, line_too_long());
        Ok(())
    }

    /// Applies the report line of a session. The process that the report
    /// names must be the sender or a process above it, so the state leaves
    /// with a process of the sender. The user IDs of the processes do not
    /// tell that: the service sees one ID for every user but its own. So
    /// it only refuses a process of its own user from another sender. The
    /// sender must still run: the hook waits until the service closes.
    fn take_report(&self, line: &str, sender: Sender) -> Result<Served> {
        let sender_is_own_user = sender.uid == self.own_uid;
        let report = SessionReport::parse(line)?;
        self.check_user(&report.user, sender_is_own_user)?;
        ensure!(
            is_at_or_above(report.pid, sender.pid)
                && (sender_is_own_user
                    || fs::metadata(process_folder(report.pid))
                        .is_ok_and(|process| process.uid() != self.own_uid)),
            "Report field pid names neither its sender nor a process above it"
        );
        let state = self
            .queue
            .report_session(&report, now()?, sender_is_own_user)?;
        Ok(Served::Session(report.session, state))
    }

    /// The entry of the request line.
    fn accept(&self, line: &str, sender_is_own_user: bool) -> Result<Entry> {
        let request = Request::parse(line)?;
        self.check_user(&request.user, sender_is_own_user)?;
        Ok(Entry::of(request, now()?, sender_is_own_user))
    }

    /// Refuses a line that names the owner of the queue from another
    /// sender, or another user from the owner.
    fn check_user(&self, user: &str, sender_is_own_user: bool) -> Result<()> {
        ensure!(
            (user == self.own_user) == sender_is_own_user,
            "Line names a user other than its sender"
        );
        Ok(())
    }
}

/// Replies the outer reason of a failure to the hook, and gives the result
/// back.
fn replied<T>(stream: &mut UnixStream, result: Result<T>) -> Result<T> {
    if let Err(error) = &result {
        reply(stream, &json!({ "error": error.to_string() }));
    }
    result
}

/// Seconds since the Unix epoch.
fn now() -> Result<u64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Clock is before 1970")?
        .as_secs())
}

fn line_too_long() -> String {
    format!("Hook sent a line longer than {MAX_UPDATE_BYTES} bytes after its request")
}

/// The hook can close first, so a failed write only loses the reply.
fn reply(stream: &mut UnixStream, value: &serde_json::Value) {
    let _ = stream.write_all(format!("{value}\n").as_bytes());
}

/// Reads the first line, without its newline, and the bytes after it that
/// came in the same read: a hook can send a later line at once.
fn read_line(stream: &mut UnixStream) -> Result<(String, Vec<u8>)> {
    let deadline = Instant::now() + REQUEST_TIMEOUT;
    let mut bytes = Vec::new();
    let mut chunk = [0; 4096];
    loop {
        let left = deadline.saturating_duration_since(Instant::now());
        ensure!(!left.is_zero(), "Request not complete in time");
        stream.set_read_timeout(Some(left))?;
        let count = match stream.read(&mut chunk) {
            Ok(count) => count,
            Err(error) if error.kind() == ErrorKind::Interrupted => continue,
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                bail!("Request not complete in time")
            }
            Err(error) => return Err(error).context("Request unreadable"),
        };
        if count == 0 {
            ensure!(!bytes.is_empty(), "Request empty");
            bail!("Request ends without a newline");
        }
        bytes.extend_from_slice(&chunk[..count]);
        if let Some(end) = bytes.iter().position(|byte| *byte == b'\n') {
            ensure!(
                end < MAX_REQUEST_BYTES,
                "Request longer than {MAX_REQUEST_BYTES} bytes"
            );
            let later_bytes = bytes.split_off(end + 1);
            bytes.truncate(end);
            let line = String::from_utf8(bytes).context("Request not UTF-8")?;
            return Ok((line, later_bytes));
        }
        ensure!(
            bytes.len() < MAX_REQUEST_BYTES,
            "Request longer than {MAX_REQUEST_BYTES} bytes"
        );
    }
}
