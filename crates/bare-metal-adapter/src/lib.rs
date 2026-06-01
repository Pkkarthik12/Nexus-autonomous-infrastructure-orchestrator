//! Bare-metal adapter — SSH provisioning + IPMI power management.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BareMetalError {
    #[error("ssh failed: {0}")]
    Ssh(String),
    #[error("ipmi failed: {0}")]
    Ipmi(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostTarget {
    pub hostname: String,
    pub ipmi_host: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HostAction {
    RunCommand { command: String },
    PowerCycle,
}

#[async_trait]
pub trait BareMetalAdapter: Send + Sync {
    async fn execute(&self, host: &HostTarget, action: HostAction) -> Result<(), BareMetalError>;
}

pub struct SshIpmiAdapter {
    pub ssh_user: String,
    pub ipmi_enabled: bool,
}

#[async_trait]
impl BareMetalAdapter for SshIpmiAdapter {
    async fn execute(&self, host: &HostTarget, action: HostAction) -> Result<(), BareMetalError> {
        match action {
            HostAction::RunCommand { command } => {
                tracing::info!(host = %host.hostname, user = %self.ssh_user, %command, "ssh command (stub)");
            }
            HostAction::PowerCycle => {
                if !self.ipmi_enabled {
                    return Err(BareMetalError::Ipmi("ipmi disabled".into()));
                }
                tracing::info!(host = %host.hostname, "ipmi power cycle (stub)");
            }
        }
        Ok(())
    }
}
