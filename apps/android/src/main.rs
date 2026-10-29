//! Android shell — WebView loads the published GitHub Pages app (network required).
use dioxus::prelude::*;

const APP_URL: &str = "https://alexrabbit.github.io/MarkDown-Ultimate/";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { style: "width:100%;height:100%;margin:0;padding:0;overflow:hidden;",
            iframe {
                src: APP_URL,
                title: "MarkDown Ultimate",
                style: "border:0;width:100%;height:100vh;",
            }
        }
    }
}
