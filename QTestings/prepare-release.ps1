$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$dest = Join-Path $root "release\MarkDown-Ultimate"
if (Test-Path $dest) {
    Remove-Item -Recurse -Force $dest
}
New-Item -ItemType Directory -Path $dest -Force | Out-Null

# Ensure fresh static build for upload snapshot
if (-not (Test-Path (Join-Path $root "dist\index.html"))) {
    Push-Location (Join-Path $root "frontend")
    trunk build --release
    Pop-Location
}

$items = @(
    "dist", "frontend", "modules", "engine", "databases", "system", "code",
    "QTestings", "logs", "tools", "apps",
    ".env.example", "Launch-MarkDownUltimate.bat", "README.md", "LICENSE",
    "Cargo.toml", ".github"
)
foreach ($i in $items) {
    $src = Join-Path $root $i
    if (Test-Path $src) {
        Copy-Item -Recurse -Force $src (Join-Path $dest $i)
    }
}

# Strip build artifacts
Get-ChildItem -Path $dest -Recurse -Directory -Filter target -ErrorAction SilentlyContinue |
    Remove-Item -Recurse -Force
Get-ChildItem -Path $dest -Recurse -Directory -Filter ".trunk-target" -ErrorAction SilentlyContinue |
    Remove-Item -Recurse -Force
Remove-Item -Force (Join-Path $dest "dist\.trunk-lock") -ErrorAction SilentlyContinue

# Runtime / private logs
Get-ChildItem -Path (Join-Path $dest "logs\hot") -Filter "*.jsonl" -ErrorAction SilentlyContinue | Remove-Item -Force
$diagHot = Join-Path $dest "code\diagnostics\logs"
if (Test-Path $diagHot) { Remove-Item -Recurse -Force $diagHot }

# Dev-only scripts inside upload tree
@(
    (Join-Path $dest "QTestings\prepare-release.ps1")
) | ForEach-Object { if (Test-Path $_) { Remove-Item -Force $_ } }

Copy-Item -Force (Join-Path $root "release\SECURITY_CHECK.md") (Join-Path $dest "SECURITY_CHECK.md")
Copy-Item -Force (Join-Path $root "release\UPLOAD_NOTES.md") (Join-Path $dest "UPLOAD_NOTES.md")

Write-Host "Public upload folder ready: $dest"
Write-Host "Next: .\QTestings\scan-release-secrets.ps1"
