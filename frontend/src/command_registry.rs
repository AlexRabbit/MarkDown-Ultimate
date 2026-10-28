//! Canonical editor command registry (single source for tools sidebar and menus).

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CommandKind {
    Wrap,
    Js,
    Modal,
}

#[derive(Clone, Copy)]
pub struct EditorCommand {
    pub id: &'static str,
    pub category: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
    pub kind: CommandKind,
    pub before: &'static str,
    pub after: &'static str,
    pub placeholder: &'static str,
    /// Js action name or modal id: `find-replace`, `shortcuts`, `table-add-row`, …
    pub action: &'static str,
}

pub const CATEGORIES: &[&str] = &[
    "Text Formatting",
    "Document Structure",
    "Lists",
    "Links",
    "Code",
    "Tables",
    "Math",
    "Diagrams",
    "Extended",
    "Document Navigation",
    "Editing Utilities",
];

pub const COMMANDS: &[EditorCommand] = &[
    EditorCommand { id: "bold", category: "Text Formatting", label: "Bold", shortcut: Some("Ctrl+B"), kind: CommandKind::Wrap, before: "**", after: "**", placeholder: "text", action: "" },
    EditorCommand { id: "italic", category: "Text Formatting", label: "Italic", shortcut: Some("Ctrl+I"), kind: CommandKind::Wrap, before: "*", after: "*", placeholder: "text", action: "" },
    EditorCommand { id: "bold-italic", category: "Text Formatting", label: "Bold and italic", shortcut: None, kind: CommandKind::Wrap, before: "***", after: "***", placeholder: "text", action: "" },
    EditorCommand { id: "strike", category: "Text Formatting", label: "Strikethrough", shortcut: None, kind: CommandKind::Wrap, before: "~~", after: "~~", placeholder: "text", action: "" },
    EditorCommand { id: "code", category: "Text Formatting", label: "Inline code", shortcut: None, kind: CommandKind::Wrap, before: "`", after: "`", placeholder: "code", action: "" },
    EditorCommand { id: "mark", category: "Text Formatting", label: "Highlight (==mark==)", shortcut: None, kind: CommandKind::Wrap, before: "==", after: "==", placeholder: "text", action: "" },
    EditorCommand { id: "spoiler", category: "Text Formatting", label: "Spoiler", shortcut: None, kind: CommandKind::Wrap, before: "||", after: "||", placeholder: "text", action: "" },
    EditorCommand { id: "underline", category: "Text Formatting", label: "Underline (HTML)", shortcut: None, kind: CommandKind::Wrap, before: "<u>", after: "</u>", placeholder: "text", action: "" },
    EditorCommand { id: "h1", category: "Document Structure", label: "Heading 1", shortcut: None, kind: CommandKind::Wrap, before: "# ", after: "\n", placeholder: "Title", action: "" },
    EditorCommand { id: "h2", category: "Document Structure", label: "Heading 2", shortcut: None, kind: CommandKind::Wrap, before: "## ", after: "\n", placeholder: "Section", action: "" },
    EditorCommand { id: "h3", category: "Document Structure", label: "Heading 3", shortcut: None, kind: CommandKind::Wrap, before: "### ", after: "\n", placeholder: "Subsection", action: "" },
    EditorCommand { id: "h4", category: "Document Structure", label: "Heading 4", shortcut: None, kind: CommandKind::Wrap, before: "#### ", after: "\n", placeholder: "Heading", action: "" },
    EditorCommand { id: "h5", category: "Document Structure", label: "Heading 5", shortcut: None, kind: CommandKind::Wrap, before: "##### ", after: "\n", placeholder: "Heading", action: "" },
    EditorCommand { id: "h6", category: "Document Structure", label: "Heading 6", shortcut: None, kind: CommandKind::Wrap, before: "###### ", after: "\n", placeholder: "Heading", action: "" },
    EditorCommand { id: "quote", category: "Document Structure", label: "Blockquote", shortcut: None, kind: CommandKind::Wrap, before: "> ", after: "\n", placeholder: "quote", action: "" },
    EditorCommand { id: "hr", category: "Document Structure", label: "Horizontal rule", shortcut: None, kind: CommandKind::Wrap, before: "\n---\n", after: "", placeholder: "", action: "" },
    EditorCommand { id: "bullet", category: "Lists", label: "Bulleted list", shortcut: None, kind: CommandKind::Wrap, before: "- ", after: "\n", placeholder: "item", action: "" },
    EditorCommand { id: "ordered", category: "Lists", label: "Numbered list", shortcut: None, kind: CommandKind::Wrap, before: "1. ", after: "\n", placeholder: "item", action: "" },
    EditorCommand { id: "task", category: "Lists", label: "Task list", shortcut: None, kind: CommandKind::Wrap, before: "- [ ] ", after: "\n", placeholder: "task", action: "" },
    EditorCommand { id: "link", category: "Links", label: "Insert link", shortcut: None, kind: CommandKind::Wrap, before: "[", after: "](https://)", placeholder: "label", action: "" },
    EditorCommand { id: "footnote", category: "Extended", label: "Footnote ref [^1]", shortcut: None, kind: CommandKind::Wrap, before: "[^", after: "]", placeholder: "1", action: "" },
    EditorCommand { id: "admonition-note", category: "Extended", label: "Callout (Note)", shortcut: None, kind: CommandKind::Wrap, before: "> [!NOTE]\n> ", after: "\n", placeholder: "Body", action: "" },
    EditorCommand { id: "fence", category: "Code", label: "Code fence", shortcut: None, kind: CommandKind::Wrap, before: "```\n", after: "\n```\n", placeholder: "code", action: "" },
    EditorCommand { id: "table", category: "Tables", label: "Insert table", shortcut: None, kind: CommandKind::Wrap, before: "| Col | Col |\n| --- | --- |\n| ", after: " |  |\n", placeholder: "", action: "" },
    EditorCommand { id: "table-row-below", category: "Tables", label: "Add row below", shortcut: None, kind: CommandKind::Js, before: "", after: "", placeholder: "", action: "table-row-below" },
    EditorCommand { id: "table-row-above", category: "Tables", label: "Add row above", shortcut: None, kind: CommandKind::Js, before: "", after: "", placeholder: "", action: "table-row-above" },
    EditorCommand { id: "table-col-right", category: "Tables", label: "Add column right", shortcut: None, kind: CommandKind::Js, before: "", after: "", placeholder: "", action: "table-col-right" },
    EditorCommand { id: "table-col-left", category: "Tables", label: "Add column left", shortcut: None, kind: CommandKind::Js, before: "", after: "", placeholder: "", action: "table-col-left" },
    EditorCommand { id: "math-inline", category: "Math", label: "Inline math", shortcut: None, kind: CommandKind::Wrap, before: "$", after: "$", placeholder: "E=mc^2", action: "" },
    EditorCommand { id: "math-block", category: "Math", label: "Display math", shortcut: None, kind: CommandKind::Wrap, before: "$$\n", after: "\n$$\n", placeholder: "E=mc^2", action: "" },
    EditorCommand { id: "mermaid", category: "Diagrams", label: "Mermaid block", shortcut: None, kind: CommandKind::Wrap, before: "```mermaid\nflowchart LR\n  A --> B\n", after: "\n```\n", placeholder: "", action: "" },
    EditorCommand { id: "details", category: "Extended", label: "Details", shortcut: None, kind: CommandKind::Wrap, before: "<details><summary>", after: "</summary>\n\nBody\n</details>\n", placeholder: "Title", action: "" },
    EditorCommand { id: "find-replace", category: "Editing Utilities", label: "Find and replace", shortcut: Some("Ctrl+F"), kind: CommandKind::Modal, before: "", after: "", placeholder: "", action: "find-replace" },
    EditorCommand { id: "shortcuts", category: "Editing Utilities", label: "Keyboard shortcuts", shortcut: Some("Ctrl+/"), kind: CommandKind::Modal, before: "", after: "", placeholder: "", action: "shortcuts" },
];

pub fn command_by_id(id: &str) -> Option<&'static EditorCommand> {
    COMMANDS.iter().find(|c| c.id == id)
}

pub fn shortcut_lines() -> Vec<(&'static str, &'static str)> {
    COMMANDS
        .iter()
        .filter_map(|c| c.shortcut.map(|s| (s, c.label)))
        .collect()
}
