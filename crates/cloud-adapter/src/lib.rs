//! Multi-cloud execution adapter (AWS / GCP / Azure).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudError {
    #[error("provider not configured: {0}")]
    NotConfigured(String),
    #[error("api error: {0}")]
    Api(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CloudProvider {
    Aws,
    Gcp,
    Azure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudAction {
    pub provider: CloudProvider,
    pub region: String,
    pub resource: String,
    pub operation: String,
}

#[async_trait]
pub trait CloudAdapter: Send + Sync {
    async fn apply(&self, action: CloudAction) -> Result<(), CloudError>;
}

pub struct MultiCloudAdapter {
    pub aws_enabled: bool,
    pub gcp_enabled: bool,
    pub azure_enabled: bool,
}

#[async_trait]
impl CloudAdapter for MultiCloudAdapter {
    async fn apply(&self, action: CloudAction) -> Result<(), CloudError> {
        let enabled = match action.provider {
            CloudProvider::Aws => self.aws_enabled,
            CloudProvider::Gcp => self.gcp_enabled,
            CloudProvider::Azure => self.azure_enabled,
        };
        if !enabled {
            return Err(CloudError::NotConfigured(format!("{:?}", action.provider)));
        }
        tracing::info!(?action, "cloud adapter apply (stub)");
        Ok(())
    }
}
