use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use mu_diagnostics::{emit, recent_events, Severity};
use mu_markdown_core::render_markdown;
use mu_workflow::{export_container, import_container, WorkflowMeta, WORKFLOW_MAGIC};
use mu_workspace::{Node, Workspace};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, FileReader, RequestMode, Url};

use crate::command_registry::{command_by_id, shortcut_lines, CommandKind, EditorCommand};
use crate::editor_commands::{copy_text, find_replace, init_editor_shortcuts, jump_to_offset, print_markdown};
use crate::tools_sidebar::{
    load_tools_open, load_tools_width, run_command_on_editor, ToolsSidebar,
};

const STORAGE_KEY: &str = "mdu_workspace_v1";
const PULSE_KEY: &str = "mdu_howto_pulse_seen";
const TOUR_STEPS: &str = include_str!("../tour/registry.json");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn mduStartTour(steps_json: &str);
    #[wasm_bindgen(js_namespace = window)]
    fn mduRenderMath(selector: &str);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Markdown,
    Split,
    Preview,
}

const VIEW_MODE_KEY: &str = "mdu_view_mode_v1";

fn load_view_mode() -> ViewMode {
    LocalStorage::get::<String>(VIEW_MODE_KEY)
        .ok()
        .and_then(|s| parse_view_mode(&s))
        .unwrap_or(ViewMode::Split)
}

fn parse_view_mode(s: &str) -> Option<ViewMode> {
    match s {
        "markdown" => Some(ViewMode::Markdown),
        "split" => Some(ViewMode::Split),
        "preview" => Some(ViewMode::Preview),
        _ => None,
    }
}

fn view_mode_storage(mode: ViewMode) -> String {
    match mode {
        ViewMode::Markdown => "markdown".into(),
        ViewMode::Split => "split".into(),
        ViewMode::Preview => "preview".into(),
    }
}

fn view_class(mode: ViewMode) -> &'static str {
    match mode {
        ViewMode::Markdown => "markdown",
        ViewMode::Split => "split",
        ViewMode::Preview => "preview",
    }
}

fn export_filename(ws: &Workspace, active_id: &str) -> String {
    ws.flatten_file_entries()
        .into_iter()
        .find(|e| e.id == active_id)
        .map(|e| {
            e.path
                .rsplit('/')
                .next()
                .unwrap_or(&e.path)
                .to_string()
        })
        .unwrap_or_else(|| "document.md".into())
}

fn unique_import_name(ws: &Workspace, fname: &str) -> String {
    let mut name = if fname.is_empty() {
        "imported.md".to_string()
    } else if !fname.to_lowercase().ends_with(".md") && !fname.to_lowercase().ends_with(".markdown") {
        format!("{fname}.md")
    } else {
        fname.to_string()
    };
    let existing: std::collections::HashSet<String> = ws
        .flatten_file_entries()
        .into_iter()
        .map(|e| {
            e.path
                .rsplit('/')
                .next()
                .unwrap_or(&e.path)
                .to_string()
        })
        .collect();
    if !existing.contains(&name) {
        return name;
    }
    let stem = name
        .trim_end_matches(".markdown")
        .trim_end_matches(".md");
    for n in 1..1000 {
        let candidate = format!("{stem}-{n}.md");
        if !existing.contains(&candidate) {
            return candidate;
        }
    }
    format!("import-{}.md", short_id())
}

fn doc_stats(text: &str) -> (usize, usize) {
    let chars = text.chars().count();
    let words = text.split_whitespace().count();
    (words, chars)
}

fn scan_headings(text: &str) -> Vec<(u8, String, usize)> {
    let mut pos = 0usize;
    let mut out = Vec::new();
    for line in text.split('\n') {
        let trimmed = line.trim_start();
        let hashes = trimmed.chars().take_while(|&c| c == '#').count();
        if (1..=6).contains(&hashes) {
            let title_part = trimmed.get(hashes..).unwrap_or("").trim();
            if !title_part.is_empty() {
                out.push((hashes as u8, title_part.to_string(), pos));
            }
        }
        pos += line.len() + 1;
    }
    out
}

