use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Node {
    Folder {
        id: String,
        name: String,
        children: Vec<Node>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    },
    File {
        id: String,
        name: String,
        content: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    },
}

impl Node {
    pub fn new_file(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self::File {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            content: String::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_folder(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self::Folder {
            id: Uuid::new_v4().to_string(),
            name: name.into(),
            children: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Workspace {
    pub root: Vec<Node>,
}

impl Workspace {
    pub fn flatten_files_newest_first(&self) -> Vec<(String, String)> {
        let mut items = Vec::new();
        walk(&self.root, String::new(), &mut items);
        items.sort_by(|a, b| b.1.cmp(&a.1));
        items
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    pub fn from_json(v: serde_json::Value) -> Self {
        serde_json::from_value(v).unwrap_or_default()
    }
}

fn walk(nodes: &[Node], prefix: String, out: &mut Vec<(String, String)>) {
    for n in nodes {
        match n {
            Node::File {
                id,
                name,
                updated_at,
                ..
            } => {
                let path = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{prefix}/{name}")
                };
                out.push((id.clone(), path));
                let _ = updated_at;
            }
            Node::Folder { name, children, .. } => {
                let p = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{prefix}/{name}")
                };
                walk(children, p, out);
            }
        }
    }
}
