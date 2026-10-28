$ErrorActionPreference = "Stop"
$script = "A:\honcho\aimemos\start-stack.ps1"
if (-not (Test-Path $script)) { throw "Honcho install missing at A:\honcho" }
try {
  $h = Invoke-WebRequest -Uri "http://127.0.0.1:8000/health" -UseBasicParsing -TimeoutSec 3
  if ($h.StatusCode -eq 200) {
    Write-Host "Honcho API already healthy at http://127.0.0.1:8000"
    exit 0
  }
} catch {}
Write-Host "Starting Honcho WSL stack..."
& powershell -ExecutionPolicy Bypass -File $script
exit $LASTEXITCODE
