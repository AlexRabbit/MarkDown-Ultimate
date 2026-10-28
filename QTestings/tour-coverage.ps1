$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$registry = Get-Content (Join-Path $root "frontend\tour\registry.json") -Raw | ConvertFrom-Json
$srcFiles = Get-ChildItem (Join-Path $root "frontend\src") -Filter "*.rs" -Recurse | ForEach-Object { Get-Content $_.FullName -Raw }
$srcBlob = $srcFiles -join "`n"
$missing = @()
foreach ($step in $registry) {
  $sel = $step.element
  $id = $sel -replace '^#', ''
  if ($srcBlob -notmatch "id=`"$id`"") {
    $missing += $sel
  }
}
if ($missing.Count -gt 0) {
  Write-Error "Tour registry references missing DOM selectors: $($missing -join ', ')"
}
Write-Host "Tour coverage OK ($($registry.Count) steps)."
