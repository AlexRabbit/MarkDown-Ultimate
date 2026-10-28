mod app;
mod command_registry;
mod editor_commands;
mod tools_sidebar;

use app::App;
use leptos::mount::mount_to_body;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(App);
}
