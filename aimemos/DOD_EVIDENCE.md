# Definition of Done — evidence pass (Phase 10)

Date: 2026-10-28  
Scope: Platform **A** (GitHub Pages / static `dist`) + **B** (local Actix engine)

## Automated verification

| Check | Command | Result |
| --- | --- | --- |
| Native unit + integration | `.\QTestings\run-all.ps1` | Run locally after changes |
| WASM compile | `cargo check -p markdown_ultimate_frontend --target wasm32-unknown-unknown` | Pass (session) |
| Release static bundle | `cd frontend && trunk build --release` | Pass after Trunk.toml hook fix |
| Engine compile | `cargo check -p markdown_ultimate_engine` | Pass (session) |
| CI | `.github/workflows/verify.yml` on push/PR | Configured |

## Manual scenarios (operator)

1. **Cold start (web):** `Launch-MarkDownUltimate.bat` → opens `dist/index.html` → edit → Save → WorkFlow export → Open re-import.
2. **Sidebar:** create folder/file, rename, delete (confirm), drag-drop nest/reorder, newest-first by `updated_at`.
3. **Tour:** pulse first visit only; 60s stop; **only** `#btnHowTo` starts Driver.js; never auto-run on load.
4. **Diagnostics:** open modal; events after import/export/workspace actions (WASM → console + ring buffer).
5. **Engine B:** `docker compose -f system/docker-compose.yml up -d` (optional); `cargo run -p markdown_ultimate_engine`; `curl http://127.0.0.1:8080/health`; GET/PUT `/api/workspace`.

## Honest remaining gaps

- Full CommonMark 0.31.2 corpus — see `aimemos/COMRAK_DELTAS.md`.
- Native Aerospike client — file store + Docker docs only.
- Honcho `create_conclusion` when MCP server offline — local `aimemos/*` mirrors authoritative.
- GitHub tag **v0.2.0** / push — requires user authorization per project rules.
- Forensic megaprompt (incidents UI, anomaly detection) — backlog in `logs/ARCHITECTURE.md`.
- Dioxus `apps/`, Telegram `bot/` — out of scope until requested.

## Release tree

- `QTestings/prepare-release.ps1` → `release/MarkDown-Ultimate/` (no `target/`).
- Secret scan documented in `release/SECURITY_CHECK.md`.
