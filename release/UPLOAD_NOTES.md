# Distribution — one project folder

| What | Path |
|------|------|
| **Your project** | This git clone — one folder only |
| **Run in browser** | `Launch-MarkDownUltimate.bat` |
| **Run desktop window** | `MarkDownUltimate.exe` or `Run-Portable-MarkDownUltimate.bat` |
| **GitHub upload staging** | `release\artifacts\` (exe, zip, apk — not committed to git) |

Legacy sibling folder `MarkDown-Ultimate` beside the repo is deleted by `tools\cleanup-release-duplicates.ps1`.

## Build downloads locally

```powershell
.\tools\portable-bundle.ps1
.\QTestings\build-android.ps1
```

## GitHub Release

```powershell
.\QTestings\scan-repo-before-push.ps1
git push origin main
gh auth login
.\QTestings\create-github-release.ps1 -Tag v0.2.5
```
