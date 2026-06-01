//! Planner agent — remediation plan generation.

use agent_common::{connect_nats, load_config, publish_json};
use futures_util::StreamExt;
use anyhow::Result;
use clap::Parser;
use orchestrator_api::{ExecutionPlan, PlanStep};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config/example.yaml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Args::parse();
    let cfg = load_config(&args.config)?;
    let client = connect_nats(&cfg.nats.url).await?;
    let sub_anomaly = cfg.nats.subjects.anomalies.clone();
    let sub_plan = cfg.nats.subjects.plans.clone();

    let mut sub = client.subscribe(sub_anomaly).await?;
    while let Some(msg) = sub.next().await {
        let _payload = String::from_utf8_lossy(&msg.payload);
        let plan = ExecutionPlan {
            plan_id: Uuid::new_v4(),
            steps: vec![
                PlanStep {
                    adapter: "kubernetes".into(),
                    action: "scale".into(),
                    payload: serde_json::json!({ "replicas": 3 }),
                },
                PlanStep {
                    adapter: "kubernetes".into(),
                    action: "rollout_restart".into(),
                    payload: serde_json::json!({ "deployment": "api" }),
                },
            ],
        };
        publish_json(&client, &sub_plan, &plan).await?;
        tracing::info!(plan_id = %plan.plan_id, "remediation plan published");
    }
    Ok(())
}