fn editor_body(
    mode: ViewMode,
    content: ReadSignal<String>,
    set_content: WriteSignal<String>,
    save: impl Fn(String) + Copy + 'static,
) -> AnyView {
    match mode {
        ViewMode::Markdown => view! {
            <div class="editor-surface source-only">
                <textarea id="mduSourceEditor" class="source" spellcheck="true"
                    prop:value=move || content.get()
                    on:input=move |ev| {
                        let ta: web_sys::HtmlTextAreaElement = event_target(&ev);
                        let v = ta.value();
                        set_content.set(v.clone());
                        save(v);
                    }></textarea>
            </div>
        }.into_any(),
        ViewMode::Split => view! {
            <div class="editor-surface split">
                <textarea id="mduSourceEditor" class="source inline" spellcheck="true"
                    prop:value=move || content.get()
                    on:input=move |ev| {
                        let ta: web_sys::HtmlTextAreaElement = event_target(&ev);
                        let v = ta.value();
                        set_content.set(v.clone());
                        save(v);
                    }></textarea>
                <div id="previewPane" class="preview" inner_html=move || render_markdown(&content.get())></div>
            </div>
        }.into_any(),
        ViewMode::Preview => view! {
            <div class="editor-surface preview-mode">
                <textarea id="mduSourceEditor" class="source preview-overlay" spellcheck="true"
                    prop:value=move || content.get()
                    on:input=move |ev| {
                        let ta: web_sys::HtmlTextAreaElement = event_target(&ev);
                        let v = ta.value();
                        set_content.set(v.clone());
                        save(v);
                    }></textarea>
                <div id="previewPane" class="preview preview-front" inner_html=move || render_markdown(&content.get())></div>
            </div>
        }.into_any(),
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ModalKind {
    None,
    Export,
    Import,
    Diagnostics,
    FindReplace,
    Shortcuts,
}

#[derive(Clone)]
enum MenuTarget {
    None,
    Editor,
    SidebarNode(String),
    SidebarRoot,
}

#[component]
pub fn App() -> impl IntoView {
    let (workspace, set_workspace) = signal(load_workspace());
    let (active_id, set_active_id) = signal(default_active(&workspace.get_untracked()));
    let (view_mode, set_view_mode) = signal(load_view_mode());
    let (modal, set_modal) = signal(ModalKind::None);
    let (content, set_content) = signal(String::new());
    let (show_pulse, set_show_pulse) = signal(!LocalStorage::get::<bool>(PULSE_KEY).unwrap_or(false));
    let (menu, set_menu) = signal((MenuTarget::None, 0i32, 0i32));
    let (drag_id, set_drag_id) = signal(String::new());
    let (engine_online, set_engine_online) = signal(false);
    let (tools_open, set_tools_open) = signal(load_tools_open());
    let (tools_width, set_tools_width) = signal(load_tools_width());

    if show_pulse.get_untracked() && !LocalStorage::get::<bool>(PULSE_KEY).unwrap_or(false) {
        schedule_pulse_stop(set_show_pulse);
    }

    Effect::new(move |_| {
        wasm_bindgen_futures::spawn_local(async move {
            set_engine_online.set(fetch_health().await);
        });
    });

    Effect::new(move || {
        if let Some(c) = find_content(&workspace.get(), &active_id.get()) {
            set_content.set(c);
        }
    });

    Effect::new(move || {
        let _ = content.get();
        let mode = view_mode.get();
        if mode == ViewMode::Split || mode == ViewMode::Preview {
            schedule_math_render();
        }
    });

    Effect::new(move |_| {
        if !engine_online.get() {
            return;
        }
        let ws = workspace.get();
        wasm_bindgen_futures::spawn_local(async move {
            let _ = push_workspace_to_engine(&ws).await;
        });
    });

    let persist = {
        let set_workspace = set_workspace;
        Callback::new(move |ws: Workspace| {
            match LocalStorage::set(STORAGE_KEY, serde_json::to_string(&ws).unwrap_or_default()) {
                Ok(()) => {}
                Err(e) => emit(
                    Severity::Warn,
                    "storage",
                    "local_storage_full",
                    "frontend",
                    format!("Could not save workspace: {e}"),
                    None,
                    None,
                ),
            }
            set_workspace.set(ws);
        })
    };

    let save_content = move |text: String| {
        let mut ws = workspace.get_untracked();
        if update_content(&mut ws, &active_id.get_untracked(), text) {
            persist.run(ws);
        }
    };

    let run_command_cb = {
        let save_content = save_content;
        Callback::new(move |cmd: EditorCommand| {
            if cmd.kind == CommandKind::Modal {
                match cmd.action {
                    "find-replace" => set_modal.set(ModalKind::FindReplace),
                    "shortcuts" => set_modal.set(ModalKind::Shortcuts),
                    _ => {}
                }
            } else {
                run_command_on_editor(cmd, set_content, save_content);
            }
        })
    };

    Effect::new({
        let save_content = save_content;
        let set_modal = set_modal;
        move |_| {
            init_editor_shortcuts();
            let window = web_sys::window().unwrap();
            let modal_closure = Closure::wrap(Box::new(move |ev: web_sys::CustomEvent| {
                if let Some(id) = ev.detail().as_string() {
                    match id.as_str() {
                        "find-replace" => set_modal.set(ModalKind::FindReplace),
                        "shortcuts" => set_modal.set(ModalKind::Shortcuts),
                        _ => {
                            if let Some(cmd) = command_by_id(&id) {
                                if cmd.kind == CommandKind::Modal {
                                    match cmd.action {
                                        "find-replace" => set_modal.set(ModalKind::FindReplace),
                                        "shortcuts" => set_modal.set(ModalKind::Shortcuts),
                                        _ => {}
                                    }
                                } else {
                                    run_command_on_editor(*cmd, set_content, save_content);
                                }
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            let _ = window.add_event_listener_with_callback("mdu-open-modal", modal_closure.as_ref().unchecked_ref());
            modal_closure.forget();

            let cmd_closure = Closure::wrap(Box::new(move |ev: web_sys::CustomEvent| {
                if let Some(id) = ev.detail().as_string() {
                    if let Some(cmd) = command_by_id(&id) {
                        if cmd.kind != CommandKind::Modal {
                            run_command_on_editor(*cmd, set_content, save_content);
                        }
                    }
                }
            }) as Box<dyn FnMut(_)>);
            let _ = window.add_event_listener_with_callback("mdu-run-command", cmd_closure.as_ref().unchecked_ref());
            cmd_closure.forget();
        }
    });

    let set_view = move |mode: ViewMode| {
        set_view_mode.set(mode);
        let _ = LocalStorage::set(VIEW_MODE_KEY, view_mode_storage(mode));
    };

    let start_tour = move |_| {
        set_show_pulse.set(false);
        let _ = LocalStorage::set(PULSE_KEY, true);
        emit(Severity::Info, "tour", "start", "frontend", "user started tour", None, None);
        mduStartTour(TOUR_STEPS);
    };

    view! {
        <div class="shell">
            <header class="topbar">
                <div class="actions">
                    <button id="btnCreate" type="button" data-tour-id="btnCreate" on:click=move |_| {
                        let mut ws = workspace.get_untracked();
                        match ws.add_file(None, &format!("Untitled-{}.md", short_id()), String::new()) {
                            Ok(id) => {
                                persist.run(ws);
                                set_active_id.set(id);
                                emit(Severity::Info, "workspace", "create", "frontend", "new file", None, None);
                            }
                            Err(e) => emit(Severity::Warn, "workspace", "create_failed", "frontend", e.to_string(), None, None),
                        }
                    }>"Create"</button>
                    <button id="btnOpen" type="button" data-tour-id="btnOpen" on:click=move |_| set_modal.set(ModalKind::Import)>"Open"</button>
                    <button id="btnSave" type="button" data-tour-id="btnSave" on:click=move |_| set_modal.set(ModalKind::Export)>"Save"</button>
                    <button id="btnDiagnostics" type="button" data-tour-id="btnDiagnostics" on:click=move |_| set_modal.set(ModalKind::Diagnostics)>"Diagnostics"</button>
                </div>
                <button id="btnHowTo" data-tour-id="btnHowTo" class=move || if show_pulse.get() { "howto pulse" } else { "howto" }
                    on:click=start_tour>
                    "How to Use"
                </button>
            </header>
            <div class="main">
                <aside class="sidebar" id="sidebarTree" data-tour-id="sidebarTree"
                    on:contextmenu=move |ev| {
                        ev.prevent_default();
                        set_menu.set((MenuTarget::SidebarRoot, ev.client_x(), ev.client_y()));
                    }>
                    <div class="sidebar-head">
                        <span>"Files"</span>
                        <button id="btnNewFolder" type="button" class="icon-btn" title="New folder" data-tour-id="btnNewFolder"
                            on:click=move |_| {
                                let mut ws = workspace.get_untracked();
                                let name = ws.unique_sibling_name(None, "New folder");
                                match ws.create_folder(None, &name) {
                                    Ok(_) => {
                                        persist.run(ws);
                                        emit(Severity::Info, "workspace", "folder_create", "frontend", &name, None, None);
                                    }
                                    Err(e) => emit(Severity::Warn, "workspace", "folder_create_failed", "frontend", e.to_string(), None, None),
                                }
                            }>"+"</button>
                    </div>
                    <TreeView
                        nodes=Signal::derive(move || workspace.get().sorted_root_nodes())
                        active_id=active_id
                        set_active_id=set_active_id
                        content=content
                        save_content=save_content
                        workspace=workspace
                        persist=persist.clone()
                        drag_id=drag_id
                        set_drag_id=set_drag_id
                        set_menu=set_menu
                        parent_id=None
                    />
                    <OutlinePanel content=content />
                </aside>
                <div class="center-column">
                <section class="editor-pane">
                    <div class="view-pill" role="tablist" aria-label="Editor view mode">
                        <button type="button" id="modeMarkdown" data-tour-id="modeSource"
                            class=move || if view_mode.get()==ViewMode::Markdown {"pill-seg active"} else {"pill-seg"}
                            on:click=move |_| set_view(ViewMode::Markdown)>"Markdown"</button>
                        <button type="button" id="modeBoth" data-tour-id="modeBoth"
                            class=move || if view_mode.get()==ViewMode::Split {"pill-seg active"} else {"pill-seg"}
                            on:click=move |_| set_view(ViewMode::Split)>"Both"</button>
                        <button type="button" id="modePreviewOnly" data-tour-id="modePreviewOnly"
                            class=move || if view_mode.get()==ViewMode::Preview {"pill-seg active"} else {"pill-seg"}
                            on:click=move |_| set_view(ViewMode::Preview)>"Preview"</button>
                    </div>
                    <div class="editor-row">
                    <div class=move || format!("editor view-{}", view_class(view_mode.get())) id="editorArea" data-tour-id="editorArea">
                        {move || editor_body(view_mode.get(), content, set_content, save_content)}
                    </div>
                    <ToolsSidebar open=tools_open set_open=set_tools_open width_px=tools_width set_width_px=set_tools_width run_command_cb=run_command_cb />
                    </div>
                    <div class="status-bar" id="docStatusBar" data-tour-id="docStatusBar">
                        {move || {
                            let (words, chars) = doc_stats(&content.get());
                            format!("{words} words · {chars} characters · Ctrl+F find · Ctrl+/ shortcuts")
                        }}
                    </div>
                </section>
                </div>
            </div>
            {move || context_menu_view(menu.get(), set_menu, workspace, persist.clone())}
            {move || modal_view(modal.get(), set_modal, workspace, active_id, view_mode, content, set_content, save_content, persist.clone(), set_active_id, set_view_mode)}
        </div>
    }
}

#[component]
fn OutlinePanel(content: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="outline-panel" id="docOutline" data-tour-id="docOutline">
            <div class="outline-head">"Outline"</div>
            <ul class="outline-list">
                {move || {
                    scan_headings(&content.get()).into_iter().map(|(level, title, offset)| {
                        let off = offset as u32;
                        view! {
                            <li class="outline-item" style=format!("padding-left:{}rem", (level as f32 - 1.0) * 0.65 + 0.35)>
                                <button type="button" class="outline-link"
                                    on:click=move |_| jump_to_offset(off)>
                                    {title}
                                </button>
                            </li>
                        }
                    }).collect_view()
                }}
            </ul>
        </div>
    }
}

#[component]
fn TreeView(
    nodes: Signal<Vec<Node>>,
    active_id: ReadSignal<String>,
    set_active_id: WriteSignal<String>,
    content: ReadSignal<String>,
    save_content: impl Fn(String) + Copy + Send + 'static,
    workspace: ReadSignal<Workspace>,
    persist: Callback<Workspace>,
    drag_id: ReadSignal<String>,
    set_drag_id: WriteSignal<String>,
    set_menu: WriteSignal<(MenuTarget, i32, i32)>,
    parent_id: Option<String>,
) -> impl IntoView {
    view! {
        <ul class="file-list">
            <For
                each=move || nodes.get()
                key=|node| node.id().to_string()
                let:node
            >
                <SidebarNodeView
                    node=node.clone()
                    active_id=active_id
                    set_active_id=set_active_id
                    content=content
                    save_content=save_content
                    workspace=workspace
                    persist=persist.clone()
                    drag_id=drag_id
                    set_drag_id=set_drag_id
                    set_menu=set_menu
                    parent_id=parent_id.clone()
                />
            </For>
        </ul>
    }
}

#[component]
fn SidebarNodeView(
    node: Node,
    active_id: ReadSignal<String>,
    set_active_id: WriteSignal<String>,
    content: ReadSignal<String>,
    save_content: impl Fn(String) + Copy + Send + 'static,
    workspace: ReadSignal<Workspace>,
    persist: Callback<Workspace>,
    drag_id: ReadSignal<String>,
    set_drag_id: WriteSignal<String>,
    set_menu: WriteSignal<(MenuTarget, i32, i32)>,
    parent_id: Option<String>,
) -> AnyView {
    let id = node.id().to_string();
    let id_click = id.clone();
    let id_ctx = id.clone();
    let id_drag = id.clone();
    let id_drop = id.clone();
    let parent_for_drop = parent_id.clone();
    let id_for_class = id.clone();
    let id_data = id.clone();
    match node {
        Node::File { name, .. } => view! {
            <li class=move || if active_id.get() == id_for_class { "row active" } else { "row" }
                draggable="true"
                data-id=id_data
                on:click=move |_| {
                    save_content(content.get_untracked());
                    set_active_id.set(id_click.clone());
                }
                on:contextmenu=move |ev| {
                    ev.prevent_default();
                    ev.stop_propagation();
                    set_menu.set((MenuTarget::SidebarNode(id_ctx.clone()), ev.client_x(), ev.client_y()));
                }
                on:dragstart=move |ev| {
                    set_drag_id.set(id_drag.clone());
                    if let Some(dt) = ev.data_transfer() { let _ = dt.set_data("text/plain", &id_drag); }
                }
                on:dragover=move |ev| ev.prevent_default()
                on:drop=move |ev| {
                    ev.prevent_default();
                    let dragged = drag_id.get();
                    if dragged.is_empty() { return; }
                    let mut ws = workspace.get_untracked();
                    let before = Some(id_drop.as_str());
                    if ws.move_node(&dragged, parent_for_drop.as_deref(), before).is_ok() {
                        persist.run(ws);
                    }
                }
            >{name}</li>
        }.into_any(),
        Node::Folder { name, id: fid, .. } => {
            let folder_id_drop = fid.clone();
            let folder_id_tree = fid.clone();
            let folder_id_menu = fid.clone();
            let folder_id_drag = fid.clone();
            let folder_id_parent = fid.clone();
            view! {
                <li class="folder-block">
                    <div class="row folder"
                        draggable="true"
                        on:dragstart=move |ev| {
                            set_drag_id.set(folder_id_drag.clone());
                            if let Some(dt) = ev.data_transfer() { let _ = dt.set_data("text/plain", &folder_id_drag); }
                        }
                        on:dragover=move |ev| ev.prevent_default()
                        on:drop=move |ev| {
                            ev.prevent_default();
                            let dragged = drag_id.get();
                            if dragged.is_empty() { return; }
                            let mut ws = workspace.get_untracked();
                            if ws.move_node(&dragged, Some(&folder_id_drop), None).is_ok() {
                                persist.run(ws);
                            }
                        }
                    >
                        <button type="button" class="kebab" aria-label="Folder menu"
                            on:click=move |ev| {
                                ev.stop_propagation();
                                set_menu.set((MenuTarget::SidebarNode(folder_id_menu.clone()), ev.client_x(), ev.client_y()));
                            }>"⋮"</button>
                        <span class="folder-name">"📁 " {name.clone()}</span>
                    </div>
                    <TreeView
                        nodes=Signal::derive(move || {
                            workspace.get().sorted_children_of(&folder_id_tree).unwrap_or_default()
                        })
                        active_id=active_id
                        set_active_id=set_active_id
                        content=content
                        save_content=save_content
                        workspace=workspace
                        persist=persist.clone()
                        drag_id=drag_id
                        set_drag_id=set_drag_id
                        set_menu=set_menu
                        parent_id=Some(folder_id_parent.clone())
                    />
                </li>
            }.into_any()
        }
    }
}

fn context_menu_view(
    menu: (MenuTarget, i32, i32),
    set_menu: WriteSignal<(MenuTarget, i32, i32)>,
    workspace: ReadSignal<Workspace>,
    persist: Callback<Workspace>,
) -> AnyView {
    let (target, x, y) = menu;
    if matches!(target, MenuTarget::None | MenuTarget::Editor) {
        return view! { <span class="hidden"></span> }.into_any();
    }
    let close = move |_| set_menu.set((MenuTarget::None, 0, 0));
    match target {
        MenuTarget::SidebarRoot | MenuTarget::SidebarNode(_) => {
            let node_id = match &target {
                MenuTarget::SidebarNode(id) => Some(id.clone()),
                _ => None,
            };
            let parent_for_folder = node_id.clone();
            view! {
                <div class="ctx-menu" style=format!("left:{x}px;top:{y}px") on:click=|ev| ev.stop_propagation()>
                    <button type="button" on:click=move |_| {
                        let mut ws = workspace.get_untracked();
                        let parent = parent_for_folder.as_deref();
                        let name = ws.unique_sibling_name(parent, "New folder");
                        match ws.create_folder(parent, &name) {
                            Ok(_) => persist.run(ws),
                            Err(e) => emit(Severity::Warn, "workspace", "folder_create_failed", "frontend", e.to_string(), None, None),
                        }
                        close(());
                    }>"New folder"</button>
                    {match node_id.clone() {
                        Some(nid) => {
                            let nid_ren = nid.clone();
                            let nid_del = nid.clone();
                            view! {
                            <>
                                <button type="button" on:click=move |_| {
                                    if let Some(new_name) = window_prompt("Rename", "Name") {
                                        let mut ws = workspace.get_untracked();
                                        if ws.rename(&nid_ren, &new_name).is_ok() { persist.run(ws); }
                                    }
                                    close(());
                                }>"Rename"</button>
                                <button type="button" on:click=move |_| {
                                    if window_confirm("Delete this item?") {
                                        let mut ws = workspace.get_untracked();
                                        if ws.delete(&nid_del).is_ok() { persist.run(ws); }
                                    }
                                    close(());
                                }>"Delete"</button>
                            </>
                        }.into_any()
                        }
                        None => view! { <span></span> }.into_any(),
                    }}
                </div>
            }.into_any()
        }
        MenuTarget::None | MenuTarget::Editor => view! { <span class="hidden"></span> }.into_any(),
    }
}

fn modal_view(
    kind: ModalKind,
    set_modal: WriteSignal<ModalKind>,
    workspace: ReadSignal<Workspace>,
    active_id: ReadSignal<String>,
    view_mode: ReadSignal<ViewMode>,
    content: ReadSignal<String>,
    set_content: WriteSignal<String>,
    save_content: impl Fn(String) + Copy + Send + 'static,
    persist: Callback<Workspace>,
    set_active_id: WriteSignal<String>,
    set_view_mode: WriteSignal<ViewMode>,
) -> AnyView {
    match kind {
        ModalKind::None => view! { <span class="hidden"></span> }.into_any(),
        ModalKind::Export => view! {
            <div class="modal-backdrop" role="dialog" aria-modal="true" on:click=move |_| set_modal.set(ModalKind::None)>
                <div class="modal" on:click=|ev| ev.stop_propagation()>
                    <h2>"Export"</h2>
                    <button type="button" on:click=move |_| {
                        let ws = workspace.get_untracked();
                        let name = export_filename(&ws, &active_id.get_untracked());
                        trigger_download(&name, &content.get_untracked(), "text/markdown");
                        emit(Severity::Info, "export", "markdown", "frontend", "md export", None, None);
                        set_modal.set(ModalKind::None);
                    }>"Markdown (.md)"</button>
                    <button type="button" on:click=move |_| {
                        let ws = workspace.get_untracked();
                        let title = export_filename(&ws, &active_id.get_untracked());
                        let html = render_markdown(&content.get_untracked());
                        set_modal.set(ModalKind::None);
                        if !print_markdown(&title, &html) {
                            emit(Severity::Warn, "export", "pdf_blocked", "frontend", "Allow pop-ups to print or save as PDF", None, None);
                        } else {
                            emit(Severity::Info, "export", "pdf", "frontend", "print document", None, None);
                        }
                    }>"PDF (print dialog)"</button>
                    <button type="button" on:click=move |_| {
                        let ws = workspace.get_untracked();
                        let meta = WorkflowMeta {
                            active_file_id: Some(active_id.get_untracked()),
                            view_mode: Some(view_mode_storage(view_mode.get_untracked())),
                        };
                        if let Ok(raw) = export_container(ws.to_json(), meta) {
                            trigger_download("MarkDown-Ultimate.mdu.json", &raw, "application/json");
                            emit(Severity::Info, "export", "workflow", "frontend", "workflow export", None, None);
                        }
                        set_modal.set(ModalKind::None);
                    }>"WorkFlow container (.mdu.json)"</button>
                </div>
            </div>
        }.into_any(),
        ModalKind::Import => view! {
            <div class="modal-backdrop" role="dialog" aria-modal="true" on:click=move |_| set_modal.set(ModalKind::None)>
                <div class="modal" on:click=|ev| ev.stop_propagation()>
                    <h2>"Import"</h2>
                    <p>"Choose a .md file or .mdu.json WorkFlow container."</p>
                    <input type="file" accept=".md,.markdown,.mdu.json,.json,text/markdown,application/json"
                        on:change=move |ev| {
                            let input: web_sys::HtmlInputElement = event_target(&ev);
                            if let Some(file) = input.files().and_then(|f| f.get(0)) {
                                let fname = file.name();
                                let current = workspace.get_untracked();
                                wasm_bindgen_futures::spawn_local(async move {
                                    match read_file_text(file).await {
                                        Ok(text) => handle_import_text(
                                            text,
                                            fname,
                                            persist.clone(),
                                            set_active_id,
                                            set_view_mode,
                                            current,
                                        ),
                                        Err(e) => emit(Severity::Error, "import", "read_failed", "frontend", e, None, None),
                                    }
                                });
                            }
                            set_modal.set(ModalKind::None);
                        } />
                </div>
            </div>
        }.into_any(),
        ModalKind::Diagnostics => {
            let lines = recent_events(80);
            let log_text = lines.join("\n");
            view! {
                <div class="modal-backdrop" role="dialog" aria-modal="true" on:click=move |_| set_modal.set(ModalKind::None)>
                    <div class="modal modal-wide" on:click=|ev| ev.stop_propagation()>
                        <h2>"Diagnostics"</h2>
                        <pre class="diag-log" id="diagLogPre">{log_text.clone()}</pre>
                        <div class="modal-actions">
                        <button type="button" on:click={
                            let log_copy = log_text.clone();
                            move |_| copy_text_to_clipboard(&log_copy)
                        }>"Copy all"</button>
                        <button type="button" on:click={
                            let log_export = log_text.clone();
                            move |_| trigger_download("events-export.jsonl", &log_export, "application/x-ndjson")
                        }>"Export log"</button>
                        </div>
                    </div>
                </div>
            }.into_any()
        }
        ModalKind::FindReplace => view! {
            <FindReplaceModal set_modal=set_modal set_content=set_content save_content=save_content />
        }.into_any(),
        ModalKind::Shortcuts => {
            let rows = shortcut_lines();
            view! {
                <div class="modal-backdrop" role="dialog" aria-modal="true" on:click=move |_| set_modal.set(ModalKind::None)>
                    <div class="modal modal-wide" on:click=|ev| ev.stop_propagation()>
                        <h2>"Keyboard shortcuts"</h2>
                        <ul class="shortcut-list">
                            {rows.into_iter().map(|(key, label)| view! {
                                <li><code>{key}</code>" — "{label}</li>
                            }).collect_view()}
                            <li><code>"Ctrl+F"</code>" — Find and replace dialog"</li>
                        </ul>
                        <button type="button" on:click=move |_| set_modal.set(ModalKind::None)>"Close"</button>
                    </div>
                </div>
            }.into_any()
        }
    }
}

#[component]
fn FindReplaceModal(
    set_modal: WriteSignal<ModalKind>,
    set_content: WriteSignal<String>,
    save_content: impl Fn(String) + Copy + Send + 'static,
) -> impl IntoView {
    let (find, set_find) = signal(String::new());
    let (replace, set_replace) = signal(String::new());
    view! {
        <div class="modal-backdrop" role="dialog" aria-modal="true" on:click=move |_| set_modal.set(ModalKind::None)>
            <div class="modal" on:click=|ev| ev.stop_propagation()>
                <h2>"Find and replace"</h2>
                <label>"Find"<input type="text" prop:value=move || find.get()
                    on:input=move |ev| set_find.set(event_target_value(&ev)) /></label>
                <label>"Replace"<input type="text" prop:value=move || replace.get()
                    on:input=move |ev| set_replace.set(event_target_value(&ev)) /></label>
                <div class="modal-actions">
                    <button type="button" on:click=move |_| {
                        if let Some(v) = find_replace(&find.get_untracked(), &replace.get_untracked(), false) {
                            set_content.set(v.clone());
                            save_content(v);
                        }
                    }>"Replace next"</button>
                    <button type="button" on:click=move |_| {
                        if let Some(v) = find_replace(&find.get_untracked(), &replace.get_untracked(), true) {
                            set_content.set(v.clone());
                            save_content(v);
                            set_modal.set(ModalKind::None);
                        }
                    }>"Replace all"</button>
                    <button type="button" on:click=move |_| set_modal.set(ModalKind::None)>"Close"</button>
                </div>
            </div>
        </div>
    }
}

fn handle_import_text(
    text: String,
    fname: String,
    persist: Callback<Workspace>,
    set_active_id: WriteSignal<String>,
    set_view_mode: WriteSignal<ViewMode>,
    current: Workspace,
) {
    let lower = fname.to_lowercase();
    if lower.ends_with(".mdu.json") || (lower.ends_with(".json") && text.contains(WORKFLOW_MAGIC)) {
        match import_container(&text) {
            Ok((tree, meta)) => {
                let ws = Workspace::from_json(tree);
                persist.run(ws.clone());
                if let Some(id) = meta.active_file_id.filter(|id| find_content(&ws, id).is_some()) {
                    set_active_id.set(id);
                } else if let Some((id, _)) = ws.flatten_files_newest_first().first().cloned() {
                    set_active_id.set(id);
                }
                if let Some(vm) = meta.view_mode.as_deref() {
                    if let Some(mode) = parse_view_mode(vm) {
                        set_view_mode.set(mode);
                        let _ = LocalStorage::set(VIEW_MODE_KEY, vm);
                    }
                }
                emit(Severity::Info, "import", "workflow", "frontend", "workflow imported", None, None);
            }
            Err(e) => emit(Severity::Error, "import", "workflow_invalid", "frontend", e.to_string(), None, None),
        }
    } else {
        let mut ws = current;
        let name = unique_import_name(&ws, &fname);
        if let Ok(id) = ws.add_file(None, &name, text) {
            persist.run(ws);
            set_active_id.set(id);
            emit(Severity::Info, "import", "markdown", "frontend", "md imported", None, None);
        }
    }
}

async fn read_file_text(file: web_sys::File) -> Result<String, String> {
    let reader = FileReader::new().map_err(|_| "FileReader unavailable")?;
    reader.read_as_text(&file).map_err(|_| "read failed")?;
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let reader_c = reader.clone();
        let onload = Closure::wrap(Box::new(move || {
            match reader_c.result() {
                Ok(v) => {
                    if let Some(s) = v.as_string() {
                        let _ = resolve.call1(&JsValue::NULL, &JsValue::from_str(&s));
                    } else {
                        let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("not text"));
                    }
                }
                Err(_) => {
                    let _ = reject.call1(&JsValue::NULL, &JsValue::from_str("read error"));
                }
            }
        }) as Box<dyn FnMut()>);
        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
    });
    JsFuture::from(promise)
        .await
        .map_err(|_| "async read failed".to_string())
        .and_then(|v| v.as_string().ok_or_else(|| "empty".to_string()))
}

fn load_workspace() -> Workspace {
    LocalStorage::get::<String>(STORAGE_KEY)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| {
            let mut ws = Workspace::default();
            ws.root.push(Node::new_file("Welcome.md"));
            ws
        })
}

fn default_active(ws: &Workspace) -> String {
    ws.flatten_files_newest_first()
        .first()
        .map(|(id, _)| id.clone())
        .unwrap_or_default()
}

fn find_content(ws: &Workspace, id: &str) -> Option<String> {
    let mut ws = ws.clone();
    find_file_mut(&mut ws, id).map(|(_, c)| c.clone())
}

fn find_file_mut<'a>(ws: &'a mut Workspace, id: &str) -> Option<(&'a str, &'a mut String)> {
    fn walk<'a>(nodes: &'a mut [Node], id: &str) -> Option<(&'a str, &'a mut String)> {
        for n in nodes {
            match n {
                Node::File { id: fid, content, .. } if fid == id => return Some((fid.as_str(), content)),
                Node::Folder { children, .. } => {
                    if let Some(r) = walk(children, id) {
                        return Some(r);
                    }
                }
                _ => {}
            }
        }
        None
    }
    walk(&mut ws.root, id)
}

fn update_content(ws: &mut Workspace, id: &str, text: String) -> bool {
    if let Some((_, c)) = find_file_mut(ws, id) {
        *c = text;
        if let Some(n) = ws.find_node_mut(id) {
            n.touch();
        }
        true
    } else {
        false
    }
}

fn short_id() -> String {
    uuid::Uuid::new_v4().to_string()[..8].to_string()
}

fn trigger_download(name: &str, data: &str, mime: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let arr = js_sys::Array::new();
    arr.push(&JsValue::from_str(data));
    let blob = Blob::new_with_blob_sequence_and_options(&arr, BlobPropertyBag::new().type_(mime)).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    let a = document.create_element("a").unwrap();
    let _ = a.set_attribute("href", &url);
    let _ = a.set_attribute("download", name);
    let a: web_sys::HtmlElement = a.dyn_into().unwrap();
    a.click();
    Url::revoke_object_url(&url).unwrap();
}

fn schedule_pulse_stop(set_show_pulse: WriteSignal<bool>) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let closure = Closure::wrap(Box::new(move || {
        set_show_pulse.set(false);
        let _ = LocalStorage::set(PULSE_KEY, true);
    }) as Box<dyn FnMut()>);
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        60_000,
    );
    closure.forget();
}

