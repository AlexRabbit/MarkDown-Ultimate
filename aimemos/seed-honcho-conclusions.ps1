$ErrorActionPreference = "Stop"
$api = "http://127.0.0.1:8000/v3"
$ws = "markdown-ultimate"
$session = "aimemos-markdown-ultimate"
& (Join-Path $PSScriptRoot "activate-honcho-api.ps1")
$headers = @{ Authorization = "Bearer local-cursor"; "Content-Type" = "application/json" }
$facts = @(
  "MDU platform A+B: static dist on GitHub Pages + optional Actix engine; no Dioxus unless requested.",
  "MDU verify gate: .\QTestings\run-all.ps1 (cargo, fixtures, tour coverage, engine compile).",
  "MDU UI entry: frontend/src/app.rs; workspace sort by updated_at in modules/workspace.",
  "MDU Honcho identity: aimemos/identity.json maps workspace markdown-ultimate session aimemos-markdown-ultimate peer robots.",
  "MDU agent bootstrap: read aimemos/AGENT_MEMORY.md first; Honcho mirrors in aimemos/*.md.",
  "MDU Pages deploy 404: enable GitHub Pages from Actions in repo Settings, then re-run pages workflow.",
  "MDU release: QTestings/prepare-release.ps1 -> release/MarkDown-Ultimate/; see release/SECURITY_CHECK.md."
)
foreach ($f in $facts) {
  $body = @{
    conclusions = @(
      @{
        content        = $f
        observer_id    = "cursor"
        observed_id    = "robots"
        session_id     = $session
      }
    )
  } | ConvertTo-Json -Depth 5
  Invoke-RestMethod -Method Post -Uri "$api/workspaces/$ws/conclusions" -Headers $headers -Body $body | Out-Null
}
Write-Host "Seeded $($facts.Count) conclusions to $ws / $session"
