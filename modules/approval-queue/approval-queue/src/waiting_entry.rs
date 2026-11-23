use std::{
    fs::{self, File},
    path::PathBuf,
};

/// The entry file of one service process. The process keeps an exclusive lock
/// on the file, so the list can tell an entry whose process died from a
/// waiting one.
#[derive(Debug)]
pub struct WaitingEntry {
    pub(crate) id: String,
    pub(crate) path: PathBuf,
    pub(crate) answer_path: PathBuf,
    pub(crate) title_path: PathBuf,
    pub(crate) risk_path: PathBuf,
    pub(crate) _lock: File,
}

impl WaitingEntry {
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Drop for WaitingEntry {
    /// The answer goes after the entry. While the entry is in the queue, its
    /// answer file refuses a second answer.
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
        let _ = fs::remove_file(&self.answer_path);
        let _ = fs::remove_file(&self.title_path);
        let _ = fs::remove_file(&self.risk_path);
    }
}
