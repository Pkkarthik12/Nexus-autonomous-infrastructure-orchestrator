/// Raft node stub — single-node leader for local/dev; extend for HA clusters.
#[derive(Debug)]
pub struct RaftNode {
    term: u64,
    leader: bool,
}

impl Default for RaftNode {
    fn default() -> Self {
        Self {
            term: 1,
            leader: true,
        }
    }
}

impl RaftNode {
    pub fn is_leader(&self) -> bool {
        self.leader
    }

    pub fn term(&self) -> u64 {
        self.term
    }
}
