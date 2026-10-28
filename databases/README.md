# Aerospike + file store

GitHub Pages uses browser `localStorage`. The **local engine** persists workspace JSON under `databases/data/` (portable folder store).

Optional Aerospike for local full stack:

```powershell
docker compose -f system/docker-compose.yml up -d aerospike
```

The engine uses the file store by default so development works without Docker. Aerospike container is ready for a future Aerospike-native backend; document version and namespace `mdu` in ops runbooks.
