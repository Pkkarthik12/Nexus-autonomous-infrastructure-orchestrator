//! Chaos agent — controlled fault injection for resilience testing.

use agent_common::{connect_nats, load_config, publish_json};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config/example.yaml")]
    config: PathBuf,
}

#[derive(serde::Serialize)]
struct ChaosEvent {
    experiment: String,
    target: String,
    fault: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let args = Args::parse();
    let cfg = load_config(&args.config)?;
    let enabled = std::env::var("CHAOS_ENABLED").map(|v| v == "true").unwrap_or(false);
    if !enabled {
        tracing::info!("chaos agent idle (set CHAOS_ENABLED=true to arm)");
        tokio::signal::ctrl_c().await?;
        return Ok(());
    }
    let client = connect_nats(&cfg.nats.url).await?;
    let subject = cfg.nats.subjects.chaos.clone();
    let mut interval = tokio::time::interval(Duration::from_secs(3600));

    loop {
        interval.tick().await;
        let event = ChaosEvent {
            experiment: "pod_kill".into(),
            target: "staging/workers".into(),
            fault: "delete_random_pod".into(),
        };
        publish_json(&client, &subject, &event).await?;
        tracing::warn!(?event, "chaos experiment scheduled");
    }
}
