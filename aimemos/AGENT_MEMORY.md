# Agent memory — MarkDown Ultimate (read this to redo or fix everything)

**Owner:** AlexRabbit · **Repo:** https://github.com/AlexRabbit/MarkDown-Ultimate  
**Root:** `A:\VibeCode\MarkDown Ultimate`  
**Honcho workspace:** `markdown-ultimate` · **Session:** `aimemos-markdown-ultimate`

## What this product is

Static-first **Leptos CSR WASM** markdown workspace editor. Default storage: **browser localStorage**. Optional **Actix** engine syncs workspace JSON to disk + optional NATS event. **No Dioxus/mobile** unless user explicitly requests.

## Architecture map

| Layer | Path | Role |
| --- | --- | --- |
| UI | `frontend/src/app.rs`, `lib.rs` | Tree sidebar CRUD/DnD, import FileReader, export WorkFlow, diagnostics modal, KaTeX preview, Driver tour |
| Styles/JS | `frontend/styles/app.css`, `frontend/js/mdu-ui.js`, `index.html` | Driver.js + KaTeX CDN, tour trigger |
| Workspace model | `modules/workspace/` | Tree CRUD, `updated_at` sort, tests |
| Markdown | `modules/markdown_core/` | Comrak GFM + Telegram preprocess (`||`, `==`), Ammonia allowlist |
| Diagnostics | `code/diagnostics/` | Schema v2 JSONL ring + native `logs/hot/` |
| Engine B | `engine/` | `/health`, GET/PUT `/api/workspace`, serves `dist/` |
| Persistence | `databases/` | `FileWorkspaceStore` → `databases/data/workspace-default.json` |
| NATS | `engine` + `system/docker-compose.yml` | Publish on save if `NATS_URL` set |
| Tests | `QTestings/run-all.ps1` | cargo tests, JSON fixtures, tour coverage, engine compile |
| Release | `QTestings/prepare-release.ps1` → `release/MarkDown-Ultimate/` | No `target/` |
| CI | `.github/workflows/verify.yml`, `pages.yml` | PR verify + Pages deploy |

## Launch (user)

1. `Launch-MarkDownUltimate.bat` → builds `dist/` if missing (Trunk), opens `dist/index.html`
2. Optional API: `cargo run -p markdown_ultimate_engine` (after `.env`); frontend probes `/health`

## Tour rules (constitution)

- **Only** `#btnHowTo` starts Driver.js
- Pulse: first visit, 60s max, stored in `localStorage` key `mdu_howto_pulse_seen`
- Registry: `frontend/tour/registry.json`; gate: `QTestings/tour-coverage.ps1`

## Verification commands

```powershell
cd "A:\VibeCode\MarkDown Ultimate"
.\QTestings\run-all.ps1
cd frontend; trunk build --release
cargo check -p markdown_ultimate_engine
.\aimemos\activate-honcho-api.ps1
```

## Honcho

See `aimemos/HONCHO.md`. Local mirrors **do not replace** Postgres-backed conclusions. Use MCP `create_conclusion` after API is healthy. Project MCP: `.cursor/mcp.json` sets `HONCHO_PROJECT_ROOT`.

Verify script (canonical): `A:\honcho\aimemos\verify_memory.py --preset markdown-ultimate`

## Known gaps (do not claim done)

- Full CommonMark 0.31.2 corpus (`aimemos/COMRAK_DELTAS.md`)
- Native Aerospike client (Docker + file store only)
- Full Telegram RichBlock parser
- Forensic diagnostics megaprompt tier-2 (`logs/ARCHITECTURE.md`)
- GitHub Pages **must be enabled** in repo Settings → Pages → GitHub Actions (404 deploy otherwise)
- KaTeX block math + Mermaid fences (backlog)
- QTestings mutation/fuzz matrix (backlog)

## Authorship policy

Commits/releases: **AlexRabbit** only when user asks. No AI co-author on git metadata.

## Last verified (2026-10-28)

- `QTestings/run-all.ps1` PASS
- `trunk build --release` PASS (Trunk.toml pre-dist hook removed)
- Honcho WSL stack starts; MCP scoped to `markdown-ultimate` (plugin patch)
- Postgres contains `markdown-ultimate` workspace + `aimemos-markdown-ultimate` session (see Honcho API logs)

## Related files

- `ARCHITECTURE.md`, `RUNBOOK.md`, `GAPS_AND_DECISIONS.md`, `DOD_EVIDENCE.md`, `HOWTO_TOUR.md`, `PROJECT_STATE.md`
