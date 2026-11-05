$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
$dist = Join-Path $root "dist"
if (-not (Test-Path (Join-Path $dist "index.html"))) {
    Write-Host "Building dist (trunk --release)..."
    Push-Location (Join-Path $root "frontend")
    trunk build --release
    Pop-Location
}
$dst = Join-Path $root "apps\android\assets\web"
if (Test-Path $dst) { Remove-Item -Recurse -Force $dst }
New-Item -ItemType Directory -Force -Path $dst | Out-Null
Copy-Item -Recurse -Force (Join-Path $dist "*") $dst
$index = Join-Path $dst "index.html"
if (Test-Path $index) {
    $html = Get-Content $index -Raw -Encoding UTF8
    $html = $html -replace '/MarkDown-Ultimate/', './'
    if ($html -notmatch 'base href') {
        $html = $html -replace '<head>', '<head><base href="./" />'
    }
    $boot = @'
<script>
(function(){try{
var m=/Android|iPhone|iPad|Mobile/i.test(navigator.userAgent)||matchMedia("(max-width:768px)").matches;
if(!m)return;
localStorage.setItem("mdu_docs_open","false");
localStorage.setItem("mdu_tools_open","false");
if(!localStorage.getItem("mdu_mobile_configured")){
  localStorage.setItem("mdu_view_mode_v1","preview");
  localStorage.setItem("mdu_mobile_configured","1");
}
document.documentElement.classList.add("mdu-mobile-ready");
}catch(e){}
})();
</script>
'@
    if ($html -notmatch 'mdu-mobile-ready') {
        $html = $html -replace '</head>', ($boot + "`n</head>")
    }
    [System.IO.File]::WriteAllText($index, $html)
}
Write-Host "Synced dist -> $dst (Android-relative paths + mobile boot)"
