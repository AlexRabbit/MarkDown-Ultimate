use mu_workspace::Workspace;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

pub struct FileWorkspaceStore {
    path: PathBuf,
}

impl FileWorkspaceStore {
    pub fn new(root: impl AsRef<Path>) -> Self {
        let path = root.as_ref().join("workspace-default.json");
        Self { path }
    }

    pub fn load(&self) -> Result<Workspace, StoreError> {
        if !self.path.exists() {
            return Ok(Workspace::default());
        }
        let raw = fs::read_to_string(&self.path)?;
        Ok(serde_json::from_str(&raw)?)
    }

    pub fn save(&self, ws: &Workspace) -> Result<(), StoreError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, serde_json::to_string_pretty(ws)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mu_workspace::Node;

    #[test]
    fn round_trip() {
        let dir = std::env::temp_dir().join("mdu-store-test");
        let _ = fs::remove_dir_all(&dir);
        let store = FileWorkspaceStore::new(&dir);
        let mut ws = Workspace::default();
        ws.root.push(Node::new_file("a.md"));
        store.save(&ws).unwrap();
        let loaded = store.load().unwrap();
        assert_eq!(loaded.flatten_files_newest_first().len(), 1);
    }
}
