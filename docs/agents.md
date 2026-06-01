# Agent mesh

Each agent is a standalone Rust binary subscribing to NATS subjects. They share types via `agent-common`.

## Scout (discovery)

Periodically queries adapters for inventory (pods, VMs, edge nodes) and publishes `DiscoveryEvent` messages.

```bash
cargo run -p agent-scout -- --config config/local.yaml
```

## Sentinel (anomaly detection)

Consumes metric summaries and ML scores; emits `AnomalyEvent` when thresholds exceeded.

## Planner (remediation)

Maps anomalies + active goals to `ExecutionPlan` DAGs (ordered steps with rollback hooks).

## Executor (safe rollout)

Applies plans via adapters using canary/blue-green strategies from config.

## Chaos (resilience)

When enabled, injects controlled faults (pod kill, latency, partition) per policy schedule.

**Warning:** Never enable chaos in production without blast-radius limits in `config/local.yaml`.

## Message flow

```
Scout  → nexus.discovery.*
Sentinel → nexus.anomalies.*
Planner → nexus.plans.*
Executor → nexus.execution.*
Chaos → nexus.chaos.*
All → nexus.feedback.* (observed state)
```

Nexus core aggregates feedback into the state store and triggers goal re-evaluation.
