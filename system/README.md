# NATS + JetStream (local stack)

Start messaging for engine workspace events:

```powershell
docker compose -f system/docker-compose.yml up -d nats
```

JetStream is enabled (`-js`). The Actix engine publishes `mdu.workspace.updated` on `PUT /api/workspace` when `NATS_URL` is set in `.env`.
