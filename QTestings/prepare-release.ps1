$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$dest = Join-Path $root "release\MarkDown-Ultimate"
if (Test-Path $dest) { Remove-Item -Recurse -Force $dest }
New-Item -ItemType Directory -Path $dest | Out-Null
$items = @(
  "dist", "frontend", "modules", "engine", "databases", "system", "code",
  "QTestings", "logs", "aimemos", "bot", "apps",
  ".env.example", "Launch-MarkDownUltimate.bat", "README.md", "LICENSE",
  "Cargo.toml", ".github"
)
foreach ($i in $items) {
  $src = Join-Path $root $i
  if (Test-Path $src) {
    Copy-Item -Recurse -Force $src (Join-Path $dest $i)
  }
}
# Strip build artifacts from copy
Get-ChildItem -Path $dest -Recurse -Directory -Filter target -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force
Write-Host "Release folder: $dest"
