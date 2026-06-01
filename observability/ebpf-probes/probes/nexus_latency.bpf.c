// SPDX-License-Identifier: MIT
// Nexus latency probe stub — extend with libbpf skeleton for production.

#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "MIT";

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, __u32);
    __type(value, __u64);
} latency_ns SEC(".maps");

SEC("tracepoint/syscalls/sys_enter_openat")
int trace_openat(void *ctx)
{
    __u32 pid = bpf_get_current_pid_tgid() >> 32;
    __u64 ts = bpf_ktime_get_ns();
    bpf_map_update_elem(&latency_ns, &pid, &ts, BPF_ANY);
    return 0;
}
