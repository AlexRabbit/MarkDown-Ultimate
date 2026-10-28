# Comrak vs CommonMark 0.31.2 / GFM (honest deltas)

MarkDown Ultimate renders via **Comrak** after Telegram preprocess (`||spoiler||`, `==mark==`, `<tg-*>` passthrough where allowed).

## Verified in QTestings

- GFM tables, task lists, strikethrough, fenced code, autolinks (see `QTestings/fixtures/gfm-basic.json` + `cargo test -p mu_markdown_core`).

## Known gaps (not silently claimed)

- Full **CommonMark 0.31.2** example corpus is **not** vendored; failures would be tracked in `QTestings/EVOLUTION.md`.
- Comrak may differ on nested emphasis, list tight/loose rules, and link reference edge cases.
- **Telegram RichBlock** is not a full custom parser — HTML tags in source rely on `unsafe_` + Ammonia allowlist.
- **KaTeX** runs in the browser preview only (`mduRenderMath`), not in Rust export HTML.

## When adding coverage

1. Add a JSON case under `QTestings/fixtures/`.
2. If Comrak cannot match spec, set `"skip": true` and document here + `EVOLUTION.md`.
