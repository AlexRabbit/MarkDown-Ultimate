use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use mu_diagnostics::{emit, recent_events, Severity};
use mu_markdown_core::render_markdown;
use mu_workflow::{export_container, import_container};
use mu_workspace::{Node, Workspace};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, BlobPropertyBag, FileReader, RequestMode, Url};

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
enum EditorMode {
    Rendered,
    Source,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ModalKind {
    None,
    Export,
    Import,
    Diagnostics,
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
    let (editor_mode, set_editor_mode) = signal(EditorMode::Rendered);
    let (modal, set_modal) = signal(ModalKind::None);
    let (content, set_content) = signal(String::new());
    let (show_pulse, set_show_pulse) = signal(!LocalStorage::get::<bool>(PULSE_KEY).unwrap_or(false));
    let (menu, set_menu) = signal((MenuTarget::None, 0i32, 0i32));
    let (drag_id, set_drag_id) = signal(String::new());
    let (engine_online, set_engine_online) = signal(false);

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
        if editor_mode.get() == EditorMode::Rendered {
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
            let _ = LocalStorage::set(STORAGE_KEY, serde_json::to_string(&ws).unwrap_or_default());
            set_workspace.set(ws);
        })
    };

    let save_content = move |text: String| {
        let mut ws = workspace.get_untracked();
        if update_content(&mut ws, &active_id.get_untracked(), text) {
            persist.run(ws);
        }
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
                    <TreeView
                        nodes=Signal::derive(move || workspace.get().sorted_root_nodes())
                        active_id=active_id
                        set_active_id=set_active_id
                        workspace=workspace
                        persist=persist.clone()
                        drag_id=drag_id
                        set_drag_id=set_drag_id
                        set_menu=set_menu
                        parent_id=None
                    />
                </aside>
                <section class="editor-pane">
                    <div class="mode-toggle">
                        <button id="modePreview" type="button" data-tour-id="modePreview"
                            class=move || if editor_mode.get()==EditorMode::Rendered {"active"} else {""}
                            on:click=move |_| set_editor_mode.set(EditorMode::Rendered)>"Preview"</button>
                        <button id="modeSource" type="button" data-tour-id="modeSource"
                            class=move || if editor_mode.get()==EditorMode::Source {"active"} else {""}
                            on:click=move |_| set_editor_mode.set(EditorMode::Source)>"Markdown"</button>
                    </div>
                    <div class="editor" id="editorArea" data-tour-id="editorArea"
                        on:contextmenu=move |ev| {
                            ev.prevent_default();
                            set_menu.set((MenuTarget::Editor, ev.client_x(), ev.client_y()));
                        }>
                        <menu class="ctx" id="formatBar" data-tour-id="formatBar">
                            {snippet_buttons(set_content, save_content)}
                        </menu>
                        {move || if editor_mode.get() == EditorMode::Source {
                            view! {
                                <textarea class="source" prop:value=move || content.get()
                                    on:input=move |ev| {
                                        let v = event_target_value(&ev);
                                        set_content.set(v.clone());
                                        save_content(v);
                                    } />
                            }.into_any()
                        } else {
                            view! {
                                <div class="split">
                                    <textarea class="source inline" prop:value=move || content.get()
                                        on:input=move |ev| {
                                            let v = event_target_value(&ev);
                                            set_content.set(v.clone());
                                            save_content(v);
                                        } />
                                    <div class="preview" id="previewPane" inner_html=move || render_markdown(&content.get()) />
                                </div>
                            }.into_any()
                        }}
                    </div>
                </section>
            </div>
            {move || context_menu_view(menu.get(), set_menu, workspace, persist.clone(), set_content, save_content)}
            {move || modal_view(modal.get(), set_modal, workspace, content, persist.clone(), set_active_id)}
        </div>
    }
}

