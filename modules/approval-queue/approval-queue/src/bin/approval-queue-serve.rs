use std::{
    env, io,
    os::{fd::AsFd, unix::net::UnixStream},
    path::Path,
    process::ExitCode,
    time::Duration,
};

use anyhow::{Context, Result};
use approval_queue::{
    log_text,
    queue_folder::QueueFolder,
    service::{Sender, Served, Service},
};

/// Above the day that a hook waits, so the hook ends the wait.
const LONGEST_WAIT: Duration = Duration::from_hours(25);
/// Long enough for the owner to close a list and open it again.
const NO_LIST_GRACE: Duration = Duration::from_secs(3);

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", log_text::failed(&error));
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let socket = io::stdin()
        .as_fd()
        .try_clone_to_owned()
        .context("Standard input not usable; start the service through its socket unit")?;
    let sender = rustix::net::sockopt::socket_peercred(&socket)
        .context("Standard input is not a connection of the socket unit")?;
    let mut stream = UnixStream::from(socket);
    let runtime_folder = env::var_os("XDG_RUNTIME_DIR").context("XDG_RUNTIME_DIR is not set")?;
    let own_user = env::var("USER").context("USER is not set")?;
    let queue = QueueFolder::in_runtime_folder(Path::new(&runtime_folder));
    queue.create()?;
    let service = Service {
        queue: &queue,
        own_user: &own_user,
        own_uid: rustix::process::getuid().as_raw(),
        poll_interval: Duration::from_millis(200),
        longest_wait: LONGEST_WAIT,
        no_list_grace: NO_LIST_GRACE,
    };
    let served = service
        .serve(
            &mut stream,
            Sender {
                uid: sender.uid.as_raw(),
                pid: sender.pid.as_raw_nonzero().get().cast_unsigned(),
            },
        )
        .context("Request refused")?;
    match served {
        Served::Prompt(entry, outcome) => eprintln!("{}", log_text::served(&entry, &outcome)),
        Served::Session(session, state) => {
            eprintln!("{}", log_text::reported(&session, state.as_ref()));
        }
    }
    Ok(())
}
