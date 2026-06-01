use crate::config::FeedbackConfig;
use chrono::Utc;
use orchestrator_api::{FeedbackEvent, SystemState};
use state_store::StateStore;
use std::time::Duration;
use tokio::time::interval;

pub async fn run_reconciler(store: StateStore, cfg: FeedbackConfig) {
    if !cfg.enabled {
        tracing::info!("feedback loop disabled");
        return;
    }
    let mut tick = interval(Duration::from_secs(cfg.reconcile_interval_secs));
    loop {
        tick.tick().await;
        if !store.is_leader().await {
            continue;
        }
        // Demo: synthesize drift feedback for known targets
        for target in ["production", "staging", "edge"] {
            let drift = 0.02_f64;
            let event = FeedbackEvent {
                target: target.into(),
                observed_hash: format!("obs-{target}"),
                drift,
            };
            if drift > cfg.drift_threshold {
                tracing::warn!(?event, "drift exceeds threshold");
            }
            let state = SystemState {
                target: target.into(),
                observed_hash: event.observed_hash.clone(),
                desired_hash: format!("des-{target}"),
                drift: event.drift,
                updated_at: Utc::now(),
            };
            store.upsert(state).await;
        }
    }
}
