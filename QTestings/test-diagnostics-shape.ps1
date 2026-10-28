$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)
Write-Host "Diagnostics JSON shape via cargo test -p mu_diagnostics"
cargo test -p mu_diagnostics
exit $LASTEXITCODE
