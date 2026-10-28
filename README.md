# MarkDown Ultimate ✨

Lightweight **markdown viewer and editor** (Rust + Leptos). Works in the browser on GitHub Pages; your files stay in **local storage** on your device.

**Author:** AlexRabbit · **Repo:** [MarkDown-Ultimate](https://github.com/AlexRabbit/MarkDown-Ultimate)

## Quick start (Windows)

1. Clone this repository.
2. Double-click **`Launch-MarkDownUltimate.bat`** at the repo root.
3. The app opens in your browser. Use **Create**, **Open**, and **Save** in the top bar.

If the launcher says build failed, install [Rust stable](https://rustup.rs/) and Visual Studio Build Tools (C++), then run the `.bat` again.

## Online use

After GitHub Actions runs, open **Settings → Pages** and enable **GitHub Actions** as the source. The site lives at:

`https://alexrabbit.github.io/MarkDown-Ultimate/`

## Features

| Area | Behavior |
| --- | --- |
| Sidebar | Files listed newest-first |
| Editor | **Preview** (rendered GFM/HTML) or **Markdown** (source); both editable |
| Save | Export **Markdown**, **PDF** (print dialog), or **WorkFlow** (`.mdu.json` container) |
| Open | Import markdown or WorkFlow (picker wiring in progress) |
| Create | New `.md` tab without replacing the current file |
| Formatting | Quick-insert menu for headings, bold, lists, tables |

Markdown engine: **CommonMark + GFM** via Comrak, with sanitized HTML and partial **Telegram-rich** HTML passthrough.

## Local full stack (optional)

```powershell
cargo run -p markdown_ultimate_engine
```

Serves `dist/` and `GET /health` on `127.0.0.1:8080` (configure in `.env`).

## Project layout

`code/` · `engine/` · `databases/` · `system/` · `frontend/` · `apps/` · `modules/` · `aimemos/` · `bot/` · `logs/` · `QTestings/`

## Tests

```powershell
.\QTestings\run-all.ps1
```

## Troubleshooting

| Issue | Fix |
| --- | --- |
| `link.exe` / `kernel32.lib` missing | Open **Developer PowerShell for VS**, then build |
| Blank page on Pages | Check repo Pages source = GitHub Actions; base path `/MarkDown-Ultimate/` |
| Data gone | Browser storage cleared — export WorkFlow regularly |

## License

MIT — Copyright (c) AlexRabbit
