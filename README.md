# MarkDown Ultimate

**Author:** [AlexRabbit](https://github.com/AlexRabbit)  
**License:** MIT  
**Live app:** [https://alexrabbit.github.io/MarkDown-Ultimate/](https://alexrabbit.github.io/MarkDown-Ultimate/)

---

## Quick start (Windows)

1. **Download or clone** this repository to a folder on your PC.
2. **Double-click** `Launch-MarkDownUltimate.bat` at the **root** of the project (same folder as this README).
3. Wait a few seconds. Your default browser should open:
   **`http://127.0.0.1:8765/MarkDown-Ultimate/`**
4. You should see the MarkDown Ultimate header (**Create**, **Open**, **Save**, **How to Use**) and the file sidebar on the left.

### What success looks like

| Signal | Meaning |
| --- | --- |
| Browser shows the editor UI | Local server and WASM app loaded correctly |
| URL starts with `http://127.0.0.1:8765` | Correct (not `file://`) |
| First launch may take longer | Trunk builds `dist/` if missing (Rust toolchain required) |

### If startup fails

| Problem | What to do |
| --- | --- |
| Blank page / broken layout | Do **not** open `dist\index.html` directly. Always use the `.bat` launcher or the URL above. |
| `Python is required` | Install [Python 3](https://www.python.org/downloads/) and ensure `python` or `py` is on PATH. |
| `dist/index.html missing` | Install [Rust](https://rustup.rs/) and [Trunk](https://trunkrs.dev/), then run `trunk build --release` inside `frontend/`. |
| Trunk build fails on Windows | Install Visual Studio Build Tools (C++ workload) or run the `.bat` once (it tries to load vcvars). |
| Port 8765 in use | Set environment variable `MDU_PORT=8770`, then launch again. |
| PDF export empty | Allow pop-ups for `127.0.0.1`; PDF uses the browser print dialog on rendered content. |
| Windows SmartScreen / "Unknown publisher" | The `.bat` is an unsigned script from this repo. Click **More info → Run anyway**, or run from a zip you trust. Signing requires a code-signing certificate (not included). |

---

## What this app is

MarkDown Ultimate is a **browser-based Markdown workspace**: multiple files and folders, live preview, export, and optional local API engine. Documents are stored in **browser local storage** unless you use the optional Actix engine.

Built with **Rust + Leptos (WASM)** for the UI and shared **markdown_core** for rendering (CommonMark/GFM-style features, KaTeX math, spoilers, etc.).

---

## Main features

| Feature | How to use |
| --- | --- |
| **View modes** | Top pill: **Markdown** / **Both** / **Preview** — all modes stay editable. |
| **Sidebar** | **+** new folders (unique names); **▼/▶** expand/collapse; **⋮** on files and folders (rename/delete); drag to reorder/nest; scroll the file list and outline. |
| **Tools panel** | **⟨ Tools** on the right — command registry (bold, H1–H6, tables, footnotes, callouts); drag the left edge to resize. |
| **Outline** | Heading list under the file tree; click to jump in the editor. |
| **Find / replace** | Tools → Find and replace, or **Ctrl+F**. |
| **Shortcuts** | Tools → Keyboard shortcuts, or **Ctrl+/**. |
| **Stats** | Word and character count in the status bar under the editor. |
| **Create** | New untitled `.md` in the workspace. |
| **Open** | Import `.md` (merges into workspace) or **WorkFlow** `.mdu.json` (full backup). |
| **Save** | Export current file as `.md`, **PDF** (print dialog), or **WorkFlow** container (all files + settings). |
| **How to Use** | Guided tour (never auto-starts). |
| **Diagnostics** | Recent in-app events + export. |

---

## Online use (no install)

Open the GitHub Pages build (same UI, no local Python server):

**[https://alexrabbit.github.io/MarkDown-Ultimate/](https://alexrabbit.github.io/MarkDown-Ultimate/)**

The GitHub **repository** page is source only; the editor runs on **Pages** at the link above.

---

## Prerequisites

| Component | Required for | Install |
| --- | --- | --- |
| **Windows 10/11** | `.bat` launcher | — |
| **Python 3** | Local static server | python.org |
| **Rust + wasm32 target** | Rebuild frontend | rustup + `rustup target add wasm32-unknown-unknown` |
| **Trunk** | `dist/` build | `cargo install trunk` |
| **Docker** (optional) | Aerospike/NATS stack | Docker Desktop |
| **Node** (optional) | CI only | For GitHub Actions if you fork |

---

## Configuration

Copy `.env.example` to `.env` for the **optional engine** (not required for the browser app):

```env
ENGINE_BIND=127.0.0.1:8080
```

Never commit `.env` — it is listed in `.gitignore`.

---

## Optional local engine

```powershell
docker compose -f system/docker-compose.yml up -d
cargo run -p markdown_ultimate_engine
```

Engine serves `dist/` and workspace API on the bind address from `.env`.

---

## Development and tests

```powershell
cd frontend
trunk serve

.\QTestings\run-all.ps1

.\QTestings\prepare-release.ps1
```

**Distribution:** develop in **repo root**; ship downloads via **[GitHub Releases](https://github.com/AlexRabbit/MarkDown-Ultimate/releases)** and **[GitHub Pages](https://alexrabbit.github.io/MarkDown-Ultimate/)**. Build outputs go to gitignored `release\artifacts\` only (not a nested full project copy).

**Portable Windows:** `powershell -File tools\portable-bundle.ps1` or `prepare-release.ps1` → `release\artifacts\MarkDown-Ultimate-Portable.exe` (+ zip). Details: `release\UPLOAD_NOTES.md`.

**Android:** `apps/` Dioxus scaffold — APK when SDK/`dx` are configured (`QTestings\build-android.ps1`).

---

## Project layout

| Path | Purpose |
| --- | --- |
| `frontend/` | Leptos WASM UI, styles, tour, tools sidebar |
| `modules/` | Workspace tree, markdown core, workflow import/export |
| `engine/` | Optional Actix HTTP API + static `dist/` |
| `code/` | Shared config and diagnostics |
| `system/` | Docker compose for local databases |
| `tools/` | Local server + launcher helpers |
| `QTestings/` | Tests and release prep scripts |
| `dist/` | Built static site (generated) |

---

## Troubleshooting logs

| Location | Content |
| --- | --- |
| In-app **Diagnostics** | Structured frontend events |
| `logs/` | Project log layout (see `logs/README.md`) |
| Browser devtools console | WASM/JS errors |

---

## Security

- No secrets belong in the repo; use `.env.example` templates only.
- See `release/SECURITY_CHECK.md` after running `prepare-release.ps1`.

---

## License and credits

**MIT License** — Copyright (c) AlexRabbit

Third-party assets (KaTeX, Driver.js, etc.) remain under their respective licenses in the frontend build.

---

## Maintainer

**AlexRabbit** — [GitHub](https://github.com/AlexRabbit/MarkDown-Ultimate)
