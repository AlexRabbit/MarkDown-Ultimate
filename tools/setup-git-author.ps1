# Run once locally (repo-only). Does NOT set --global config.
# Uses GitHub noreply email — safe for public commits.
$ErrorActionPreference = "Stop"
Set-Location (Split-Path $PSScriptRoot -Parent)
git config user.name "AlexRabbit"
git config user.email "AlexRabbit@users.noreply.github.com"
Write-Host "Repo git author set to AlexRabbit (this repository only)."
