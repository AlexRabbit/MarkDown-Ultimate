$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)
Write-Host "== MarkDown Ultimate verification =="
cargo test -p mu_config -p mu_diagnostics -p mu_workflow -p mu_workspace -p mu_markdown_core
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
Write-Host "PASS (native unit suites)"
