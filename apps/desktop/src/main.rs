//! Dioxus desktop scaffold — point webview at local engine or GitHub Pages.
use dioxus::prelude::*;

const DEFAULT_URL: &str = "http://127.0.0.1:8765/MarkDown-Ultimate/";

fn main() {
    dioxus::desktop::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { style: "font-family: system-ui; padding: 1.5rem;",
            h1 { "MarkDown Ultimate (desktop scaffold)" }
            p { "Open the web app in your browser, or run the Actix engine and load {DEFAULT_URL} in a custom webview (next step)." }
            p { "Build: cd apps && cargo build -p mdu_desktop" }
        }
    }
}
