use crate::api::{router, ApiState};
use crate::config::AppConfig;
use crate::feedback;
use anyhow::Result;
use axum::serve;
use metrics_exporter_prometheus::PrometheusBuilder;
use state_store::StateStore;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run(cfg: AppConfig) -> Result<()> {
    let store = StateStore::new();
    let api_state = ApiState {
        store: store.clone(),
    };

    let metrics_addr: SocketAddr = cfg.server.metrics_addr.parse()?;
    PrometheusBuilder::new()
        .with_http_listener(metrics_addr)
        .install()?;

    let feedback_cfg = cfg.feedback_loop.clone();
    let feedback_store = store.clone();
    tokio::spawn(async move {
        feedback::run_reconciler(feedback_store, feedback_cfg).await;
    });

    let app = router(api_state);
    let http_addr: SocketAddr = cfg.server.http_addr.parse()?;
    let listener = TcpListener::bind(http_addr).await?;
    tracing::info!(%http_addr, "nexus-core listening");
    tracing::info!(grpc = %cfg.server.grpc_addr, "gRPC endpoint reserved for proto service");

    serve(listener, app).await?;
    Ok(())
}
