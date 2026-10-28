# Distribution layout (read this once)

## One project — one folder

**Edit and run only here:**

`A:\VibeCode\MarkDown Ultimate\`

That is the git repo root (`Launch-MarkDownUltimate.bat`, `dist/`, `frontend/`, etc.).

We **do not** maintain a second full copy at `release\MarkDown-Ultimate\`. If you see that folder, delete it and run:

```powershell
.\tools\cleanup-release-duplicates.ps1
```

## GitHub Releases assets only

Build outputs go to **`release\artifacts\`** (gitignored):

- `MarkDown-Ultimate-Portable.exe` — native window (WebView2), not an external browser
- `MarkDown-Ultimate-Portable.zip` — same app + embedded Python + `dist/`
- `MarkDown-Ultimate.apk` — when Android SDK + `dx` are installed

Upload those files to [GitHub Releases](https://github.com/AlexRabbit/MarkDown-Ultimate/releases).

## Not the same as the repo name on disk

The folder is `MarkDown Ultimate` (space) on your PC. GitHub repo is `MarkDown-Ultimate` (hyphen). That is normal — do not clone a third copy under `A:\VibeCode\MarkDown-Ultimate` unless you intentionally want a separate clone.
