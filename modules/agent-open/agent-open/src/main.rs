use std::{env, path::PathBuf, process::ExitCode};

use agent_open::{config::Config, connection, launch, socket_activation};
use anyhow::{Context, Result, anyhow};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let mut stream = socket_activation::take_connection()?;
    let config = match load_config() {
        Ok(config) => config,
        Err(error) => {
            connection::reply(&mut stream, &Err(anyhow!("agent-open has no valid config")));
            return Err(error);
        }
    };
    let (line, result) = connection::serve(&mut stream, &config, launch::run);
    let line = line.as_deref().unwrap_or("<unreadable>");
    result.with_context(|| format!("Request {line:?} refused"))?;
    eprintln!("Request {line:?} opened");
    Ok(())
}

fn load_config() -> Result<Config> {
    let path = env::var_os("AGENT_OPEN_CONFIG").context("AGENT_OPEN_CONFIG is not set")?;
    let home = env::var_os("HOME").context("HOME is not set")?;
    Config::load(&PathBuf::from(path), &PathBuf::from(home))
}
