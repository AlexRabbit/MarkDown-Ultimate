# Builds a sanitized folder for manual GitHub upload (does not push).
$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$out = Join-Path $root "release\github-public"
& (Join-Path $root "tools\cleanup-release-duplicates.ps1")

if (Test-Path $out) { Remove-Item -Recurse -Force $out }
New-Item -ItemType Directory -Force -Path $out | Out-Null

$exclude = @(
    ".git", "target", "apps\target", "frontend\target", "tools\mdu-launcher\target",
    "release\github-public", "python", "code\diagnostics\logs",
    "MarkDownUltimate.exe.WebView2", "MarkDownUltimate.exe"
)

function Should-Skip($rel) {
    foreach ($e in $exclude) {
        if ($rel -like $e -or $rel -like "*\$e\*" -or $rel -like "$e\*") { return $true }
    }
    if ($rel -match "\\target\\") { return $true }
    if ($rel -match "\\\.env") { return $true }
    if ($rel -like "MarkDownUltimate.exe*") { return $true }
    return $false
}

Get-ChildItem -Path $root -Force | ForEach-Object {
    $rel = $_.Name
    if (Should-Skip $rel) { return }
    Copy-Item -Recurse -Force $_.FullName (Join-Path $out $rel)
}

if (-not (Test-Path (Join-Path $out "dist\index.html"))) {
    Push-Location (Join-Path $root "frontend")
    trunk build --release
    Pop-Location
    Copy-Item -Recurse -Force (Join-Path $root "dist") (Join-Path $out "dist")
}

& (Join-Path $PSScriptRoot "scan-release-secrets.ps1") -Root $out

Write-Host ""
Write-Host "Public upload folder ready:"
Write-Host "  $out"
Write-Host "Review README, run Launch-MarkDownUltimate.bat from this folder after building portable locally."
