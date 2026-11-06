use std::{
    ffi::OsString,
    io::{BufRead, BufReader, Read, Write},
    os::unix::net::UnixStream,
    time::Duration,
};

use anyhow::{Context, Result, anyhow, bail};

use crate::{
    config::Config,
    launch,
    request::{self, MAX_REQUEST_BYTES},
};

const READ_TIMEOUT: Duration = Duration::from_secs(2);

/// Reads one request, opens it with `launch` and replies. Returns the request
/// line and the result, for the log.
pub fn serve(
    stream: &mut UnixStream,
    config: &Config,
    launch: impl FnOnce(&[OsString]) -> Result<()>,
) -> (Option<String>, Result<()>) {
    let line = read_line(stream);
    let result = match &line {
        Ok(line) => request::check(line, config)
            .and_then(|request| launch(&launch::command_line(&request, config))),
        Err(error) => Err(anyhow!("{error:#}")),
    };
    reply(stream, &result);
    (line.ok(), result)
}

/// Writes `ok`, or `error: <reason>` with the outer reason only. The inner
/// reasons, such as which path was refused, go only to the journal. The client
/// can close first, so a failed write only loses the reply.
pub fn reply(stream: &mut UnixStream, result: &Result<()>) {
    let reply = match result {
        Ok(()) => "ok\n".to_owned(),
        Err(error) => format!("error: {error}\n"),
    };
    let _ = stream.write_all(reply.as_bytes());
}

fn read_line(stream: &UnixStream) -> Result<String> {
    stream.set_read_timeout(Some(READ_TIMEOUT))?;
    let limit = u64::try_from(MAX_REQUEST_BYTES).unwrap_or(u64::MAX);
    let mut bytes = Vec::new();
    BufReader::new(stream.take(limit))
        .read_until(b'\n', &mut bytes)
        .context("Request unreadable")?;
    match bytes.last() {
        Some(b'\n') => {
            bytes.pop();
        }
        None => bail!("Request empty"),
        Some(_) if bytes.len() >= MAX_REQUEST_BYTES => {
            bail!("Request longer than {MAX_REQUEST_BYTES} bytes")
        }
        Some(_) => bail!("Request ends without a newline"),
    }
    String::from_utf8(bytes).context("Request not UTF-8")
}
