//! Android shell — WebView loads the published app (requires SDK build via build-android.ps1).
use dioxus::prelude::*;

fn main() {
    dioxus::mobile::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { style: "padding:1rem;font-family:system-ui;",
            h1 { "MarkDown Ultimate" }
            p { "Install the release APK from GitHub Releases after build-android.ps1 succeeds." }
            p { "Online: https://alexrabbit.github.io/MarkDown-Ultimate/" }
        }
    }
}
