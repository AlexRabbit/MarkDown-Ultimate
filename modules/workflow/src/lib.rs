use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const WORKFLOW_MAGIC: &str = "MDU-WORKFLOW-1";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkflowMeta {
    #[serde(default)]
    pub active_file_id: Option<String>,
    #[serde(default)]
    pub view_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowManifest {
    pub magic: String,
    pub version: u32,
    pub exported_at: DateTime<Utc>,
    pub app: String,
    pub tree: serde_json::Value,
    #[serde(default)]
    pub meta: WorkflowMeta,
}

#[derive(Debug, Error)]
pub enum WorkflowError {
    #[error("invalid workflow container")]
    Invalid,
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn export_container(tree: serde_json::Value, meta: WorkflowMeta) -> Result<String, WorkflowError> {
    let manifest = WorkflowManifest {
        magic: WORKFLOW_MAGIC.into(),
        version: 1,
        exported_at: Utc::now(),
        app: "MarkDown-Ultimate".into(),
        tree,
        meta,
    };
    Ok(serde_json::to_string_pretty(&manifest)?)
}

pub fn import_container(raw: &str) -> Result<(serde_json::Value, WorkflowMeta), WorkflowError> {
    let manifest: WorkflowManifest = serde_json::from_str(raw)?;
    if manifest.magic != WORKFLOW_MAGIC {
        return Err(WorkflowError::Invalid);
    }
    Ok((manifest.tree, manifest.meta))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_roundtrip_meta() {
        let tree = serde_json::json!({ "root": [] });
        let meta = WorkflowMeta {
            active_file_id: Some("abc".into()),
            view_mode: Some("split".into()),
        };
        let raw = export_container(tree.clone(), meta.clone()).unwrap();
        let (t2, m2) = import_container(&raw).unwrap();
        assert_eq!(t2, tree);
        assert_eq!(m2.active_file_id, meta.active_file_id);
        assert_eq!(m2.view_mode, meta.view_mode);
    }
}
