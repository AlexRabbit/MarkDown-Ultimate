//! Android shell — WebView loads bundled mobile web app from APK assets (offline-capable).
use dioxus::prelude::*;

#[cfg(target_os = "android")]
const APP_URL: &str = "file:///android_asset/web/index.html";

#[cfg(not(target_os = "android"))]
const APP_URL: &str = "https://alexrabbit.github.io/MarkDown-Ultimate/";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { style: "width:100%;height:100dvh;margin:0;padding:0;overflow:hidden;background:#0f1419;",
            iframe {
                src: APP_URL,
                title: "MarkDown Ultimate",
                style: "border:0;width:100%;height:100%;display:block;",
            }
        }
    }
}
