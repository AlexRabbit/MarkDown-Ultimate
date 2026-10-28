//! Dioxus Android scaffold — bundle with `dx bundle android` once SDK/NDK are configured.
use dioxus::prelude::*;

fn main() {
    dioxus::mobile::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { style: "padding: 1rem; font-family: system-ui;",
            h1 { "MarkDown Ultimate (Android scaffold)" }
            p { "Wire this shell to the WASM web UI or a native markdown surface in a follow-up." }
        }
    }
}
