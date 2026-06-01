# Getting started

## 1. Clone and configure

```bash
git clone https://github.com/YOUR_USER/nexus-orchestrator.git
cd nexus-orchestrator
cp config/example.yaml config/local.yaml
cp .env.example .env
```

## 2. Start dependencies

```bash
docker compose up -d
```

Verify:

- NATS: http://localhost:8222
- Prometheus: http://localhost:9090
- Anomaly scorer: http://localhost:8090/health

## 3. Build binaries

```bash
make build
```

Binaries:

- `target/release/nexus-core`
- `target/release/agent-*`
- `bin/k8s-adapter`

## 4. Run control plane

```bash
cargo run -p nexus-core -- --config config/local.yaml
```

## 5. Submit an intent

```bash
curl -X POST http://localhost:8080/v1/intents \
  -H "Content-Type: application/json" \
  -d '{
    "goal": "scale_to_capacity",
    "target": "staging/workers",
    "parameters": { "min_replicas": 2, "max_replicas": 10, "cpu_target": 70 }
  }'
```

## 6. Observe the feedback loop

Watch agent activity:

```bash
nats sub "nexus.>"
```

Check metrics: http://localhost:9091/metrics (nexus-core).

## Production deployment

Use Helm:

```bash
helm upgrade --install nexus deploy/helm/nexus \
  -f deploy/helm/nexus/values-production.yaml \
  -n nexus-system --create-namespace
```

Enable eBPF probes only on Linux nodes with appropriate capabilities (`CAP_BPF`, `CAP_PERFMON`).
