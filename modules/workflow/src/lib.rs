use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const WORKFLOW_MAGIC: &str = "MDU-WORKFLOW-1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowManifest {
    pub magic: String,
    pub version: u32,
    pub exported_at: DateTime<Utc>,
    pub app: String,
    pub tree: serde_json::Value,
}

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("invalid workflow container")]
    Invalid,
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn export_container(tree: serde_json::Value) -> Result<String, WorkflowError> {
    let manifest = WorkflowManifest {
        magic: WORKFLOW_MAGIC.into(),
        version: 1,
        exported_at: Utc::now(),
        app: "MarkDown-Ultimate".into(),
        tree,
    };
    Ok(serde_json::to_string_pretty(&manifest)?)
}

pub fn import_container(raw: &str) -> Result<serde_json::Value, WorkflowError> {
    let manifest: WorkflowManifest = serde_json::from_str(raw)?;
    if manifest.magic != WORKFLOW_MAGIC {
        return Err(WorkflowError::Invalid);
    }
    Ok(manifest.tree)
}
