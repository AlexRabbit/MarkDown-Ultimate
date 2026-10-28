param(
    [string]$Root = (Join-Path (Split-Path $PSScriptRoot -Parent) "release\MarkDown-Ultimate")
)
$ErrorActionPreference = "Stop"
if (-not (Test-Path $Root)) {
    throw "Release folder missing. Run prepare-release.ps1 first: $Root"
}

$patterns = @(
    'ghp_[A-Za-z0-9]{20,}',
    'github_pat_[A-Za-z0-9_]{20,}',
    'sk-[A-Za-z0-9]{20,}',
    'AKIA[0-9A-Z]{16}',
    '-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----',
    'password\s*=\s*[^\s#]+',
    'api[_-]?key\s*=\s*[^\s#]+'
)

$hits = @()
Get-ChildItem -Path $Root -Recurse -File -ErrorAction SilentlyContinue | ForEach-Object {
    if ($_.Extension -match '\.(wasm|pdb|exe|zip)$') { return }
    if ($_.FullName -match '\\code\\diagnostics\\src\\redact\.rs$') { return }
    $text = Get-Content $_.FullName -Raw -ErrorAction SilentlyContinue
    if (-not $text) { return }
    foreach ($p in $patterns) {
        if ($text -match $p) {
            $hits += [pscustomobject]@{ File = $_.FullName.Replace($Root, ''); Pattern = $p }
        }
    }
}

if ($hits.Count -gt 0) {
    $hits | Format-Table -AutoSize
    throw "Secret scan found $($hits.Count) potential hit(s). Review before upload."
}
Write-Host "Secret scan OK: $Root"
