# Removes rebuildable caches and runtime junk (safe for git source tree).
$ErrorActionPreference = "Continue"
$root = Split-Path $PSScriptRoot -Parent
& (Join-Path $root "tools\cleanup-release-duplicates.ps1")

$patterns = @(
    "target",
    "MarkDownUltimate.exe.WebView2",
    "release\github-public",
    "apps\target",
    "frontend\target",
    "tools\mdu-launcher\target",
    "engine\target",
    "code\diagnostics\logs\*.jsonl"
)

Write-Host "Cleaning rebuildable caches under: $root"
Get-ChildItem -Path $root -Directory -Recurse -Force -ErrorAction SilentlyContinue |
    Where-Object { $_.Name -eq "target" -and $_.FullName -notmatch "\\\.git\\" } |
    ForEach-Object {
        Write-Host "  Removing $($_.FullName)"
        Remove-Item -Recurse -Force $_.FullName -ErrorAction SilentlyContinue
    }

foreach ($rel in $patterns) {
    $p = Join-Path $root $rel
    if ($rel -like "*\*") {
        Get-ChildItem (Split-Path $p -Parent) -Filter (Split-Path $p -Leaf) -ErrorAction SilentlyContinue |
            ForEach-Object {
                Write-Host "  Removing $($_.FullName)"
                Remove-Item -Recurse -Force $_.FullName -ErrorAction SilentlyContinue
            }
    } elseif (Test-Path $p) {
        Write-Host "  Removing $p"
        Remove-Item -Recurse -Force $p -ErrorAction SilentlyContinue
    }
}

Write-Host "Done. Rebuild with: trunk build --release, portable-bundle.ps1, build-android.ps1"
