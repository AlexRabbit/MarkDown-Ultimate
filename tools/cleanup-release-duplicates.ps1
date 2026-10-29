$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$remove = @(
    (Join-Path $root "release\MarkDown-Ultimate"),
    (Join-Path $root "portable"),
    (Join-Path $root "release\artifacts\MarkDown-Ultimate-Portable"),
    (Join-Path $root "release\artifacts\MarkDown-Ultimate-Portable.exe"),
    (Join-Path $root "release\artifacts\MarkDown-Ultimate-Portable.zip")
)
foreach ($p in $remove) {
    if (Test-Path $p) {
        Remove-Item -Recurse -Force $p -ErrorAction SilentlyContinue
        Write-Host "Removed stale duplicate: $p"
    }
}

# Legacy second folder (hyphen name next to repo) — no longer used.
$legacySibling = Join-Path (Split-Path $root -Parent) "MarkDown-Ultimate"
if ((Test-Path $legacySibling) -and ($legacySibling -ne $root)) {
    Write-Host "Removing legacy duplicate folder: $legacySibling"
    Remove-Item -Recurse -Force $legacySibling -ErrorAction SilentlyContinue
}

Write-Host "Project folder (everything lives here): $root"
Write-Host "Release file copies: $(Join-Path $root 'release\artifacts')"
