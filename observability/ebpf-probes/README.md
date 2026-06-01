# eBPF probes

Kernel telemetry for the Observability Fabric (Linux only).

## Components

- `probes/nexus_latency.bpf.c` — syscall latency histogram (libbpf skeleton stub)
- `loader/` — attach probes via CO-RE when built with BTF-enabled kernel

## Build (requires libbpf, clang, llvm)

```bash
cd observability/ebpf-probes
make
```

## Deploy

Mount compiled BPF objects into Nexus agents with `observability.ebpf.enabled: true` in config.

Capabilities required: `CAP_BPF`, `CAP_PERFMON`, `CAP_SYS_ADMIN` (distro-dependent).
