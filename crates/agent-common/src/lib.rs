//! Shared agent runtime: config loading, NATS publish/subscribe helpers.

use anyhow::{Context, Result};
use async_nats::Client;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct NexusConfig {
    pub nats: NatsConfig,
    #[serde(default)]
    pub agents: AgentsConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NatsConfig {
    pub url: String,
    pub stream: String,
    pub subjects: NatsSubjects,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NatsSubjects {
    pub discovery: String,
    pub anomalies: String,
    pub plans: String,
    pub execution: String,
    pub chaos: String,
    pub feedback: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AgentsConfig {
    pub scout: Option<serde_json::Value>,
    pub sentinel: Option<serde_json::Value>,
    pub planner: Option<serde_json::Value>,
    pub executor: Option<serde_json::Value>,
    pub chaos: Option<serde_json::Value>,
}

pub fn load_config(path: &Path) -> Result<NexusConfig> {
    let text = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    serde_yaml::from_str(&text).context("parse config yaml")
}

pub async fn connect_nats(url: &str) -> Result<Client> {
    async_nats::connect(url)
        .await
        .with_context(|| format!("connect nats at {url}"))
}

pub async fn publish_json(client: &Client, subject: &str, value: &impl serde::Serialize) -> Result<()> {
    let bytes = serde_json::to_vec(value)?;
    client.publish(subject.to_string(), bytes.into()).await?;
    Ok(())
}
