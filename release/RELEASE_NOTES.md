## v0.2.4

- **CI:** Actix 3.20 still pulls vulnerable `h2` 0.3.x in the lockfile; engine stays on **Axum** so `cargo audit` passes
- **UI:** Sidebar file-tree scroll; compact **📄** / **+** toolbar buttons (same size)
- **Portable:** `MarkDownUltimate.exe` only from sibling **`MarkDown-Ultimate`** folder (or set `MDU_PORTABLE_ROOT`); clear error if run from dev root without `dist/`
- **Privacy:** Release docs use generic paths only; artifacts scanned before upload

## v0.2.2

- Release workflow: artifacts only under `release/artifacts/` (no duplicate project tree)
- **MarkDownUltimate.exe** — Rust launcher + embeddable Python + built `dist`
- Portable zip for users who prefer a folder layout
- GitHub Releases as primary download channel

## v0.2.1

- Sidebar: delete/rename files via ⋮ menu; folder expand/collapse (▼/▶); scrollable file tree and outline
- PDF export via browser print (allow pop-ups)
- Diagnostics: Copy all
- Engine: Axum + updated NATS client; `cargo audit` clean
- Portable Windows bundle script (`tools/portable-bundle.ps1`)

**Android:** scaffold APK via `QTestings/build-android.ps1` when SDK/NDK and `dx` are installed.
