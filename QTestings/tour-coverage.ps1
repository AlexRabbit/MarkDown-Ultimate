$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$registry = Get-Content (Join-Path $root "frontend\tour\registry.json") -Raw | ConvertFrom-Json
$appRs = Get-Content (Join-Path $root "frontend\src\app.rs") -Raw
$missing = @()
foreach ($step in $registry) {
  $sel = $step.element
  $id = $sel -replace '^#', ''
  if ($appRs -notmatch "id=`"$id`"") {
    $missing += $sel
  }
}
if ($missing.Count -gt 0) {
  Write-Error "Tour registry references missing DOM selectors: $($missing -join ', ')"
}
Write-Host "Tour coverage OK ($($registry.Count) steps)."
