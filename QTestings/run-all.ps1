$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)
Write-Host "== MarkDown Ultimate verification =="
cargo test -p mu_config -p mu_diagnostics -p mu_workflow -p mu_workspace -p mu_markdown_core -p mu_persistence
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& "$PSScriptRoot\run-fixtures.ps1"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& "$PSScriptRoot\test-diagnostics-shape.ps1"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& "$PSScriptRoot\tour-coverage.ps1"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
& "$PSScriptRoot\smoke-pdf-export.ps1"
& "$PSScriptRoot\smoke-local-serve.ps1"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
$honchoScript = Join-Path $PSScriptRoot "..\aimemos\verify-honcho.ps1"
if (Test-Path $honchoScript) {
  & $honchoScript
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}
& "$PSScriptRoot\engine-smoke.ps1"
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "PASS (native suites + fixtures + tour coverage + aimemos identity + engine compile)"
