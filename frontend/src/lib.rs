use gloo_storage::{LocalStorage, Storage};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use mu_diagnostics::{emit, Severity};
use mu_markdown_core::render_markdown;
use mu_workflow::export_container;
use mu_workspace::{Node, Workspace};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, Url};

const STORAGE_KEY: &str = "mdu_workspace_v1";
const PULSE_KEY: &str = "mdu_howto_pulse_seen";

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
}

#[component]
pub fn App() -> impl IntoView {
    let (workspace, set_workspace) = signal(load_workspace());
    let (active_id, set_active_id) = signal(default_active(&workspace.get_untracked()));
    let (editor_mode, set_editor_mode) = signal(EditorMode::Rendered);
    let (modal, set_modal) = signal(ModalKind::None);
    let (content, set_content) = signal(String::new());
    let (show_pulse, set_show_pulse) = signal(!LocalStorage::get::<bool>(PULSE_KEY).unwrap_or(false));

    Effect::new(move || {
        if let Some(c) = find_content(&workspace.get(), &active_id.get()) {
            set_content.set(c);
        }
    });

    let save_content = move |text: String| {
        let mut ws = workspace.get_untracked();
        if update_content(&mut ws, &active_id.get_untracked(), text) {
            let _ = LocalStorage::set(STORAGE_KEY, serde_json::to_string(&ws).unwrap_or_default());
            set_workspace.set(ws);
        }
    };

    view! {
        <div class="shell">
            <header class="topbar">
                <div class="actions">
                    <button type="button" on:click=move |_| {
                        let mut ws = workspace.get_untracked();
                        let file = Node::new_file(format!("Untitled-{}.md", short_id()));
                        let id = if let Node::File { id, .. } = &file { id.clone() } else { String::new() };
                        ws.root.push(file);
                        let _ = LocalStorage::set(STORAGE_KEY, serde_json::to_string(&ws).unwrap_or_default());
                        set_workspace.set(ws);
                        set_active_id.set(id);
                        emit(Severity::Info, "workspace", "create", "frontend", "new file", None, None);
                    }>"Create"</button>
                    <button type="button" on:click=move |_| set_modal.set(ModalKind::Import)>"Open"</button>
                    <button type="button" on:click=move |_| set_modal.set(ModalKind::Export)>"Save"</button>
                </div>
                <button id="btnHowTo" class=move || if show_pulse.get() { "howto pulse" } else { "howto" }
                    on:click=move |_| { set_show_pulse.set(false); let _ = LocalStorage::set(PULSE_KEY, true); }>
                    "How to Use"
                </button>
            </header>
            <div class="main">
                <aside class="sidebar">
                    <ul class="file-list">
                        {move || workspace.get().flatten_files_newest_first().into_iter().map(|(id, path)| {
                            let pick = id.clone();
                            let pick_click = id.clone();
                            view! {
                                <li class=move || if active_id.get() == pick { "row active" } else { "row" }
                                    on:click=move |_| set_active_id.set(pick_click.clone())>{path}</li>
                            }
                        }).collect_view()}
                    </ul>
                </aside>
                <section class="editor-pane">
                    <div class="mode-toggle">
                        <button type="button" class=move || if editor_mode.get()==EditorMode::Rendered {"active"} else {""}
                            on:click=move |_| set_editor_mode.set(EditorMode::Rendered)>"Preview"</button>
                        <button type="button" class=move || if editor_mode.get()==EditorMode::Source {"active"} else {""}
                            on:click=move |_| set_editor_mode.set(EditorMode::Source)>"Markdown"</button>
                    </div>
                    <div class="editor">
                        <menu class="ctx">
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
                                    <div class="preview" inner_html=move || render_markdown(&content.get()) />
                                </div>
                            }.into_any()
                        }}
                    </div>
                </section>
            </div>
            {move || match modal.get() {
                ModalKind::None => view! { <span class="hidden"></span> }.into_any(),
                ModalKind::Export => view! {
                    <div class="modal-backdrop" on:click=move |_| set_modal.set(ModalKind::None)>
                        <div class="modal" on:click=|ev| ev.stop_propagation()>
                            <h2>"Export"</h2>
                            <button type="button" on:click=move |_| {
                                trigger_download("document.md", &content.get_untracked(), "text/markdown");
                                set_modal.set(ModalKind::None);
                            }>"Markdown (.md)"</button>
                            <button type="button" on:click=move |_| { web_sys::window().and_then(|w| { let _ = w.print(); Some(()) }); set_modal.set(ModalKind::None); }>"PDF (print dialog)"</button>
                            <button type="button" on:click=move |_| {
                                if let Ok(raw) = export_container(workspace.get_untracked().to_json()) {
                                    trigger_download("workflow.mdu.json", &raw, "application/json");
                                }
                                set_modal.set(ModalKind::None);
                            }>"WorkFlow container (.mdu.json)"</button>
                        </div>
                    </div>
                }.into_any(),
                ModalKind::Import => view! {
                    <div class="modal-backdrop" on:click=move |_| set_modal.set(ModalKind::None)>
                        <div class="modal" on:click=|ev| ev.stop_propagation()>
                            <h2>"Import"</h2>
                            <p>"Drop a .md file or .mdu.json WorkFlow in the file picker below."</p>
                            <input type="file" accept=".md,.markdown,.mdu.json,.json" on:change=move |ev| {
                                let _ = ev;
                                set_modal.set(ModalKind::None);
                            } />
                        </div>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}

fn snippet_buttons(
    set_content: WriteSignal<String>,
    save: impl Fn(String) + Copy + 'static,
) -> impl IntoView {
    let add = move |snippet: &'static str| move |_| {
        set_content.update(|c| {
            c.push_str(snippet);
            save(c.clone());
        });
    };
    view! {
        <button type="button" on:click=add("# Heading\n")>"Heading"</button>
        <button type="button" on:click=add("**bold** " )>"Bold"</button>
        <button type="button" on:click=add("*italic* " )>"Italic"</button>
        <button type="button" on:click=add("`code` " )>"Code"</button>
        <button type="button" on:click=add("- item\n")>"List"</button>
        <button type="button" on:click=add("| H | V |\n|---|---|\n| | |\n")>"Table"</button>
    }
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
    ws.root.iter().find_map(|n| match n {
        Node::File { id, .. } => Some(id.clone()),
        _ => None,
    }).unwrap_or_default()
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
                    if let Some(r) = walk(children, id) { return Some(r); }
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

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(App);
}
