# Verifies local aimemos identity and optional Honcho API health.
$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$identity = Join-Path $PSScriptRoot "identity.json"
if (-not (Test-Path $identity)) {
  Write-Error "Missing aimemos/identity.json"
}
$id = Get-Content $identity -Raw | ConvertFrom-Json
$slug = if ($id.project_slug) { $id.project_slug } else { $id.workspace }
Write-Host "Project slug (local): $slug"
Write-Host "Repo path expected: $root"
if ($slug -ne "markdown-ultimate") {
  Write-Error "workspace/project_slug should be markdown-ultimate"
}
try {
  $h = Invoke-WebRequest -Uri "http://127.0.0.1:8000/health" -UseBasicParsing -TimeoutSec 3
  Write-Host "Honcho API: $($h.Content)"
} catch {
  Write-Warning "Honcho API not reachable on :8000 — run .\aimemos\activate-honcho-api.ps1"
}
Write-Host "PASS (local identity file)"
