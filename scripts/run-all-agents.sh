#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CONFIG="${ROOT}/config/example.yaml"

for a in agent-scout agent-sentinel agent-planner agent-executor agent-chaos; do
  cargo run -p "$a" -- --config "$CONFIG" &
done
wait
