$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$stale = Join-Path $root "release\MarkDown-Ultimate"
$portable = Join-Path $root "portable"
foreach ($p in @($stale, $portable)) {
    if (Test-Path $p) {
        Remove-Item -Recurse -Force $p
        Write-Host "Removed duplicate tree: $p"
    }
}
Write-Host "Canonical project root only: $root"
Write-Host "Release assets only: $(Join-Path $root 'release\artifacts')"
