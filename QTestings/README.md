# QTestings — verification protocol

Run from repo root (VsDevShell recommended on Windows):

```powershell
.\QTestings\run-all.ps1
```

Layers:

- **Unit:** `cargo test --workspace --exclude markdown_ultimate_frontend`
- **Markdown:** `cargo test -p mu_markdown_core`
- **Meta:** test discovery script ensures workspace members resolve

Evolution log: `QTestings/EVOLUTION.md`
