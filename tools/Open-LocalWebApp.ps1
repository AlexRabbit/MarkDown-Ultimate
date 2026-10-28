$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
$Dist = Join-Path $Root "dist"
$Port = if ($env:MDU_PORT) { $env:MDU_PORT } else { "8765" }
$Url = "http://127.0.0.1:$Port/MarkDown-Ultimate/"

if (-not (Test-Path (Join-Path $Dist "index.html"))) {
    Write-Error "dist/index.html missing. Run: trunk build --release (in frontend/)"
}

$py = Get-Command python -ErrorAction SilentlyContinue
if (-not $py) {
    $py = Get-Command py -ErrorAction SilentlyContinue
}
if (-not $py) {
    Write-Error "Python is required to run tools/serve_dist.py"
}

$existing = Get-NetTCPConnection -LocalPort $Port -State Listen -ErrorAction SilentlyContinue
if ($existing) {
    Write-Host "Port $Port already in use - opening $Url"
    Start-Process $Url
    exit 0
}

$serve = Join-Path $Root "tools\serve_dist.py"
Start-Process -WindowStyle Hidden -FilePath $py.Source -ArgumentList @($serve) -WorkingDirectory $Root
Start-Sleep -Seconds 1
Start-Process $Url
Write-Host ("MarkDown Ultimate: " + $Url)
