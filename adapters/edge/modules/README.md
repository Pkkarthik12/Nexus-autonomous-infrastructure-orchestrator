# Edge WASM modules

Place compiled `.wasm` modules here for the edge adapter (Wasmtime).

Example build (Rust):

```bash
cargo build --target wasm32-wasi --release
cp target/wasm32-wasi/release/edge_task.wasm .
```

Expected export: `run` function returning i32 status code.
