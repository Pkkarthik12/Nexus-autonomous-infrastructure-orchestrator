//! Sentinel agent — anomaly detection from metrics + ML scorer.

use agent_common::{connect_nats, load_config, publish_json};
use anyhow::Result;
use clap::Parser;
use orchestrator_api::AnomalyEvent;
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
    let subject = cfg.nats.subjects.anomalies.clone();
    let http = reqwest::Client::new();
    let scorer_url = std::env::var("ANOMALY_SCORER_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8090".into());
    let mut interval = tokio::time::interval(Duration::from_secs(15));

    loop {
        interval.tick().await;
        let score: f64 = http
            .get(format!("{scorer_url}/v1/score"))
            .send()
            .await
            .ok()
            .and_then(|r| r.json().ok())
            .and_then(|v: serde_json::Value| v.get("score")?.as_f64())
            .unwrap_or(1.5);

        if score > 3.0 {
            let event = AnomalyEvent {
                metric: "latency_p99".into(),
                target: "production/api".into(),
                score,
                severity: "high".into(),
            };
            publish_json(&client, &subject, &event).await?;
            tracing::warn!(%score, "anomaly detected");
        }
    }
}
