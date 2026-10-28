# MarkDown Ultimate ✨

Lightweight **markdown viewer and editor** (Rust + Leptos). GitHub Pages for online use; workspace stays in **browser local storage**. Optional **Actix engine** for local API + persistence.

**Author:** AlexRabbit · **Repo:** [MarkDown-Ultimate](https://github.com/AlexRabbit/MarkDown-Ultimate)

## Quick start (Windows)

1. Clone this repository.
2. Double-click **`Launch-MarkDownUltimate.bat`** at the repo root.
3. Your browser opens **`http://127.0.0.1:8765/MarkDown-Ultimate/`** — do **not** open `dist\index.html` as a file (assets will not load).

Use **Create**, **Open**, **Save**, and **How to Use** in the header.

## Online app (live site)

**Use the web app here:** [https://alexrabbit.github.io/MarkDown-Ultimate/](https://alexrabbit.github.io/MarkDown-Ultimate/)

The GitHub repository page is source code only; the editor runs on **GitHub Pages** at the link above.

## Features

| Area | Behavior |
| --- | --- |
| Sidebar | Folder tree, newest-first files, drag-and-drop, right-click folder/rename/delete |
| Editor | **Preview** (split + KaTeX) or **Markdown** source; both editable |
| Formats | Toolbar + right-click insert (GFM, spoiler, mark, math, details, …) |
| Save | Export **Markdown**, **PDF** (print), **WorkFlow** (`.mdu.json`) |
| Open | Import `.md` or WorkFlow via file picker |
| Tour | **How to Use** only (`#btnHowTo`); first-visit pulse stops at 60s or click |
| Diagnostics | In-app event tail + export |

## Local engine (optional)

```powershell
docker compose -f system/docker-compose.yml up -d
cargo run -p markdown_ultimate_engine
```

Serves `dist/` and `GET/PUT /api/workspace` on `ENGINE_BIND` from `.env`.

## Tests

```powershell
.\QTestings\run-all.ps1
```

## Layout

`code/` · `engine/` · `databases/` · `system/` · `frontend/` · `apps/` · `modules/` · `aimemos/` · `bot/` · `logs/` · `QTestings/`

## License

MIT — Copyright (c) AlexRabbit
