# Architecture (MarkDown Ultimate)

## Platform scope (locked)

- **A:** Leptos CSR WASM → GitHub Pages (`/MarkDown-Ultimate/`)
- **B:** Actix engine serves `dist/` + `/api/workspace` + optional NATS publish
- **Not in scope until asked:** Dioxus `apps/`

## Modules (order)

1. `markdown_core` — Comrak GFM + Telegram preprocess + Ammonia sanitize  
2. `workspace` — tree CRUD, newest-first sort  
3. `workflow` — `.mdu.json` container  
4. `diagnostics` — JSONL + ring buffer  
5. `frontend` — UI, Driver.js tour, diagnostics modal  
6. `engine` — HTTP API  
7. `mu_persistence` — file store under `databases/data/`  
8. `system/` — Docker NATS (+ Aerospike for ops)

## Honcho

- Install: `A:\honcho`
- Project mapping: `aimemos/identity.json`, session `markdown-ultimate` for this folder (verified in Honcho config `sessions` map)
