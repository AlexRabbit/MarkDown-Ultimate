$ErrorActionPreference = "Stop"
$url = if ($env:MDU_SMOKE_URL) { $env:MDU_SMOKE_URL } else { "https://alexrabbit.github.io/MarkDown-Ultimate/" }
Write-Host "Smoke check: $url"
$r = Invoke-WebRequest -Uri $url -UseBasicParsing -TimeoutSec 60
if ($r.StatusCode -ne 200) { throw "HTTP $($r.StatusCode)" }
$body = $r.Content
$js = (Invoke-WebRequest -Uri ($url.TrimEnd('/') + '/js/mdu-ui.js') -UseBasicParsing -TimeoutSec 60).Content
$needles = @("MarkDown Ultimate", "markdown_ultimate_frontend", "mduInitAppShortcuts")
$missing = @()
foreach ($n in $needles) {
  $hay = if ($n -eq "mduInitAppShortcuts") { $js } else { $body }
  if ($hay -notmatch [regex]::Escape($n)) { $missing += $n }
}
if ($missing.Count -gt 0) {
  Write-Error "Live page missing expected markers: $($missing -join ', '). Wait for GitHub Pages deploy or hard-refresh (Ctrl+F5)."
}
Write-Host "Pages smoke OK."
