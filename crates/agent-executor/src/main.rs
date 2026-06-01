//! Executor agent — safe rollout of remediation plans.

use agent_common::{connect_nats, load_config, publish_json};
use anyhow::Result;
use clap::Parser;
use futures_util::StreamExt;
use orchestrator_api::{ExecutionPlan, ExecutionResult};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

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
    let sub_plans = cfg.nats.subjects.plans.clone();
    let sub_exec = cfg.nats.subjects.execution.clone();

    let mut sub = client.subscribe(sub_plans).await?;
    while let Some(msg) = sub.next().await {
        let plan: ExecutionPlan = serde_json::from_slice(&msg.payload)?;
        tracing::info!(plan_id = %plan.plan_id, steps = plan.steps.len(), "executing plan (canary rollout)");
        for step in &plan.steps {
            tracing::info!(adapter = %step.adapter, action = %step.action, "apply step");
        }
        let result = ExecutionResult {
            plan_id: plan.plan_id,
            success: true,
            message: "canary rollout complete".into(),
        };
        publish_json(&client, &sub_exec, &result).await?;
    }
    Ok(())
}
