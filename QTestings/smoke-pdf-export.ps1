$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$js = Get-Content (Join-Path $root "frontend\js\mdu-ui.js") -Raw
if ($js -notmatch "mduPrintMarkdown\s*=\s*function") { throw "mduPrintMarkdown missing" }
if ($js -notmatch "setTimeout") { throw "PDF print must defer print() via setTimeout" }
if ($js -notmatch "return true") { throw "mduPrintMarkdown must return success boolean" }
if ($js -notmatch "mduCopyText\s*=\s*function") { throw "mduCopyText missing" }
Write-Host "PDF export JS smoke OK."
