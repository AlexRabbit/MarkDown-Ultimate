$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$dist = Join-Path $root "dist"
if (-not (Test-Path (Join-Path $dist "index.html"))) {
    Write-Host "Building dist (trunk --release)..."
    Push-Location (Join-Path $root "frontend")
    trunk build --release
    Pop-Location
}
$dst = Join-Path $root "apps\android\assets\web"
if (Test-Path $dst) { Remove-Item -Recurse -Force $dst }
New-Item -ItemType Directory -Force -Path $dst | Out-Null
Copy-Item -Recurse -Force (Join-Path $dist "*") $dst
Write-Host "Synced dist -> $dst"
