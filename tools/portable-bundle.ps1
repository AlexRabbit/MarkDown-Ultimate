$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
& (Join-Path $root "tools\cleanup-release-duplicates.ps1")

# ONE portable install folder (not nested under release/, not a second copy of the repo).
$portableRoot = if ($env:MDU_PORTABLE_ROOT) { $env:MDU_PORTABLE_ROOT } else { "A:\VibeCode\MarkDown-Ultimate" }
$artifacts = Join-Path $root "release\artifacts"
$pyDir = Join-Path $portableRoot "python"

if (-not (Test-Path (Join-Path $root "dist\index.html"))) {
    Write-Host "dist/index.html missing - building frontend..."
    Push-Location (Join-Path $root "frontend")
    trunk build --release
    Pop-Location
}
if (-not (Test-Path (Join-Path $root "dist\js\mdu-ui.js"))) {
    New-Item -ItemType Directory -Force -Path (Join-Path $root "dist\js") | Out-Null
    Copy-Item -Force (Join-Path $root "frontend\js\mdu-ui.js") (Join-Path $root "dist\js\mdu-ui.js")
}

New-Item -ItemType Directory -Path $portableRoot -Force | Out-Null
New-Item -ItemType Directory -Path $artifacts -Force | Out-Null

# Sync runtime files into the single portable root (update in place).
Copy-Item -Recurse -Force (Join-Path $root "dist") (Join-Path $portableRoot "dist")
New-Item -ItemType Directory -Force -Path (Join-Path $portableRoot "tools") | Out-Null
Copy-Item -Force (Join-Path $root "tools\serve_dist.py") (Join-Path $portableRoot "tools\serve_dist.py")
Copy-Item -Force (Join-Path $root "README.md") $portableRoot -ErrorAction SilentlyContinue

if (-not (Test-Path (Join-Path $pyDir "python.exe"))) {
    $embedUrl = "https://www.python.org/ftp/python/3.12.7/python-3.12.7-embed-amd64.zip"
    $zipPath = Join-Path $env:TEMP "mdu-python-embed-amd64.zip"
    Write-Host "Downloading embeddable Python (once)..."
    Invoke-WebRequest -Uri $embedUrl -OutFile $zipPath -UseBasicParsing
    New-Item -ItemType Directory -Force -Path $pyDir | Out-Null
    Expand-Archive -Path $zipPath -DestinationPath $pyDir -Force
    Remove-Item $zipPath -Force
    $pth = Join-Path $pyDir "python312._pth"
    if (Test-Path $pth) {
        $content = Get-Content $pth -Raw
        if ($content -notmatch "import site") {
            Add-Content $pth "`nimport site"
        }
    }
}

Write-Host "Building MarkDownUltimate.exe (native window, no browser, no console)..."
Push-Location (Join-Path $root "tools\mdu-launcher")
cargo build --release
Pop-Location
$builtExe = Join-Path $root "tools\mdu-launcher\target\release\MarkDownUltimate.exe"
if (-not (Test-Path $builtExe)) {
    throw "Launcher build failed: $builtExe"
}

Copy-Item -Force $builtExe (Join-Path $portableRoot "MarkDownUltimate.exe")
Copy-Item -Force $builtExe (Join-Path $artifacts "MarkDownUltimate.exe")

$zipPath = Join-Path $artifacts "MarkDownUltimate-Portable.zip"
if (Test-Path $zipPath) { Remove-Item -Force $zipPath }
Compress-Archive -Path (Join-Path $portableRoot "*") -DestinationPath $zipPath -Force

Write-Host ""
Write-Host "Single portable install (run this):"
Write-Host "  $portableRoot\MarkDownUltimate.exe"
Write-Host "GitHub upload copies:"
Write-Host "  $artifacts\MarkDownUltimate.exe"
Write-Host "  $zipPath"
