use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use goal_engine::GoalEngine;
use orchestrator_api::{Intent, SubmitIntentResponse, SystemState};
use state_store::StateStore;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiState {
    pub store: StateStore,
}

pub fn router(state: ApiState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/intents", post(submit_intent))
        .route("/v1/state/:target", get(get_state))
        .with_state(Arc::new(state))
}

async fn health() -> &'static str {
    "ok"
}

async fn submit_intent(
    State(state): State<Arc<ApiState>>,
    Json(intent): Json<Intent>,
) -> Json<SubmitIntentResponse> {
    let resolved = GoalEngine::sort_goals(GoalEngine::resolve(&intent));
    tracing::info!(intent_id = %resolved.intent_id, goals = resolved.goals.len(), "intent resolved");
    let _ = state;
    Json(SubmitIntentResponse {
        intent_id: resolved.intent_id,
        status: "accepted".into(),
    })
}

async fn get_state(
    State(state): State<Arc<ApiState>>,
    axum::extract::Path(target): axum::extract::Path<String>,
) -> Json<Option<SystemState>> {
    Json(state.store.get(&target).await)
}
