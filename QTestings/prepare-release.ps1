$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
& (Join-Path $root "tools\cleanup-release-duplicates.ps1")
$artifacts = Join-Path $root "release\artifacts"
New-Item -ItemType Directory -Path $artifacts -Force | Out-Null

Write-Host "=== MarkDown Ultimate release artifacts (repo root build) ==="
Write-Host "Output: $artifacts"
Write-Host "Note: GitHub Releases host downloadable assets; this folder is gitignored staging only."
Write-Host ""

if (-not (Test-Path (Join-Path $root "dist\index.html"))) {
    Write-Host "Building frontend (trunk --release)..."
    Push-Location (Join-Path $root "frontend")
    trunk build --release
    Pop-Location
}

Write-Host "Building portable Windows bundle + MarkDown-Ultimate-Portable.exe..."
& (Join-Path $root "tools\portable-bundle.ps1")

Write-Host "Attempting Android APK (optional)..."
$androidScript = Join-Path $PSScriptRoot "build-android.ps1"
if (Test-Path $androidScript) {
    & $androidScript
}

Write-Host "Secret scan on artifacts..."
& (Join-Path $PSScriptRoot "scan-release-secrets.ps1") -Root $artifacts

Write-Host ""
Write-Host "Done. Upload assets from:"
Get-ChildItem -Path $artifacts -File | ForEach-Object { Write-Host "  $($_.FullName)" }
Write-Host ""
Write-Host "GitHub Releases: https://github.com/AlexRabbit/MarkDown-Ultimate/releases"
Write-Host "Optional: .\QTestings\create-github-release.ps1 -Tag v0.2.2"
