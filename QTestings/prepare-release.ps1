$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$dest = Join-Path $root "release\MarkDown-Ultimate"
if (Test-Path $dest) {
    Remove-Item -Recurse -Force $dest
}
New-Item -ItemType Directory -Path $dest -Force | Out-Null

$items = @(
    "dist", "frontend", "modules", "engine", "databases", "system", "code",
    "QTestings", "logs", "bot", "apps", "tools",
    ".env.example", "Launch-MarkDownUltimate.bat", "README.md", "LICENSE",
    "Cargo.toml", ".github"
)
foreach ($i in $items) {
    $src = Join-Path $root $i
    if (Test-Path $src) {
        Copy-Item -Recurse -Force $src (Join-Path $dest $i)
    }
}

# Remove build artifacts and local-only clutter from the upload tree
Get-ChildItem -Path $dest -Recurse -Directory -Filter target -ErrorAction SilentlyContinue |
    Remove-Item -Recurse -Force
Get-ChildItem -Path $dest -Recurse -Directory -Filter ".trunk-target" -ErrorAction SilentlyContinue |
    Remove-Item -Recurse -Force
Remove-Item -Force (Join-Path $dest "dist\.trunk-lock") -ErrorAction SilentlyContinue

$logHot = Join-Path $dest "logs\hot"
Get-ChildItem -Path $logHot -Filter "*.jsonl" -ErrorAction SilentlyContinue | Remove-Item -Force
$diagHot = Join-Path $dest "code\diagnostics\logs"
if (Test-Path $diagHot) {
    Remove-Item -Recurse -Force $diagHot
}

# Dev-only paths (not for public GitHub)
@(
    (Join-Path $dest "QTestings\prepare-release.ps1")
) | ForEach-Object {
    if (Test-Path $_) { Remove-Item -Force $_ }
}

Copy-Item -Force (Join-Path $root "release\SECURITY_CHECK.md") (Join-Path $dest "SECURITY_CHECK.md") -ErrorAction SilentlyContinue
Copy-Item -Force (Join-Path $root "release\UPLOAD_NOTES.md") (Join-Path $dest "UPLOAD_NOTES.md") -ErrorAction SilentlyContinue

Write-Host "Public upload folder ready: $dest"
