# MarkDown Ultimate — agent memory (Honcho-backed)

## Identity

| Field | Value |
| --- | --- |
| Honcho install | `A:\honcho` |
| Workspace | `markdown-ultimate` |
| Session | `aimemos-markdown-ultimate` |
| Plugin mapping | `aimemos/identity.json` + Cursor `sessions.set` for this folder |
| Local mirror | This directory (decisions, runbooks — not the Postgres DB) |

## Product (2026-10-28)

- **Goal:** Lightweight markdown viewer/editor for GitHub Pages and local use.
- **Platform scope (inferred):** **Web only** (Leptos CSR/WASM) for https://github.com/AlexRabbit/MarkDown-Ultimate — **confirm desktop/mobile with user before Dioxus work.**
- **Stack (locked):** Rust `code/`, Actix `engine/`, Aerospike `databases/` (stub), NATS `system/` (stub), Leptos `frontend/`, Dioxus `apps/` (stub), `modules/`, `aimemos/`, `bot/` (empty).

## Architecture snapshot

- `modules/markdown_core` — Comrak GFM + HTML sanitize (Ammonia); Telegram-rich HTML passthrough partial.
- `modules/workspace` — tree model (files/folders), JSON persistence.
- `modules/workflow` — `.mdu.json` WorkFlow container export/import.
- `frontend` — Leptos 0.7 CSR; localStorage workspace; Preview/Markdown toggle; Create/Open/Save modals.
- `engine` — Actix serves `/health` + static `dist/` for local full stack.
- `code/diagnostics` — JSONL flight recorder (`logs/hot/events.jsonl` native; console ring on WASM).

## Verified on this machine

- `cargo test -p mu_markdown_core` (GFM table test) — pass.
- `trunk build --release` in `frontend/` — pass (wasm32 target added).
- Honcho MCP `sessions.set` for project path — applied.
- MSVC: use **VsDevShell** before `cargo`/`trunk` (kernel32.lib / link.exe PATH).

## Known gaps (honest)

- Import file picker not fully wired (FileReader); WorkFlow import/export JSON works via Save.
- Sidebar: folder create/rename/delete/drag-drop — not complete in UI.
- Driver.js “How to Use” tour — button + pulse only; full tour registry not implemented.
- Aerospike/NATS not running in GitHub Pages path (browser IndexedDB only online).
- Telegram rich markdown: subset via HTML passthrough; not full Bot API block model.
- CommonMark 0.31.2 / full GFM parity — via Comrak; add conformance tests in `QTestings/`.

## Module order

1. `markdown_core` 2. `workspace` 3. `workflow` 4. `diagnostics` 5. `frontend` 6. `engine` 7. `databases` 8. `system` 9. `apps`

## Commands

```powershell
# Build web artifact
Import-Module "${env:ProgramFiles(x86)}\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\Microsoft.VisualStudio.DevShell.dll"
Enter-VsDevShell -VsInstallPath (& "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe" -latest -property installationPath) -SkipAutomaticLocation -DevCmdArguments '-arch=amd64'
cd "A:\VibeCode\MarkDown Ultimate\frontend"
trunk build --release

# Local engine + static
cd "A:\VibeCode\MarkDown Ultimate"
cargo run -p markdown_ultimate_engine
```

## Next agent actions

1. Ask user: web / desktop / mobile / subset (if not confirmed).
2. Wire import FileReader + drag-drop + folder CRUD.
3. Expand `QTestings` CommonMark/GFM fixtures.
4. GitHub Actions deploy `dist/` to Pages (workflow added).
5. Optional: Honcho `verify_memory.py` clone for workspace `markdown-ultimate`.
