# Diagnostics layout

| Path | Purpose |
| --- | --- |
| `logs/hot/events.jsonl` | Live structured events (native builds) |
| `logs/incidents/` | Preserved incident bundles |
| `logs/meta/logger-health.json` | Self-observability counters |
| `logs/archives/` | Rotated JSONL |

Event schema: see `code/diagnostics/src/lib.rs` (`schema_version: 2`, PII redaction on emit).

| Path | Purpose |
| --- | --- |
| `logs/hot/serve.log` | Local Python static server access log (when `MDU_QUIET=1`) |

WASM builds log to browser console and an in-memory ring (512 events) via `mu_diagnostics`.
