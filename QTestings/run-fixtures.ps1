$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)
Write-Host "GFM/CommonMark spot checks run via: cargo test -p mu_markdown_core"
cargo test -p mu_markdown_core
exit $LASTEXITCODE
