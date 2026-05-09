use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use kastore::SaveGame;

#[derive(Debug, Default)]
pub struct ManagedSaveState {
    current: Mutex<Option<SaveState>>,
}

impl ManagedSaveState {
    pub fn lock(&self) -> Result<MutexGuard<'_, Option<SaveState>>, String> {
        self.current
            .lock()
            .map_err(|_| "Open save state lock was poisoned.".to_string())
    }
}

#[derive(Debug)]
pub struct SaveState {
    pub path: PathBuf,
    pub save: SaveGame,
    pub dirty: bool,
    pub revision: u64,
}

impl SaveState {
    pub fn opened(path: PathBuf, save: SaveGame) -> Self {
        Self {
            path,
            save,
            dirty: false,
            revision: 0,
        }
    }

    pub fn mark_changed(&mut self) {
        self.revision += 1;
        self.dirty = true;
    }
}
