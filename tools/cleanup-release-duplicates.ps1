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
Write-Host "Dev repo (source): $root"
Write-Host "Portable app (only copy to run): $(if ($env:MDU_PORTABLE_ROOT) { $env:MDU_PORTABLE_ROOT } else { Join-Path (Split-Path $root -Parent) 'MarkDown-Ultimate' })"
Write-Host "GitHub staging only: $(Join-Path $root 'release\artifacts') (exe + zip + apk, not a full tree)"
