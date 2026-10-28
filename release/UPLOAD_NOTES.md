# GitHub distribution — AlexRabbit

**Primary distribution:** [GitHub Releases](https://github.com/AlexRabbit/MarkDown-Ultimate/releases) assets, **repo root** source, and [GitHub Pages](https://alexrabbit.github.io/MarkDown-Ultimate/).

There is **no** second copy of the project under `release/MarkDown-Ultimate/`. Staging is **`release/artifacts/`** only (gitignored).

## Build release assets (from repo root)

```powershell
.\QTestings\prepare-release.ps1
```

| Asset | Path |
| --- | --- |
| **Portable launcher (.exe)** | `release\artifacts\MarkDown-Ultimate-Portable.exe` |
| **Portable folder + zip** | `release\artifacts\MarkDown-Ultimate-Portable\`, `MarkDown-Ultimate-Portable.zip` |
| **Android APK** | `release\artifacts\MarkDown-Ultimate.apk` (when SDK/`dx` available; else `ANDROID_BUILD_BLOCKER.md`) |

The `.exe` starts embedded or system Python + `tools/serve_dist.py` and opens the browser. The `.bat` launcher in the same folder is optional.

## Create / upload GitHub Release

```powershell
.\QTestings\create-github-release.ps1 -Tag v0.2.2
```

Requires [GitHub CLI](https://cli.github.com/) (`gh auth login`). Without `gh`, attach files from `release\artifacts\` manually on the Releases page.

## Verify before upload

```powershell
.\QTestings\scan-release-secrets.ps1
cargo audit
.\Launch-MarkDownUltimate.bat
.\QTestings\smoke-local-serve.ps1
```

## Repo-local git author (optional)

```powershell
powershell -File tools\setup-git-author.ps1
```

Sets **AlexRabbit** / `AlexRabbit@users.noreply.github.com` for this repo only.
