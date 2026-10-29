# Distribution — one portable folder only

| What | Path |
|------|------|
| **Source / dev** | `A:\VibeCode\MarkDown Ultimate\` — edit code, run `Launch-MarkDownUltimate.bat` (browser + local server) |
| **Portable Windows app** | `A:\VibeCode\MarkDown-Ultimate\` — double-click **`MarkDownUltimate.exe`** (native window, no extra CMD, no external browser) |
| **GitHub Releases upload** | `release\artifacts\` — only **`MarkDownUltimate.exe`**, **`MarkDownUltimate-Portable.zip`**, **`MarkDown-Ultimate.apk`** (staging, gitignored) |

We **do not** create `release\MarkDown-Ultimate\` or nested `MarkDown-Ultimate-Portable\` copies anymore.  
Run `tools\cleanup-release-duplicates.ps1` if old duplicates remain.

## Build portable + staging assets

```powershell
cd "A:\VibeCode\MarkDown Ultimate"
.\tools\portable-bundle.ps1
```

## GitHub Release

```powershell
& "${env:ProgramFiles}\GitHub CLI\gh.exe" auth login
.\QTestings\create-github-release.ps1 -Tag v0.2.3
```

## Android APK

Requires Android Studio SDK + `cargo install dioxus-cli`, then:

```powershell
.\QTestings\build-android.ps1
```

Output: `release\artifacts\MarkDown-Ultimate.apk`
