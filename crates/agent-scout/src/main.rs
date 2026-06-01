//! Scout agent — infrastructure discovery.

use agent_common::{connect_nats, load_config, publish_json};
use anyhow::Result;
use clap::Parser;
use orchestrator_api::DiscoveryEvent;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
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
    let subject = cfg.nats.subjects.discovery.clone();
    let mut interval = tokio::time::interval(Duration::from_secs(30));

    loop {
        interval.tick().await;
        for env in ["production", "staging", "edge"] {
            let event = DiscoveryEvent {
                resource_id: format!("cluster/{env}"),
                kind: "kubernetes_cluster".into(),
                target_env: env.into(),
                labels: HashMap::from([("managed_by".into(), "nexus".into())]),
            };
            publish_json(&client, &subject, &event).await?;
            tracing::debug!(%env, "discovery published");
        }
    }
}
