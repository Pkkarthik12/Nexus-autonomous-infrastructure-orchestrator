use agent_common::NexusConfig;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    #[serde(flatten)]
    pub nexus: NexusConfig,
    pub state_store: StateStoreConfig,
    pub feedback_loop: FeedbackConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub http_addr: String,
    pub grpc_addr: String,
    pub metrics_addr: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StateStoreConfig {
    pub raft: RaftConfig,
    pub crdt: CrdtConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RaftConfig {
    pub bind_addr: String,
    pub peers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CrdtConfig {
    pub sync_interval_ms: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FeedbackConfig {
    pub enabled: bool,
    pub reconcile_interval_secs: u64,
    pub drift_threshold: f64,
}

impl AppConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        serde_yaml::from_str(&text).context("parse config")
    }
}
