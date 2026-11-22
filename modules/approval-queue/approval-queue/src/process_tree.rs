//! The processes above a process, as `/proc` shows them.

use std::{
    fs,
    path::{Path, PathBuf},
};

/// The most processes above a sender that the service reads.
const MAX_PROCESSES_ABOVE: usize = 64;
/// The first process of the system, which is above every process.
const FIRST_PROCESS: u32 = 1;

/// The folder that exists while the process runs.
#[must_use]
pub fn process_folder(pid: u32) -> PathBuf {
    Path::new("/proc").join(pid.to_string())
}

/// The process that started the process, while both run.
#[must_use]
pub fn parent_of(pid: u32) -> Option<u32> {
    fs::read_to_string(process_folder(pid).join("status"))
        .ok()?
        .lines()
        .find_map(|line| line.strip_prefix("PPid:"))?
        .trim()
        .parse()
        .ok()
}

/// True when `pid` is the process `below` or a process above it, other
/// than the first process of the system.
#[must_use]
pub fn is_at_or_above(pid: u32, below: u32) -> bool {
    let mut current = below;
    for _ in 0..=MAX_PROCESSES_ABOVE {
        if current <= FIRST_PROCESS {
            return false;
        }
        if current == pid {
            return process_folder(pid).exists();
        }
        let Some(parent) = parent_of(current) else {
            return false;
        };
        current = parent;
    }
    false
}
