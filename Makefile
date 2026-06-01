.PHONY: all build test lint fmt docker-up docker-down proto clean symphony

all: build

build:
	cargo build --workspace --release
	cd adapters/k8s && go build -o ../../bin/k8s-adapter ./cmd/adapter
	cd observability/anomaly-scorer && pip install -e ".[dev]" -q

test:
	cargo test --workspace
	cd adapters/k8s && go test ./...
	cd observability/anomaly-scorer && pytest -q

lint:
	cargo clippy --workspace -- -D warnings
	cargo fmt --all -- --check
	cd adapters/k8s && go vet ./...

fmt:
	cargo fmt --all
	cd adapters/k8s && gofmt -w .

proto:
	cargo run -p orchestrator-api --bin gen-stubs 2>/dev/null || echo "Proto stubs committed; regenerate with buf if installed"

docker-up:
	docker compose up -d --build

docker-down:
	docker compose down -v

symphony:
	cargo run -p symphony-bridge --release

clean:
	cargo clean
	rm -rf bin/ adapters/k8s/bin/
