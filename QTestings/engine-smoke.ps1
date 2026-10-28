$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)
Write-Host "Engine compile check"
cargo check -p markdown_ultimate_engine
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
$bind = "127.0.0.1:8080"
if ($env:ENGINE_BIND) { $bind = $env:ENGINE_BIND }
Write-Host "Optional live smoke: start engine then curl http://$bind/health"
Write-Host "PASS (compile)"
