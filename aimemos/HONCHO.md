# Honcho + Cursor — MarkDown Ultimate

## Identity (isolated from other projects)

| Field | Value |
| --- | --- |
| Honcho workspace | `markdown-ultimate` |
| Session | `aimemos-markdown-ultimate` |
| User peer | `robots` |
| AI peer | `cursor` |
| Project root | `A:\VibeCode\MarkDown Ultimate` |
| Canonical file | `aimemos/identity.json` |

Reasoned memory lives in **Postgres** (Honcho backend). Files under `aimemos/` are **mirrors and runbooks** for agents when Honcho is offline.

## Start the API (required for MCP)

**Verified path (Docker off):** WSL stack on port **8000** (matches `HONCHO_ENDPOINT` in Cursor MCP).

```powershell
powershell -ExecutionPolicy Bypass -File A:\honcho\aimemos\start-stack.ps1
```

Or from this project:

```powershell
.\aimemos\activate-honcho-api.ps1
```

Health: `http://127.0.0.1:8000/health` → `{"status":"ok"}`

**Alternative (Docker Desktop running):** `A:\honcho\start-local.ps1` → API on **8899**; then set `HONCHO_ENDPOINT=http://127.0.0.1:8899/v3` in MCP env (not default).

## MCP tools (Cursor)

- Plugin: `%USERPROFILE%\.honcho\plugins\cursor-honcho`
- Global MCP: `%USERPROFILE%\.cursor\mcp.json`
- **Project override:** `.cursor/mcp.json` sets `HONCHO_PROJECT_ROOT` so workspace scopes to `markdown-ultimate`.

After plugin updates, **reload MCP** in Cursor (Settings → MCP → restart honcho server).

Tools: `search`, `chat`, `create_conclusion`, `get_config`, `set_config`, `activate_api` (starts WSL stack).

**Fix applied (2026-10-28):** MCP session tools now scope Honcho client per project via `aimemos/identity.json` (patch in `mcp/server.ts`).

## Verify memory round-trip

Uses `A:\honcho\aimemos\verify_memory.py` preset `markdown-ultimate`:

```powershell
$env:HONCHO_VERIFY_API = "http://127.0.0.1:8000/v3"
cd A:\honcho
.\.venv\Scripts\python.exe aimemos\verify_memory.py write --preset markdown-ultimate
.\.venv\Scripts\python.exe aimemos\verify_memory.py read --preset markdown-ultimate
```

From WSL (more stable under load):

```powershell
wsl -d Ubuntu -u root -- bash -lc 'export HONCHO_VERIFY_API=http://127.0.0.1:8000/v3; /mnt/a/honcho/.venv-linux/bin/python /mnt/a/honcho/aimemos/verify_memory.py read --preset markdown-ultimate'
```

## Agent reading order

1. `aimemos/AGENT_MEMORY.md` — full project truth for redo/fix
2. `aimemos/PROJECT_STATE.md` — short status
3. `aimemos/RUNBOOK.md` — commands
4. `aimemos/GAPS_AND_DECISIONS.md` — honest backlog
5. Honcho `search` / `chat` when API is up

## GitHub Pages deploy error (404)

If Actions `deploy-pages` fails with *Ensure GitHub Pages has been enabled*:

1. GitHub → **AlexRabbit/MarkDown-Ultimate** → **Settings** → **Pages**
2. **Build and deployment** → Source: **GitHub Actions**
3. Re-run the **Deploy GitHub Pages** workflow

No code change fixes a disabled Pages product on the repo.

## Recovery

- Stack logs: `A:\honcho\aimemos\logs\` (`api-wsl.err.log`, `wsl-start.log`)
- Stop stack: `A:\honcho\aimemos\stop-stack.ps1`
- Monitor: `A:\honcho\aimemos\monitor-stack.ps1`
