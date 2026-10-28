# Diagnostics architecture (MarkDown Ultimate)

## Scope

This tranche implements schema v2 events, ring buffer (512), native JSONL under `LOGS_DIR/hot/events.jsonl`, WASM console mirroring, and an in-app **Diagnostics** modal (`recent_events`).

## Layout

| Path | Role |
| --- | --- |
| `logs/hot/` | Append-only JSONL |
| `logs/incidents/` | Reserved for bundled incident exports |
| `logs/meta/logger-health.json` | Dropped-event counter (native) |
| `logs/archives/` | Reserved for rotation |

## Event fields

`schema_version`, `event_id`, `trace_id`, `operation_id`, `parent_event_id`, `severity`, `category`, `event_type`, `component`, `message`, optional `context` / `error`.

## Backlog (honest)

Full forensic megaprompt items not yet implemented: incident auto-bundling UI, distributed trace export, anomaly detection, regression latency baselines, live WebSocket stream, redaction pipeline tests, crash minidump hook on native.

## Extension

Call `mu_diagnostics::emit_with_parent` for correlated sub-operations.
