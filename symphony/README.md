# Symphony of the Stack

**Turn your infrastructure into music and motion.** Live request rates drive tempo, latency shapes pitch, errors add dissonance, anomalies shift timbre, and chaos experiments hit like a drum.

Part of [Nexus Orchestrator](../README.md) — works standalone or wired to NATS + the anomaly scorer.

```
  Metrics (demo / NATS / HTTP)  →  symphony-bridge  →  WebSocket  →  Browser
                                        │                              │
                                        └──────── static UI ───────────┘
                                              Web Audio + canvas
```

## Quick start (local)

```bash
# From repository root
cargo run -p symphony-bridge
```

Open **http://localhost:8765**, click **Start audio**, and watch the demo orchestra react to synthetic production traffic.

## Docker

```bash
docker compose up -d symphony
# UI at http://localhost:8765
```

## Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `SYMPHONY_HTTP_ADDR` | `0.0.0.0:8765` | HTTP + WebSocket listen address |
| `SYMPHONY_WEB_DIR` | `symphony/web` | Static UI assets |
| `SYMPHONY_DEMO` | `true` | Synthetic metrics when no live publishers |
| `NATS_URL` | — | Subscribe to Nexus event subjects |
| `SYMPHONY_NATS_SUBJECTS` | `nexus.anomalies.>,nexus.chaos.>,nexus.feedback.>` | Comma-separated subjects |
| `SYMPHONY_ANOMALY_URL` | — | Poll anomaly scorer (e.g. `http://127.0.0.1:8090`) |
| `SYMPHONY_TICK_MS` | `100` | Broadcast interval (ms) |

## Sonification map

| Signal | Sound | Visual |
|--------|-------|--------|
| Request rate | Arpeggio tempo | Particle speed |
| p99 latency | Root pitch | Hue / stress |
| Error rate | Detuned layer | Particle glow |
| Anomaly score | Filter brightness & Q | Color shift |
| Chaos event | Square drum hit | Expanding ring |

## WebSocket protocol

Connect to `ws://localhost:8765/ws`. Each message is JSON:

```json
{
  "ts_ms": 1717200000123,
  "target": "production/api",
  "request_rate": 1240.5,
  "latency_p99_ms": 87.2,
  "error_rate": 0.004,
  "anomaly_score": 1.8,
  "chaos": false,
  "event": null
}
```

## Integrate with Nexus

1. Start the stack: `docker compose up -d`
2. Symphony reads NATS chaos/anomaly/feedback subjects and polls the anomaly scorer.
3. Arm chaos for a dramatic moment: `CHAOS_ENABLED=true` and run the chaos agent.

## License

MIT — same as the parent project.
