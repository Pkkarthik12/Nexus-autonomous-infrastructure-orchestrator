//! Distributed state: CRDT replica + Raft metadata (simplified in-memory for v0.1).

mod crdt;
mod raft;

pub use crdt::CrdtReplica;
pub use raft::RaftNode;

use orchestrator_api::SystemState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct StateStore {
    inner: Arc<RwLock<HashMap<String, SystemState>>>,
    crdt: Arc<RwLock<CrdtReplica>>,
    raft: Arc<RwLock<RaftNode>>,
}

impl StateStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn upsert(&self, state: SystemState) {
        let mut map = self.inner.write().await;
        map.insert(state.target.clone(), state);
    }

    pub async fn get(&self, target: &str) -> Option<SystemState> {
        self.inner.read().await.get(target).cloned()
    }

    pub async fn merge_crdt(&self, key: &str, value: serde_json::Value) {
        self.crdt.write().await.merge(key, value);
    }

    pub async fn is_leader(&self) -> bool {
        self.raft.read().await.is_leader()
    }
}
