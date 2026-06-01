//! Shared API types for Nexus Orchestrator (HTTP + events).
//! gRPC stubs can be generated from `api/proto` with buf/prost.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub goal: String,
    pub target: String,
    #[serde(default)]
    pub parameters: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub slo: Option<Slo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slo {
    pub availability: Option<f64>,
    pub latency_p99_ms: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitIntentResponse {
    pub intent_id: Uuid,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub target: String,
    pub observed_hash: String,
    pub desired_hash: String,
    pub drift: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NexusEvent {
    Discovery(DiscoveryEvent),
    Anomaly(AnomalyEvent),
    Plan(ExecutionPlan),
    Execution(ExecutionResult),
    Feedback(FeedbackEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryEvent {
    pub resource_id: String,
    pub kind: String,
    pub target_env: String,
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyEvent {
    pub metric: String,
    pub target: String,
    pub score: f64,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub plan_id: Uuid,
    pub steps: Vec<PlanStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub adapter: String,
    pub action: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub plan_id: Uuid,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackEvent {
    pub target: String,
    pub observed_hash: String,
    pub drift: f64,
}
