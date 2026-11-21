use std::{
    env, io,
    os::{fd::AsFd, unix::net::UnixStream},
    path::Path,
    process::ExitCode,
    time::Duration,
};

use anyhow::{Context, Result};
use approval_queue::{log_text, queue_folder::QueueFolder, service::Service};

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
        poll_interval: Duration::from_millis(200),
        longest_wait: LONGEST_WAIT,
        no_list_grace: NO_LIST_GRACE,
    };
    let (entry, outcome) = service
        .serve(&mut stream, sender.uid == rustix::process::getuid())
        .context("Request refused")?;
    eprintln!("{}", log_text::served(&entry, &outcome));
    Ok(())
}
