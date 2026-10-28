# Native shells (`apps/`)

| Crate | Target | Build |
| --- | --- | --- |
| `desktop/` | Windows/macOS/Linux | `cd apps && cargo build -p mdu_desktop` (requires Dioxus desktop deps) |
| `android/` | Android APK | Install [Dioxus CLI](https://dioxuslabs.com/), Android SDK/NDK, then `dx bundle android` from `apps/android` |

The production editor UI remains **Leptos WASM** under `frontend/` (GitHub Pages + local `serve_dist.py`). Native apps are scaffolds until you choose embedded webview vs shared Rust UI.
