$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$artifacts = Join-Path $root "release\artifacts"
$out = Join-Path $artifacts "MarkDown-Ultimate-Portable"
$pyDir = Join-Path $out "python"

if (-not (Test-Path (Join-Path $root "dist\index.html"))) {
    Write-Host "dist/index.html missing - building frontend..."
    Push-Location (Join-Path $root "frontend")
    trunk build --release
    Pop-Location
}

if (Test-Path $out) {
  try { Remove-Item -Recurse -Force $out -ErrorAction Stop }
  catch { Write-Host "Note: could not remove $out (in use) — updating files in place." }
}
if (-not (Test-Path $out)) { New-Item -ItemType Directory -Path $out -Force | Out-Null }
New-Item -ItemType Directory -Path $artifacts -Force | Out-Null

Copy-Item -Recurse -Force (Join-Path $root "dist") (Join-Path $out "dist")
New-Item -ItemType Directory -Force -Path (Join-Path $out "tools") | Out-Null
Copy-Item -Force (Join-Path $root "tools\serve_dist.py") (Join-Path $out "tools\serve_dist.py")
Copy-Item -Force (Join-Path $root "README.md") $out

$embedUrl = "https://www.python.org/ftp/python/3.12.7/python-3.12.7-embed-amd64.zip"
$zipPath = Join-Path $env:TEMP "mdu-python-embed-amd64.zip"
Write-Host "Downloading embeddable Python..."
Invoke-WebRequest -Uri $embedUrl -OutFile $zipPath -UseBasicParsing
Expand-Archive -Path $zipPath -DestinationPath $pyDir -Force
Remove-Item $zipPath -Force

$pth = Join-Path $pyDir "python312._pth"
if (Test-Path $pth) {
    $content = Get-Content $pth -Raw
    if ($content -notmatch "import site") {
        Add-Content $pth "`nimport site"
    }
}

Write-Host "Building MarkDown-Ultimate-Portable.exe..."
$launcherManifest = Join-Path $root "tools\mdu-launcher\Cargo.toml"
Push-Location (Join-Path $root "tools\mdu-launcher")
cargo build --release
Pop-Location
$builtExe = Join-Path $root "tools\mdu-launcher\target\release\MarkDown-Ultimate-Portable.exe"
if (-not (Test-Path $builtExe)) {
    throw "Launcher build failed: $builtExe"
}
Copy-Item -Force $builtExe (Join-Path $out "MarkDown-Ultimate-Portable.exe")
Copy-Item -Force $builtExe (Join-Path $artifacts "MarkDown-Ultimate-Portable.exe")

Copy-Item -Force (Join-Path $root "Launch-MarkDownUltimate.bat") $out
$launcherBat = Join-Path $out "Launch-MarkDownUltimate.bat"
$batTail = @'
REM Portable bundle: prefer embedded Python next to this folder
if exist "%~dp0python\pythonw.exe" (
  set "PYEXE=%~dp0python\pythonw.exe"
  set "PYARG="
)
if exist "%~dp0python\python.exe" (
  set "PYEXE=%~dp0python\python.exe"
  set "PYARG="
)
'@
Add-Content -Path $launcherBat -Value $batTail

$zipPath = Join-Path $artifacts "MarkDown-Ultimate-Portable.zip"
if (Test-Path $zipPath) { Remove-Item -Force $zipPath }
Compress-Archive -Path (Join-Path $out "*") -DestinationPath $zipPath -Force

Write-Host "Portable bundle ready:"
Write-Host "  Folder: $out"
Write-Host "  Exe:    $(Join-Path $out 'MarkDown-Ultimate-Portable.exe')"
Write-Host "  Zip:    $zipPath"
