$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$out = Join-Path $root "portable\MarkDown-Ultimate-Portable"
if (Test-Path $out) { Remove-Item -Recurse -Force $out }
New-Item -ItemType Directory -Path $out -Force | Out-Null
Copy-Item -Recurse -Force (Join-Path $root "dist") (Join-Path $out "dist")
Copy-Item -Force (Join-Path $root "Launch-MarkDownUltimate.bat") $out
Copy-Item -Recurse -Force (Join-Path $root "tools") (Join-Path $out "tools")
Copy-Item -Force (Join-Path $root "README.md") $out
Write-Host "Portable folder (needs Python on PATH): $out"
