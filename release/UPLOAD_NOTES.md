# Distribution — one portable folder only

| What | Path |
|------|------|
| **Source / dev** | Your git clone (repo root) — `Launch-MarkDownUltimate.bat` opens the browser + local server |
| **Portable Windows app** | Sibling folder **`MarkDown-Ultimate`** next to the repo (created by `portable-bundle.ps1`) — run **`MarkDownUltimate.exe`** |
| **GitHub Releases** | `release/artifacts/` — `MarkDownUltimate.exe`, zip, APK (gitignored staging) |

Set **`MDU_PORTABLE_ROOT`** to override the portable install folder.

We do **not** ship a second full copy of the repo under `release/MarkDown-Ultimate/`.

## Build

```powershell
cd frontend
trunk build --release
cd ..
.\tools\portable-bundle.ps1
.\QTestings\build-android.ps1
```

## GitHub Release

```powershell
.\QTestings\scan-repo-before-push.ps1
git push origin main
gh auth login
.\QTestings\create-github-release.ps1 -Tag v0.2.4
```

Do **not** run `MarkDownUltimate.exe` from the dev repo root unless `dist/` and `python/` sit beside that exe (use the portable folder instead).