fn schedule_math_render() {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    let closure = Closure::wrap(Box::new(move || {
        mduRenderMath("#previewPane");
    }) as Box<dyn FnMut()>);
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        30,
    );
    closure.forget();
}

fn window_prompt(title: &str, default: &str) -> Option<String> {
    web_sys::window()?.prompt_with_message_and_default(title, default).ok().flatten()
}

fn window_confirm(msg: &str) -> bool {
    web_sys::window()
        .and_then(|w| w.confirm_with_message(msg).ok())
        .unwrap_or(false)
}

fn copy_text_to_clipboard(text: &str) {
    if !copy_text(text) {
        emit(
            Severity::Warn,
            "clipboard",
            "copy_failed",
            "frontend",
            "Could not copy to clipboard",
            None,
            None,
        );
    }
}

async fn fetch_health() -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };
    let opts = web_sys::RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request = match web_sys::Request::new_with_str_and_init("/health", &opts) {
        Ok(r) => r,
        Err(_) => return false,
    };
    let resp = match wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await {
        Ok(v) => match v.dyn_into::<web_sys::Response>() {
            Ok(r) if r.ok() => r,
            _ => return false,
        },
        Err(_) => return false,
    };
    let ct = resp
        .headers()
        .get("content-type")
        .ok()
        .flatten()
        .unwrap_or_default();
    if !ct.contains("json") {
        return false;
    }
    match wasm_bindgen_futures::JsFuture::from(resp.json().unwrap()).await {
        Ok(v) => v
            .dyn_into::<js_sys::Object>()
            .ok()
            .and_then(|o| js_sys::Reflect::get(&o, &JsValue::from_str("status")).ok())
            .and_then(|s| s.as_string())
            .as_deref()
            == Some("ok"),
        Err(_) => false,
    }
}

async fn push_workspace_to_engine(ws: &Workspace) -> Result<(), String> {
    let body = serde_json::to_string(ws).map_err(|e| e.to_string())?;
    let mut opts = web_sys::RequestInit::new();
    opts.set_method("PUT");
    opts.set_mode(RequestMode::Cors);
    opts.set_body(&JsValue::from_str(&body));
    let request = web_sys::Request::new_with_str_and_init("/api/workspace", &opts)
        .map_err(|_| "request failed")?;
    let window = web_sys::window().ok_or("no window")?;
    let resp = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "fetch failed")?;
    let resp: web_sys::Response = resp.dyn_into().map_err(|_| "bad response")?;
    if resp.ok() { Ok(()) } else { Err("engine rejected".into()) }
}

fn event_target<T: JsCast>(ev: &web_sys::Event) -> T {
    ev.target().unwrap().unchecked_into()
}
