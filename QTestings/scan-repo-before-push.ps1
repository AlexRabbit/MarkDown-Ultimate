$ErrorActionPreference = "Stop"
$root = Split-Path $PSScriptRoot -Parent
Push-Location $root
try {
    $tracked = git ls-files
    if (-not $tracked) { throw "Not a git repo or no tracked files" }

    $secretPatterns = @(
        'ghp_[A-Za-z0-9]{20,}',
        'github_pat_[A-Za-z0-9_]{20,}',
        'sk-[A-Za-z0-9]{20,}',
        'AKIA[0-9A-Z]{16}',
        '-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----'
    )
    $pathPatterns = @(
        'C:\\Users\\',
        'C:/Users/'
    )

    $hits = @()
    foreach ($rel in $tracked) {
        if ($rel -match '\.(wasm|png|jpg|jpeg|gif|webp|ico|pdf|zip|apk|exe|pdb|dll)$') { continue }
        if ($rel -match '(^|/)target/') { continue }
        if ($rel -match 'code/diagnostics/src/redact\.rs$') { continue }
        if ($rel -match '(^|/)\.cursor/') { continue }
        if ($rel -match 'QTestings/scan-') { continue }

        $full = Join-Path $root $rel
        if (-not (Test-Path $full -PathType Leaf)) { continue }
        $text = Get-Content $full -Raw -ErrorAction SilentlyContinue
        if (-not $text) { continue }

        foreach ($p in $secretPatterns) {
            if ($text -match $p) {
                $hits += [pscustomobject]@{ File = $rel; Issue = "secret-pattern"; Detail = $p }
            }
        }
        foreach ($p in $pathPatterns) {
            if ($text -match [regex]::Escape($p)) {
                $hits += [pscustomobject]@{ File = $rel; Issue = "local-path"; Detail = $p }
            }
        }
    }

    if ($hits.Count -gt 0) {
        $hits | Format-Table -AutoSize
        throw "Repo scan found $($hits.Count) issue(s). Fix before push."
    }
    Write-Host "Repo privacy scan OK ($($tracked.Count) tracked files checked)."
} finally {
    Pop-Location
}
