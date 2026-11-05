use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;

pub const DOCS_OPEN_KEY: &str = "mdu_docs_open";

pub fn load_docs_open() -> bool {
    LocalStorage::get(DOCS_OPEN_KEY).unwrap_or(true)
}

pub fn save_docs_open(open: bool) {
    let _ = LocalStorage::set(DOCS_OPEN_KEY, open);
}

#[component]
pub fn DocsToggle(open: ReadSignal<bool>, set_open: WriteSignal<bool>) -> impl IntoView {
    view! {
        <button type="button" class="docs-toggle" id="btnDocsToggle" data-tour-id="btnDocsToggle"
            title="Files and outline"
            on:click=move |_| {
                let next = !open.get_untracked();
                set_open.set(next);
                save_docs_open(next);
            }>
            {move || if open.get() { "⟨" } else { "Docs ⟩" }}
        </button>
    }
}
