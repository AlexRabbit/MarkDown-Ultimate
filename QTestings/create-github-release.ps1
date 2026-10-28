param(
    [string]$Tag = "v0.2.2",
    [string]$Title = "v0.2.2",
    [string]$NotesFile = ""
)
$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$artifacts = Join-Path $root "release\artifacts"
if (-not $NotesFile) {
    $NotesFile = Join-Path $root "release\RELEASE_NOTES.md"
}

$gh = Get-Command gh -ErrorAction SilentlyContinue
if (-not $gh) {
    $candidates = @(
        "${env:ProgramFiles}\GitHub CLI\gh.exe",
        "${env:LocalAppData}\Programs\GitHub CLI\gh.exe"
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $gh = $c; break }
    }
}
if (-not $gh) {
    Write-Warning "GitHub CLI (gh) not found. Install from https://cli.github.com/ then run: gh auth login"
    Write-Host "Manual upload: https://github.com/AlexRabbit/MarkDown-Ultimate/releases/new"
    Get-ChildItem -Path $artifacts -File | ForEach-Object { Write-Host "  Attach: $($_.Name)" }
    exit 2
}

$assets = @(
    (Join-Path $artifacts "MarkDown-Ultimate-Portable.exe"),
    (Join-Path $artifacts "MarkDown-Ultimate-Portable.zip"),
    (Join-Path $artifacts "MarkDown-Ultimate.apk")
) | Where-Object { Test-Path $_ }

if ($assets.Count -eq 0) {
    $portableExe = Join-Path $artifacts "MarkDown-Ultimate-Portable\MarkDown-Ultimate-Portable.exe"
    $portableZip = Join-Path $artifacts "MarkDown-Ultimate-Portable.zip"
    if (Test-Path $portableExe) {
        Copy-Item -Force $portableExe (Join-Path $artifacts "MarkDown-Ultimate-Portable.exe")
        $assets += (Join-Path $artifacts "MarkDown-Ultimate-Portable.exe")
    }
    if (Test-Path $portableZip) { $assets += $portableZip }
}

if ($assets.Count -eq 0) {
    throw "No release assets in $artifacts - run prepare-release.ps1 first"
}

$notesArg = if (Test-Path $NotesFile) { @("--notes-file", $NotesFile) } else { @("--notes", "MarkDown Ultimate release") }

Push-Location $root
try {
    $existing = & $gh release view $Tag 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Release $Tag exists - uploading assets..."
        foreach ($a in $assets) {
            & $gh release upload $Tag $a --clobber
        }
    } else {
        & $gh release create $Tag @assets @notesArg --title $Title
    }
    $url = & $gh release view $Tag --json url -q .url
    Write-Host "Release URL: $url"
} finally {
    Pop-Location
}