#[component]
fn TreeView(
    nodes: Signal<Vec<Node>>,
    active_id: ReadSignal<String>,
    set_active_id: WriteSignal<String>,
    workspace: ReadSignal<Workspace>,
    persist: Callback<Workspace>,
    drag_id: ReadSignal<String>,
    set_drag_id: WriteSignal<String>,
    set_menu: WriteSignal<(MenuTarget, i32, i32)>,
    parent_id: Option<String>,
) -> impl IntoView {
    view! {
        <ul class="file-list">
            {move || nodes.get().into_iter().map(|node| {
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
                            on:click=move |_| set_active_id.set(id_click.clone())
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
                        let folder_id = fid.clone();
                        let folder_id_drop = fid.clone();
                        let folder_id_tree = fid.clone();
                        view! {
                            <li class="folder-block">
                                <div class=move || if active_id.get() == id { "row folder active" } else { "row folder" }
                                    on:contextmenu=move |ev| {
                                        ev.prevent_default();
                                        ev.stop_propagation();
                                        set_menu.set((MenuTarget::SidebarNode(id_ctx.clone()), ev.client_x(), ev.client_y()));
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
                                >"📁 " {name.clone()}</div>
                                <TreeView
                                    nodes=Signal::derive(move || {
                                        workspace.get().children_of(&folder_id_tree).unwrap_or_default()
                                    })
                                    active_id=active_id
                                    set_active_id=set_active_id
                                    workspace=workspace
                                    persist=persist.clone()
                                    drag_id=drag_id
                                    set_drag_id=set_drag_id
                                    set_menu=set_menu
                                    parent_id=Some(folder_id.clone())
                                />
                            </li>
                        }.into_any()
                    }
                }
            }).collect_view()}
        </ul>
    }
}

fn context_menu_view(
    menu: (MenuTarget, i32, i32),
    set_menu: WriteSignal<(MenuTarget, i32, i32)>,
    workspace: ReadSignal<Workspace>,
    persist: Callback<Workspace>,
    set_content: WriteSignal<String>,
    save_content: impl Fn(String) + Copy + 'static,
) -> AnyView {
    let (target, x, y) = menu;
    if matches!(target, MenuTarget::None) {
        return view! { <span class="hidden"></span> }.into_any();
    }
    let close = move |_| set_menu.set((MenuTarget::None, 0, 0));
    match target {
        MenuTarget::Editor => view! {
            <div class="ctx-menu" style=format!("left:{x}px;top:{y}px") on:click=|ev| ev.stop_propagation()>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "# Heading\n"); close(()); }>"Heading"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "**bold** "); close(()); }>"Bold"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "*italic* "); close(()); }>"Italic"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "||spoiler|| "); close(()); }>"Spoiler"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "==mark== "); close(()); }>"Mark"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "$$\\nE=mc^2\\n$$\\n"); close(()); }>"Math block"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "| H | V |\\n|---|---|\\n| | |\\n"); close(()); }>"Table"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "- [ ] task\\n"); close(()); }>"Task"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "> quote\\n"); close(()); }>"Quote"</button>
                <button type="button" on:click=move |_| { insert_snippet(set_content, save_content, "<details><summary>Title</summary>\\n\\nBody\\n</details>\\n"); close(()); }>"Details"</button>
            </div>
        }.into_any(),
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
                        let _ = ws.create_folder(parent, "New folder");
                        persist.run(ws);
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
        MenuTarget::None => view! { <span class="hidden"></span> }.into_any(),
    }
}

