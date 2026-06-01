//! Edge / IoT adapter — WASM runtime for field deployments.

use async_trait::async_trait;
use std::path::Path;
use thiserror::Error;
use wasmtime::*;

#[derive(Debug, Error)]
pub enum EdgeError {
    #[error("wasm load failed: {0}")]
    Load(String),
    #[error("wasm invoke failed: {0}")]
    Invoke(String),
}

#[async_trait]
pub trait EdgeAdapter: Send + Sync {
    async fn invoke(&self, module_path: &Path, func: &str) -> Result<i32, EdgeError>;
}

pub struct WasmtimeAdapter {
    engine: Engine,
}

impl Default for WasmtimeAdapter {
    fn default() -> Self {
        Self {
            engine: Engine::default(),
        }
    }
}

#[async_trait]
impl EdgeAdapter for WasmtimeAdapter {
    async fn invoke(&self, module_path: &Path, func: &str) -> Result<i32, EdgeError> {
        if !module_path.exists() {
            tracing::warn!(path = %module_path.display(), "wasm module missing, skipping invoke");
            return Ok(0);
        }
        let module = Module::from_file(&self.engine, module_path)
            .map_err(|e| EdgeError::Load(e.to_string()))?;
        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, &module, &[])
            .map_err(|e| EdgeError::Invoke(e.to_string()))?;
        let run = instance
            .get_typed_func::<(), i32>(&mut store, func)
            .map_err(|e| EdgeError::Invoke(e.to_string()))?;
        run.call(&mut store, ())
            .map_err(|e| EdgeError::Invoke(e.to_string()))
    }
}
