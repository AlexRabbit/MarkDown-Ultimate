//! Text commands for the tools sidebar (selection-aware via JS when available).

use crate::command_registry::{CommandKind, EditorCommand};
use leptos::prelude::Update;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = mduApplyEditorCommand)]
    fn mdu_apply_editor_command(
        editor_id: &str,
        before: &str,
        after: &str,
        placeholder: &str,
    ) -> JsValue;
    #[wasm_bindgen(js_namespace = window, js_name = mduReadEditor)]
    fn mdu_read_editor(editor_id: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = window, js_name = mduPrintMarkdown)]
    fn mdu_print_markdown(title: &str, html: &str) -> bool;
    #[wasm_bindgen(js_namespace = window, js_name = mduCopyText)]
    fn mdu_copy_text(text: &str) -> bool;
    #[wasm_bindgen(js_namespace = window, js_name = mduJsEditorAction)]
    fn mdu_js_editor_action(editor_id: &str, action: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = window, js_name = mduJumpToOffset)]
    fn mdu_jump_to_offset(editor_id: &str, offset: u32);
    #[wasm_bindgen(js_namespace = window, js_name = mduFindReplace)]
    fn mdu_find_replace(editor_id: &str, find: &str, replace: &str, replace_all: bool) -> JsValue;
    #[wasm_bindgen(js_namespace = window, js_name = mduInitOutlineResize)]
    fn mdu_init_outline_resize(handle_id: &str, panel_id: &str, storage_key: &str);
    #[wasm_bindgen(js_namespace = window, js_name = mduSyncPreviewLayout)]
    fn mdu_sync_preview_layout();
    #[wasm_bindgen(js_namespace = window, js_name = mduInitAppShortcuts)]
    fn mdu_init_app_shortcuts();
}

pub fn sync_preview_layout() {
    mdu_sync_preview_layout();
}

const EDITOR_ID: &str = "mduSourceEditor";

pub fn init_editor_shortcuts() {
    mdu_init_app_shortcuts();
}

pub fn jump_to_offset(offset: u32) {
    mdu_jump_to_offset(EDITOR_ID, offset);
}

pub fn find_replace(find: &str, replace: &str, replace_all: bool) -> Option<String> {
    js_value_to_string(mdu_find_replace(EDITOR_ID, find, replace, replace_all))
}

pub fn run_command(
    cmd: EditorCommand,
    set_content: leptos::prelude::WriteSignal<String>,
    save: impl Fn(String) + Copy + 'static,
) {
    if cmd.kind == CommandKind::Modal {
        return;
    }
    set_content.update(|c| {
        match cmd.kind {
            CommandKind::Wrap => {
                apply_tool(EDITOR_ID, cmd.before, cmd.after, cmd.placeholder, c);
            }
            CommandKind::Js => {
                let _ = mdu_js_editor_action(EDITOR_ID, cmd.action);
                if let Some(v) = js_value_to_string(mdu_read_editor(EDITOR_ID)) {
                    *c = v;
                }
            }
            CommandKind::Modal => {}
        }
        save(c.clone());
    });
}

pub fn apply_tool(
    editor_id: &str,
    before: &str,
    after: &str,
    placeholder: &str,
    content: &mut String,
) {
    let _ = mdu_apply_editor_command(editor_id, before, after, placeholder);
    if let Some(v) = js_value_to_string(mdu_read_editor(editor_id)) {
        *content = v;
        return;
    }
    if !before.is_empty() || !after.is_empty() || !placeholder.is_empty() {
        content.push_str(before);
        content.push_str(placeholder);
        content.push_str(after);
    }
}

pub fn print_markdown(title: &str, html: &str) -> bool {
    mdu_print_markdown(title, html)
}

pub fn copy_text(text: &str) -> bool {
    mdu_copy_text(text)
}

fn js_value_to_string(v: JsValue) -> Option<String> {
    if v.is_null() || v.is_undefined() {
        None
    } else {
        v.as_string()
    }
}
