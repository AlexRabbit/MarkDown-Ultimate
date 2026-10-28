$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$port = if ($env:MDU_SMOKE_PORT) { $env:MDU_SMOKE_PORT } else { "18765" }
$base = "http://127.0.0.1:$port/MarkDown-Ultimate"

Get-NetTCPConnection -LocalPort $port -State Listen -ErrorAction SilentlyContinue |
    ForEach-Object { Stop-Process -Id $_.OwningProcess -Force -ErrorAction SilentlyContinue }

$js = Join-Path $root "dist\js\mdu-ui.js"
if (-not (Test-Path $js)) {
    New-Item -ItemType Directory -Force -Path (Split-Path $js) | Out-Null
    Copy-Item -Force (Join-Path $root "frontend\js\mdu-ui.js") $js
}

$env:MDU_PORT = $port
$env:MDU_QUIET = "1"
$proc = Start-Process -FilePath "python" -ArgumentList "tools\serve_dist.py" -WorkingDirectory $root -PassThru -WindowStyle Hidden
$html = $null
for ($i = 0; $i -lt 20; $i++) {
    Start-Sleep -Milliseconds 500
    try {
        $html = (Invoke-WebRequest -Uri "$base/" -UseBasicParsing -TimeoutSec 5).Content
        break
    } catch {
        if ($i -eq 19) { throw "serve_dist did not start on port $port" }
    }
}

try {
    if (-not $html) { throw "empty index response" }
    if ($html -notmatch "MarkDown Ultimate") { throw "title missing from index.html" }
    if ($html -notmatch "markdown_ultimate_frontend-[a-f0-9]+_bg\.wasm") { throw "wasm href missing" }
    $wasmName = [regex]::Match($html, "markdown_ultimate_frontend-[a-f0-9]+_bg\.wasm").Value
    $wasm = Invoke-WebRequest -Uri "$base/$wasmName" -UseBasicParsing
    if ($wasm.RawContentLength -lt 100000) { throw "wasm too small" }
    $ui = Invoke-WebRequest -Uri "$base/js/mdu-ui.js" -UseBasicParsing
    if ($ui.StatusCode -ne 200) { throw "mdu-ui.js not served" }
    Write-Host "Local serve smoke OK (port $port)"
} finally {
    if ($proc) {
        Stop-Process -Id $proc.Id -Force -ErrorAction SilentlyContinue
    }
}
