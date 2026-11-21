use std::{
    fs::{self, File},
    path::PathBuf,
};

/// The mark of one open list. The list keeps an exclusive lock on the file,
/// so a service process can tell an open list from one that ended.
#[derive(Debug)]
pub struct OpenList {
    pub(crate) path: PathBuf,
    pub(crate) _lock: File,
}

impl Drop for OpenList {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
