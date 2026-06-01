# Nexus Orchestrator

**Autonomous Infrastructure Orchestration Engine** — a self-healing, self-scaling distributed system controller that monitors, reasons about, and autonomously manages infrastructure across cloud, on-prem, and edge environments.

```
┌─────────────────────────────────────────────────────────────┐
│  Control Plane (Nexus core · Goal engine · State store)     │
├─────────────────────────────────────────────────────────────┤
│  Autonomous Agent Mesh (Scout · Sentinel · Planner · …)     │
├─────────────────────────────────────────────────────────────┤
│  Observability Fabric (eBPF · OTel · ML scorer · NATS)      │
├─────────────────────────────────────────────────────────────┤
│  Execution Adapters (K8s · Cloud · Bare metal · Edge WASM)  │
├─────────────────────────────────────────────────────────────┤
│  Targets (Production · Staging · Edge / IoT)                │
└─────────────────────────────────────────────────────────────┘
                              ▲
                              │ feedback loop
                              ▼
```

## Features

| Layer | Components | Technology |
|-------|------------|------------|
| **Control plane** | Nexus core, Goal engine, State store | Rust, gRPC, CRDT + Raft |
| **Agent mesh** | Scout, Sentinel, Planner, Executor, Chaos | Rust agents over NATS |
| **Observability** | eBPF probes, Metrics pipeline, Anomaly scorer, Event bus | eBPF, OpenTelemetry, Python ML, NATS |
| **Adapters** | K8s, Cloud, Bare metal, Edge | Go controller, multi-cloud SDKs, SSH/IPMI, Wasmtime |
| **Targets** | Production, Staging, Edge/IoT | Multi-region clusters, dev envs, field nodes |

## Quick start

### Prerequisites

- Rust 1.75+
- Go 1.22+ (K8s adapter)
- Python 3.11+ (anomaly scorer)
- Docker & Docker Compose

### Local development stack

```bash
cp config/example.yaml config/local.yaml
docker compose up -d
make build
cargo run -p nexus-core -- --config config/local.yaml
```

### Declare intent (example)

```bash
curl -X POST http://localhost:8080/v1/intents \
  -H "Content-Type: application/json" \
  -d '{"goal":"maintain_slo","target":"production/api","slo":{"latency_p99_ms":200,"availability":0.999}}'
```

## Repository layout

```
nexus-orchestrator/
├── crates/                 # Rust control plane, agents, adapters
├── adapters/k8s/           # Go Kubernetes controller
├── observability/          # eBPF, OTel, ML scorer, NATS configs
├── api/proto/              # gRPC / event schemas
├── deploy/                 # Helm charts & Kubernetes manifests
├── config/                 # Example configuration
└── docs/                   # Architecture & operations guides
```

## Documentation

- [Architecture](docs/architecture.md)
- [Getting started](docs/getting-started.md)
- [Agent mesh](docs/agents.md)
- [Contributing](CONTRIBUTING.md)

## Push to GitHub

```bash
git add .
git commit -m "Initial commit: Nexus autonomous infra orchestrator"
git remote add origin https://github.com/YOUR_USER/nexus-orchestrator.git
git branch -M main
git push -u origin main
```

## License

MIT — see [LICENSE](LICENSE).
