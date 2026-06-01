# Architecture

Nexus Orchestrator implements a five-layer stack with a continuous **feedback loop** from targets back to the control plane.

## Control plane

### Nexus core (Rust)

Central orchestrator: loads configuration, wires agents, exposes HTTP/gRPC APIs, and drives the feedback reconciler.

### Goal engine

Translates declarative **intents** (SLOs, capacity targets, compliance policies) into concrete **goals** and prioritized work items for the agent mesh.

### State store

- **CRDT layer**: eventually-consistent replica of discovered resources and desired state.
- **Raft layer**: strongly-consistent cluster metadata and leadership for control-plane HA.

## Autonomous agent mesh

| Agent | Responsibility | Input | Output |
|-------|----------------|-------|--------|
| **Scout** | Discovery | Adapter telemetry | Resource graph events |
| **Sentinel** | Anomaly detection | Metrics + ML scores | Anomaly events |
| **Planner** | Remediation | Anomalies + goals | Execution plans |
| **Executor** | Safe rollout | Plans | Adapter commands |
| **Chaos** | Resilience | Schedules + policies | Fault injections |

All agents communicate over **NATS JetStream** subjects defined in `config/example.yaml`.

## Observability fabric

1. **eBPF probes** — kernel-level syscall/network latency telemetry (Linux).
2. **Metrics pipeline** — OpenTelemetry collectors → Prometheus.
3. **Anomaly scorer** — Python service scoring metric windows (z-score + isolation forest stub).
4. **Event bus** — NATS streaming for cross-component events.

## Execution adapters

| Adapter | Targets | Mechanism |
|---------|---------|-----------|
| K8s | Clusters | Go controller + client-go |
| Cloud | AWS/GCP/Azure | Rust SDK wrappers |
| Bare metal | Physical hosts | SSH + IPMI |
| Edge | IoT / field | Wasmtime modules |

## Feedback loop

```
Targets → Adapters → Observability → Sentinel/Scout
    → State store → Goal engine → Planner → Executor → Targets
```

The reconciler in Nexus core compares **observed** vs **desired** state every `reconcile_interval_secs` and emits drift events when threshold exceeded.

## Security model (overview)

- mTLS between control plane components (production).
- RBAC on Kubernetes adapter service account.
- Chaos agent disabled by default; requires explicit `agents.chaos.enabled`.

See [getting-started.md](getting-started.md) for deployment paths.
