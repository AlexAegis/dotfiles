use std::{
    io::{ErrorKind, Read, Write},
    os::unix::net::UnixStream,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use anyhow::{Context, Result, bail, ensure};
use serde_json::json;

use crate::{
    entry::Entry,
    outcome::Outcome,
    queue_folder::QueueFolder,
    request::{MAX_REQUEST_BYTES, Request},
};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Serves the connection of one hook: one request line in, at most one reply
/// line out.
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
            .and_then(|entry| Ok((self.queue.add(&entry)?, entry)));
        let (waiting, entry) = match accepted {
            Ok(accepted) => accepted,
            Err(error) => {
                reply(stream, &json!({ "error": error.to_string() }));
                return Err(error);
            }
        };
        let deadline = Instant::now() + self.longest_wait;
        let mut list_seen = Instant::now();
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
            match stream.read(&mut [0]) {
                Ok(0) => return Ok((entry, Outcome::ClientLeft)),
                Ok(_) => bail!("Hook sent bytes after its request"),
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

    fn accept(&self, stream: &mut UnixStream, sender_is_own_user: bool) -> Result<Entry> {
        let request = Request::parse(&read_line(stream)?)?;
        ensure!(
            (request.user == self.own_user) == sender_is_own_user,
            "Request names a user other than its sender"
        );
        let received = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Clock is before 1970")?
            .as_secs();
        Ok(Entry::of(request, received, sender_is_own_user))
    }
}

/// The hook can close first, so a failed write only loses the reply.
fn reply(stream: &mut UnixStream, value: &serde_json::Value) {
    let _ = stream.write_all(format!("{value}\n").as_bytes());
}

fn read_line(stream: &mut UnixStream) -> Result<String> {
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
            ensure!(end + 1 == bytes.len(), "Request has bytes after its line");
            bytes.truncate(end);
            return String::from_utf8(bytes).context("Request not UTF-8");
        }
        ensure!(
            bytes.len() < MAX_REQUEST_BYTES,
            "Request longer than {MAX_REQUEST_BYTES} bytes"
        );
    }
}
