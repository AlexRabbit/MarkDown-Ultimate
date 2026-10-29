$ErrorActionPreference = "Stop"
$base = "https://alexrabbit.github.io/MarkDown-Ultimate"
$html = Invoke-WebRequest -Uri "$base/" -UseBasicParsing -TimeoutSec 60
if ($html.StatusCode -ne 200) { throw "Pages returned $($html.StatusCode)" }
$cssHref = [regex]::Match($html.Content, 'href="([^"]+\.css)"').Groups[1].Value
if (-not $cssHref) { throw "No CSS link in Pages HTML" }
$cssUrl = if ($cssHref.StartsWith("http")) { $cssHref } else { "$base/$($cssHref.TrimStart('/'))" }
$css = (Invoke-WebRequest -Uri $cssUrl -UseBasicParsing).Content
foreach ($marker in @("sidebar-tree-scroll", "icon-sm")) {
    if ($css -notmatch [regex]::Escape($marker)) { throw "Deployed CSS missing $marker" }
}
Write-Host "Pages smoke OK: $base"
