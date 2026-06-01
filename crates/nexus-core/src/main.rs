mod api;
mod config;
mod feedback;
mod orchestrator;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Parser)]
#[command(name = "nexus-core", about = "Nexus Autonomous Infrastructure Orchestrator")]
struct Args {
    #[arg(short, long, default_value = "config/example.yaml")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "nexus_core=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    let cfg = config::AppConfig::load(&args.config)?;
    orchestrator::run(cfg).await
}
