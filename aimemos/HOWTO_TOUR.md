# How to add a tour step

1. Add a stable DOM id to the control in `frontend/src/app.rs`.
2. Add an entry to `frontend/tour/registry.json` with `"element": "#yourId"`.
3. Run `.\QTestings\tour-coverage.ps1`.

The tour never auto-starts; only `#btnHowTo` calls `mduStartTour`.