fn modal_view(
    kind: ModalKind,
    set_modal: WriteSignal<ModalKind>,
    workspace: ReadSignal<Workspace>,
    content: ReadSignal<String>,
    persist: Callback<Workspace>,
    set_active_id: WriteSignal<String>,
) -> AnyView {
    match kind {
        ModalKind::None => view! { <span class="hidden"></span> }.into_any(),
        ModalKind::Export => view! {
            <div class="modal-backdrop" role="dialog" aria-modal="true" on:click=move |_| set_modal.set(ModalKind::None)>
                <div class="modal" on:click=|ev| ev.stop_propagation()>
                    <h2>"Export"</h2>
                    <button type="button" on:click=move |_| {
                        trigger_download("document.md", &content.get_untracked(), "text/markdown");
                        emit(Severity::Info, "export", "markdown", "frontend", "md export", None, None);
                        set_modal.set(ModalKind::None);
                    }>"Markdown (.md)"</button>
                    <button type="button" on:click=move |_| {
                        web_sys::window().and_then(|w| { let _ = w.print(); Some(()) });
                        set_modal.set(ModalKind::None);
                    }>"PDF (print dialog)"</button>
                    <button type="button" on:click=move |_| {
                        if let Ok(raw) = export_container(workspace.get_untracked().to_json()) {
                            trigger_download("workflow.mdu.json", &raw, "application/json");
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
                                wasm_bindgen_futures::spawn_local(async move {
                                    match read_file_text(file).await {
                                        Ok(text) => handle_import_text(text, fname, persist.clone(), set_active_id),
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
            view! {
                <div class="modal-backdrop" role="dialog" aria-modal="true" on:click=move |_| set_modal.set(ModalKind::None)>
                    <div class="modal modal-wide" on:click=|ev| ev.stop_propagation()>
                        <h2>"Diagnostics"</h2>
                        <pre class="diag-log">{lines.join("\n")}</pre>
                        <button type="button" on:click=move |_| {
                            trigger_download("events-export.jsonl", &lines.join("\n"), "application/x-ndjson");
                        }>"Export log"</button>
                    </div>
                </div>
            }.into_any()
        }
    }
}

fn handle_import_text(
    text: String,
    fname: String,
    persist: Callback<Workspace>,
    set_active_id: WriteSignal<String>,
) {
    let lower = fname.to_lowercase();
    if lower.ends_with(".mdu.json") || lower.ends_with(".json") {
        match import_container(&text) {
            Ok(tree) => {
                let ws = Workspace::from_json(tree);
                persist.run(ws.clone());
                if let Some((id, _)) = ws.flatten_files_newest_first().first().cloned() {
                    set_active_id.set(id);
                }
                emit(Severity::Info, "import", "workflow", "frontend", "workflow imported", None, None);
            }
            Err(e) => emit(Severity::Error, "import", "workflow_invalid", "frontend", e.to_string(), None, None),
        }
    } else {
        let mut ws = Workspace::default();
        let name = if fname.is_empty() { "imported.md".into() } else { fname };
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

fn snippet_buttons(
    set_content: WriteSignal<String>,
    save: impl Fn(String) + Copy + 'static,
) -> impl IntoView {
    let add = move |snippet: &'static str| move |_| insert_snippet(set_content, save, snippet);
    view! {
        <button type="button" on:click=add("# Heading\n")>"Heading"</button>
        <button type="button" on:click=add("**bold** ")>"Bold"</button>
        <button type="button" on:click=add("*italic* ")>"Italic"</button>
        <button type="button" on:click=add("`code` ")>"Code"</button>
        <button type="button" on:click=add("- item\n")>"List"</button>
        <button type="button" on:click=add("| H | V |\n|---|---|\n| | |\n")>"Table"</button>
        <button type="button" on:click=add("||secret|| ")>"Spoiler"</button>
    }
}

fn insert_snippet(set_content: WriteSignal<String>, save: impl Fn(String), snippet: &str) {
    set_content.update(|c| {
        c.push_str(snippet);
        save(c.clone());
    });
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

async fn fetch_health() -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };
    let mut opts = web_sys::RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    let request = match web_sys::Request::new_with_str_and_init("/health", &opts) {
        Ok(r) => r,
        Err(_) => return false,
    };
    match wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await {
        Ok(resp) => {
            resp.dyn_into::<web_sys::Response>()
                .map(|r| r.ok())
                .unwrap_or(false)
        }
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
