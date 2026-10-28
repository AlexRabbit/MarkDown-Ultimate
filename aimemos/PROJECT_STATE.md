# MarkDown Ultimate — project state

Updated: 2026-10-28

## Status

Web + local engine path **implemented**; `QTestings/run-all.ps1` green (cargo tests, JSON fixtures, tour coverage, aimemos identity, engine compile).

## Verified

- `trunk build --release` → `dist/` (Trunk.toml hook removed)
- `release/MarkDown-Ultimate/` via `QTestings/prepare-release.ps1`
- Tour registry 10 steps, coverage gate PASS
- Honcho: local `aimemos/identity.json`; MCP `create_conclusion` when server at 127.0.0.1:8899

## See also

- `aimemos/ARCHITECTURE.md`
- `aimemos/GAPS_AND_DECISIONS.md`
- `aimemos/RUNBOOK.md`
- `aimemos/DOD_EVIDENCE.md`
- `aimemos/COMRAK_DELTAS.md`
