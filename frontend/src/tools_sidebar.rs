use crate::command_registry::{CommandKind, EditorCommand, CATEGORIES, COMMANDS};
use crate::editor_commands::run_command;
use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const TOOLS_OPEN_KEY: &str = "mdu_tools_open";
const TOOLS_WIDTH_KEY: &str = "mdu_tools_width";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = mduInitToolsSidebarResize)]
    fn mdu_init_tools_sidebar_resize(handle_id: &str, storage_key: &str);
}

pub fn load_tools_open() -> bool {
    LocalStorage::get(TOOLS_OPEN_KEY).unwrap_or(true)
}

pub fn load_tools_width() -> u32 {
    LocalStorage::get(TOOLS_WIDTH_KEY).unwrap_or(320)
}

pub fn save_tools_open(open: bool) {
    let _ = LocalStorage::set(TOOLS_OPEN_KEY, open);
}

pub fn save_tools_width(w: u32) {
    let _ = LocalStorage::set(TOOLS_WIDTH_KEY, w);
}

#[component]
pub fn ToolsSidebar(
    open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    width_px: ReadSignal<u32>,
    set_width_px: WriteSignal<u32>,
    run_command_cb: Callback<EditorCommand>,
) -> impl IntoView {
    let (search, set_search) = signal(String::new());
    let (expanded, set_expanded) = signal::<Vec<&'static str>>(vec!["Text Formatting", "Document Structure"]);

    Effect::new(move |_| {
        if open.get() {
            mdu_init_tools_sidebar_resize("toolsResizeHandle", TOOLS_WIDTH_KEY);
            let set_w = set_width_px;
            let closure = Closure::wrap(Box::new(move |ev: web_sys::CustomEvent| {
                if let Some(w) = ev.detail().as_f64() {
                    let w = w as u32;
                    set_w.set(w);
                    save_tools_width(w);
                }
            }) as Box<dyn FnMut(_)>);
            let window = web_sys::window().unwrap();
            let _ = window.add_event_listener_with_callback("mdu-tools-width", closure.as_ref().unchecked_ref());
            closure.forget();
        }
    });

    view! {
        <button type="button" class="tools-toggle" id="btnToolsToggle" data-tour-id="btnToolsToggle"
            title="Editor tools"
            on:click=move |_| {
                let next = !open.get_untracked();
                set_open.set(next);
                save_tools_open(next);
            }>
            {move || if open.get() { "⟩" } else { "⟨ Tools" }}
        </button>
        {move || if !open.get() {
            view! { <span class="hidden"></span> }.into_any()
        } else {
            view! {
                <>
                    <div id="toolsResizeHandle" class="tools-resize" role="separator" aria-orientation="vertical"
                        aria-label="Resize tools sidebar"></div>
                    <aside class="tools-sidebar" id="toolsSidebar" data-tour-id="toolsSidebar"
                        style=move || format!("width:{}px", width_px.get())>
                        <header class="tools-header">
                            <strong>"Tools"</strong>
                            <button type="button" class="tools-close" aria-label="Close tools"
                                on:click=move |_| { set_open.set(false); save_tools_open(false); }>"×"</button>
                        </header>
                        <input class="tools-search" type="search" placeholder="Search tools…"
                            prop:value=move || search.get()
                            on:input=move |ev| set_search.set(event_target_value(&ev)) />
                        <div class="tools-scroll">
                            {move || {
                                let q = search.get().to_lowercase();
                                CATEGORIES.iter().copied().filter_map(|cat| {
                                    let items: Vec<_> = COMMANDS.iter().filter(|t| {
                                        t.category == cat && (q.is_empty() || t.label.to_lowercase().contains(&q) || t.id.contains(&q))
                                    }).collect();
                                    if items.is_empty() { return None; }
                                    let open_cat = expanded.get().contains(&cat) || !q.is_empty();
                                    Some(view! {
                                        <section class="tools-cat" data-category=cat>
                                            <button type="button" class="tools-cat-head"
                                                on:click=move |_| {
                                                    set_expanded.update(|e| {
                                                        if e.contains(&cat) { e.retain(|c| c != &cat); }
                                                        else { e.push(cat); }
                                                    });
                                                }>
                                                <span>{if open_cat { "▾" } else { "▸" }}</span>
                                                <span>{cat}</span>
                                                <span class="tools-count">{items.len()}</span>
                                            </button>
                                            {open_cat.then(|| view! {
                                                <div class="tools-items">
                                                    {items.into_iter().map(|t| {
                                                        let cmd = *t;
                                                        let hint = t.shortcut.unwrap_or("");
                                                        view! {
                                                            <button type="button" class="tool-row" data-tool-id=t.id
                                                                on:click=move |_| run_command_cb.run(cmd)>
                                                                <span>{t.label}</span>
                                                                {(!hint.is_empty()).then(|| view! {
                                                                    <span class="tool-shortcut">{hint}</span>
                                                                })}
                                                            </button>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            })}
                                        </section>
                                    })
                                }).collect_view()
                            }}
                        </div>
                    </aside>
                </>
            }.into_any()
        }}
    }
}

pub fn run_command_on_editor(
    cmd: EditorCommand,
    set_content: WriteSignal<String>,
    save: impl Fn(String) + Copy + 'static,
) {
    run_command(cmd, set_content, save);
}

fn event_target_value(ev: &web_sys::Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|i| i.value())
        .unwrap_or_default()
}
