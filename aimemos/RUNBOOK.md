# Runbook

## Build web

```powershell
cd frontend
trunk build --release
```

Use VS DevShell if `link.exe` is missing.

## Verify

```powershell
.\QTestings\run-all.ps1
```

## Local engine

```powershell
docker compose -f system/docker-compose.yml up -d
cargo run -p markdown_ultimate_engine
```

Open `http://127.0.0.1:8080` (after `dist/` exists).

## Launcher

Double-click `Launch-MarkDownUltimate.bat` (builds once, opens `dist/index.html`).

## Honcho memory (Cursor)

```powershell
.\aimemos\activate-honcho-api.ps1
.\aimemos\seed-honcho-conclusions.ps1
```

Docs: `aimemos/HONCHO.md`, agent bootstrap: `aimemos/AGENT_MEMORY.md`.  
After updating the Honcho MCP plugin, **reload the honcho MCP server** in Cursor so `create_conclusion` scopes to workspace `markdown-ultimate`.

## GitHub Pages 404 on deploy

Enable **Settings → Pages → Build and deployment → GitHub Actions**, then re-run the Pages workflow.
