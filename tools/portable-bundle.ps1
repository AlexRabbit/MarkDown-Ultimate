$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$out = Join-Path $root "portable\MarkDown-Ultimate-Portable"
$pyDir = Join-Path $out "python"
if (Test-Path $out) { Remove-Item -Recurse -Force $out }
New-Item -ItemType Directory -Path $out -Force | Out-Null
Copy-Item -Recurse -Force (Join-Path $root "dist") (Join-Path $out "dist")
Copy-Item -Force (Join-Path $root "Launch-MarkDownUltimate.bat") $out
Copy-Item -Recurse -Force (Join-Path $root "tools") (Join-Path $out "tools")
Copy-Item -Force (Join-Path $root "README.md") $out

$embedUrl = "https://www.python.org/ftp/python/3.12.7/python-3.12.7-embed-amd64.zip"
$zipPath = Join-Path $env:TEMP "python-embed-amd64.zip"
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

$launcher = Join-Path $out "Launch-MarkDownUltimate.bat"
$batExtra = @"

if exist "%~dp0python\python.exe" (
  set "PYEXE=%~dp0python\python.exe"
  set "PYARG="
)
"@
Add-Content $launcher $batExtra
Write-Host "Portable bundle ready: $out"
