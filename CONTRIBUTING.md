# Contributing

Thank you for contributing to Nexus Orchestrator.

## Development setup

1. Install Rust, Go 1.22+, Python 3.11+, Docker.
2. `cp config/example.yaml config/local.yaml`
3. `docker compose up -d`
4. `make build && make test`

## Code style

- Rust: `cargo fmt` and `cargo clippy -- -D warnings`
- Go: `gofmt` and `go vet`
- Python: `ruff check` and `pytest`

## Pull requests

1. Fork and create a feature branch.
2. Add tests for behavioral changes.
3. Update docs when changing public APIs or architecture.
4. Ensure CI passes before requesting review.
