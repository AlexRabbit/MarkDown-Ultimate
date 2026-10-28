use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
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

    pub fn new_file_with_content(name: impl Into<String>, content: String) -> Self {
        let mut n = Self::new_file(name);
        if let Self::File { content: c, .. } = &mut n {
            *c = content;
        }
        n
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

    pub fn id(&self) -> &str {
        match self {
            Self::File { id, .. } | Self::Folder { id, .. } => id,
        }
    }

    pub fn touch(&mut self) {
        let now = Utc::now();
        match self {
            Self::File { updated_at, .. } | Self::Folder { updated_at, .. } => *updated_at = now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Workspace {
    pub root: Vec<Node>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum WorkspaceError {
    #[error("node not found")]
    NotFound,
    #[error("invalid operation")]
    Invalid,
    #[error("name already exists")]
    DuplicateName,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub id: String,
    pub path: String,
    pub updated_at: DateTime<Utc>,
}

impl Workspace {
    pub fn flatten_files_newest_first(&self) -> Vec<(String, String)> {
        self.flatten_file_entries()
            .into_iter()
            .map(|e| (e.id, e.path))
            .collect()
    }

    pub fn flatten_file_entries(&self) -> Vec<FileEntry> {
        let mut items = Vec::new();
        walk_entries(&self.root, String::new(), &mut items);
        items.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        items
    }

    pub fn sorted_root_nodes(&self) -> Vec<Node> {
        let mut nodes = self.root.clone();
        nodes.sort_by(|a, b| node_updated_at(b).cmp(&node_updated_at(a)));
        nodes
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }

    pub fn from_json(v: serde_json::Value) -> Self {
        serde_json::from_value(v).unwrap_or_default()
    }

    pub fn find_node_mut(&mut self, id: &str) -> Option<&mut Node> {
        find_in_nodes_mut(&mut self.root, id)
    }

    pub fn children_of(&self, id: &str) -> Option<Vec<Node>> {
        find_in_nodes(&self.root, id).and_then(|n| match n {
            Node::Folder { children, .. } => Some(children.clone()),
            Node::File { .. } => None,
        })
    }

    pub fn rename(&mut self, id: &str, new_name: &str) -> Result<(), WorkspaceError> {
        let new_name = new_name.trim();
        if new_name.is_empty() {
            return Err(WorkspaceError::Invalid);
        }
        let parent = parent_id_and_siblings(self, id)?;
        if sibling_name_exists(parent.1, new_name, Some(id)) {
            return Err(WorkspaceError::DuplicateName);
        }
        if let Some(n) = self.find_node_mut(id) {
            match n {
                Node::File { name, .. } | Node::Folder { name, .. } => {
                    *name = new_name.to_string();
                    n.touch();
                }
            }
            Ok(())
        } else {
            Err(WorkspaceError::NotFound)
        }
    }

    pub fn delete(&mut self, id: &str) -> Result<(), WorkspaceError> {
        if remove_from_nodes(&mut self.root, id) {
            Ok(())
        } else {
            Err(WorkspaceError::NotFound)
        }
    }

    pub fn create_folder(&mut self, parent_id: Option<&str>, name: &str) -> Result<String, WorkspaceError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(WorkspaceError::Invalid);
        }
        let folder = Node::new_folder(name);
        let id = folder.id().to_string();
        match parent_id {
            None => {
                if sibling_name_exists(&self.root, name, None) {
                    return Err(WorkspaceError::DuplicateName);
                }
                self.root.push(folder);
            }
            Some(pid) => {
                let children = if let Some(Node::Folder { children, .. }) = self.find_node_mut(pid) {
                    if sibling_name_exists(children, name, None) {
                        return Err(WorkspaceError::DuplicateName);
                    }
                    children
                } else {
                    return Err(WorkspaceError::NotFound);
                };
                children.push(folder);
                if let Some(p) = self.find_node_mut(pid) {
                    p.touch();
                }
            }
        }
        Ok(id)
    }

    pub fn add_file(&mut self, parent_id: Option<&str>, name: &str, content: String) -> Result<String, WorkspaceError> {
        let name = name.trim();
        if name.is_empty() {
            return Err(WorkspaceError::Invalid);
        }
        let file = Node::new_file_with_content(name, content);
        let id = file.id().to_string();
        match parent_id {
            None => {
                if sibling_name_exists(&self.root, name, None) {
                    return Err(WorkspaceError::DuplicateName);
                }
                self.root.push(file);
            }
            Some(pid) => {
                let node = self.find_node_mut(pid).ok_or(WorkspaceError::NotFound)?;
                let children = match node {
                    Node::Folder { children, .. } => children,
                    Node::File { .. } => return Err(WorkspaceError::Invalid),
                };
                if sibling_name_exists(children, name, None) {
                    return Err(WorkspaceError::DuplicateName);
                }
                children.push(file);
                node.touch();
            }
        }
        Ok(id)
    }

    /// Move node under `new_parent_id` (None = root). Reorder within same parent when `before_id` set.
    pub fn move_node(
        &mut self,
        id: &str,
        new_parent_id: Option<&str>,
        before_id: Option<&str>,
    ) -> Result<(), WorkspaceError> {
        if id == new_parent_id.unwrap_or("") {
            return Err(WorkspaceError::Invalid);
        }
        let mut detached = detach_node(&mut self.root, id).ok_or(WorkspaceError::NotFound)?;
        let name = match &detached {
            Node::File { name, .. } | Node::Folder { name, .. } => name.clone(),
        };
        let target = match new_parent_id {
            None => &mut self.root,
            Some(pid) => {
                let n = self.find_node_mut(pid).ok_or(WorkspaceError::NotFound)?;
                match n {
                    Node::Folder { children, .. } => children,
                    Node::File { .. } => return Err(WorkspaceError::Invalid),
                }
            }
        };
        if sibling_name_exists(target, &name, None) {
            target.push(detached);
            return Err(WorkspaceError::DuplicateName);
        }
        insert_before(target, detached, before_id);
        Ok(())
    }
}

fn node_updated_at(n: &Node) -> DateTime<Utc> {
    match n {
        Node::File { updated_at, .. } | Node::Folder { updated_at, .. } => *updated_at,
    }
}

fn walk_entries(nodes: &[Node], prefix: String, out: &mut Vec<FileEntry>) {
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
                out.push(FileEntry {
                    id: id.clone(),
                    path,
                    updated_at: *updated_at,
                });
            }
            Node::Folder { name, children, .. } => {
                let p = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{prefix}/{name}")
                };
                walk_entries(children, p, out);
            }
        }
    }
}

