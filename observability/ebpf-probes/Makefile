CLANG ?= clang
ARCH := $(shell uname -m | sed 's/x86_64/x86/' | sed 's/aarch64/arm64/')

.PHONY: all clean

all: probes/nexus_latency.bpf.o

probes/nexus_latency.bpf.o: probes/nexus_latency.bpf.c
	$(CLANG) -g -O2 -target bpf -D__TARGET_ARCH_$(ARCH) -c $< -o $@

clean:
	rm -f probes/*.bpf.o
