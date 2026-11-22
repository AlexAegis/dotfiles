use std::{
    io::{ErrorKind, Read, Write},
    os::unix::net::UnixStream,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail, ensure};
use serde_json::json;

use crate::{
    entry::Entry,
    log_text,
    outcome::Outcome,
    queue_folder::QueueFolder,
    request::{MAX_REQUEST_BYTES, Request, TitleUpdate},
};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
/// Longest later line of a hook, the newline included: above a title of the
/// longest size in JSON with each character as an escape, and its key.
const MAX_UPDATE_BYTES: usize = 2048;

/// Serves the connection of one hook: one request line in, then lines that
/// give the waiting prompt a new title, and at most one reply line out.
#[derive(Debug)]
pub struct Service<'a> {
    pub queue: &'a QueueFolder,
    /// The name of the user that owns the queue.
    pub own_user: &'a str,
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
    /// the log.
    pub fn serve(
        &self,
        stream: &mut UnixStream,
        sender_is_own_user: bool,
    ) -> Result<(Entry, Outcome)> {
        let accepted = self
            .accept(stream, sender_is_own_user)
            .and_then(|(entry, later_bytes)| Ok((self.queue.add(&entry)?, entry, later_bytes)));
        let (waiting, entry, mut later_bytes) = match accepted {
            Ok(accepted) => accepted,
            Err(error) => {
                reply(stream, &json!({ "error": error.to_string() }));
                return Err(error);
            }
        };
        let deadline = Instant::now() + self.longest_wait;
        let mut list_seen = Instant::now();
        self.take_titles(waiting.id(), &mut later_bytes)?;
        stream.set_read_timeout(Some(self.poll_interval))?;
        loop {
            if let Some(answer) = self.queue.answer_for(waiting.id())? {
                reply(stream, &serde_json::to_value(&answer)?);
                return Ok((entry, Outcome::Answered(answer)));
            }
            let now = Instant::now();
            if now >= deadline {
                return Ok((entry, Outcome::Expired));
            }
            if entry.list_only {
                if self.queue.has_open_list()? {
                    list_seen = now;
                } else if now.duration_since(list_seen) >= self.no_list_grace {
                    reply(stream, &json!({ "error": "No list is open" }));
                    return Ok((entry, Outcome::NoList));
                }
            }
            let mut chunk = [0; 512];
            match stream.read(&mut chunk) {
                Ok(0) => return Ok((entry, Outcome::ClientLeft)),
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
                    return Ok((entry, Outcome::ClientLeft));
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

    /// Reads the request. Returns its entry and the bytes that came after
    /// its line in the same read.
    fn accept(
        &self,
        stream: &mut UnixStream,
        sender_is_own_user: bool,
    ) -> Result<(Entry, Vec<u8>)> {
        let (line, later_bytes) = read_line(stream)?;
        let request = Request::parse(&line)?;
        ensure!(
            (request.user == self.own_user) == sender_is_own_user,
            "Request names a user other than its sender"
        );
        let received = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Clock is before 1970")?
            .as_secs();
        Ok((
            Entry::of(request, received, sender_is_own_user),
            later_bytes,
        ))
    }
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