fn find_in_nodes<'a>(nodes: &'a [Node], id: &str) -> Option<&'a Node> {
    for n in nodes {
        if n.id() == id {
            return Some(n);
        }
        if let Node::Folder { children, .. } = n {
            if let Some(found) = find_in_nodes(children, id) {
                return Some(found);
            }
        }
    }
    None
}

fn find_in_nodes_mut<'a>(nodes: &'a mut [Node], id: &str) -> Option<&'a mut Node> {
    for n in nodes {
        if n.id() == id {
            return Some(n);
        }
        if let Node::Folder { children, .. } = n {
            if let Some(found) = find_in_nodes_mut(children, id) {
                return Some(found);
            }
        }
    }
    None
}

fn remove_from_nodes(nodes: &mut Vec<Node>, id: &str) -> bool {
    if let Some(pos) = nodes.iter().position(|n| n.id() == id) {
        nodes.remove(pos);
        return true;
    }
    for n in nodes.iter_mut() {
        if let Node::Folder { children, .. } = n {
            if remove_from_nodes(children, id) {
                n.touch();
                return true;
            }
        }
    }
    false
}

fn detach_node(nodes: &mut Vec<Node>, id: &str) -> Option<Node> {
    if let Some(pos) = nodes.iter().position(|n| n.id() == id) {
        return Some(nodes.remove(pos));
    }
    for n in nodes.iter_mut() {
        if let Node::Folder { children, .. } = n {
            if let Some(detached) = detach_node(children, id) {
                n.touch();
                return Some(detached);
            }
        }
    }
    None
}

fn insert_before(target: &mut Vec<Node>, node: Node, before_id: Option<&str>) {
    if let Some(bid) = before_id {
        if let Some(pos) = target.iter().position(|n| n.id() == bid) {
            target.insert(pos, node);
            return;
        }
    }
    target.push(node);
}

fn sibling_name_exists(siblings: &[Node], name: &str, ignore_id: Option<&str>) -> bool {
    siblings.iter().any(|n| {
        if Some(n.id()) == ignore_id {
            return false;
        }
        match n {
            Node::File { name: n, .. } | Node::Folder { name: n, .. } => n.eq_ignore_ascii_case(name),
        }
    })
}

fn parent_id_and_siblings<'a>(
    ws: &'a Workspace,
    id: &str,
) -> Result<(Option<String>, &'a [Node]), WorkspaceError> {
    if ws.root.iter().any(|n| n.id() == id) {
        return Ok((None, &ws.root));
    }
    fn walk<'a>(nodes: &'a [Node], target_id: &str) -> Option<(Option<String>, &'a [Node])> {
        for n in nodes {
            if let Node::Folder {
                id: folder_id,
                children,
                ..
            } = n
            {
                if children.iter().any(|c| c.id() == target_id) {
                    return Some((Some(folder_id.clone()), children.as_slice()));
                }
                if let Some(r) = walk(children, target_id) {
                    return Some(r);
                }
            }
        }
        None
    }
    walk(&ws.root, id).ok_or(WorkspaceError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newest_first_by_updated_at() {
        let mut ws = Workspace::default();
        let mut older = Node::new_file("a.md");
        let mut newer = Node::new_file("b.md");
        if let Node::File { updated_at, .. } = &mut older {
            *updated_at = Utc::now() - chrono::Duration::hours(2);
        }
        ws.root.push(older);
        ws.root.push(newer);
        let flat = ws.flatten_file_entries();
        assert_eq!(flat.len(), 2);
        assert!(flat[0].updated_at >= flat[1].updated_at);
    }

    #[test]
    fn rename_and_delete() {
        let mut ws = Workspace::default();
        let f = Node::new_file("x.md");
        let id = f.id().to_string();
        ws.root.push(f);
        ws.rename(&id, "y.md").unwrap();
        ws.delete(&id).unwrap();
        assert!(ws.root.is_empty());
    }

    #[test]
    fn create_folder_and_file_inside() {
        let mut ws = Workspace::default();
        let fid = ws.create_folder(None, "docs").unwrap();
        let file_id = ws.add_file(Some(&fid), "note.md", "# hi".into()).unwrap();
        assert_eq!(ws.flatten_files_newest_first().len(), 1);
        assert!(!file_id.is_empty());
    }
}
