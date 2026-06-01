use serde_json::Value;
use std::collections::HashMap;

/// Last-write-wins register map (simplified CRDT for v0.1).
#[derive(Debug, Default)]
pub struct CrdtReplica {
    registers: HashMap<String, (u64, Value)>,
    version: u64,
}

impl CrdtReplica {
    pub fn merge(&mut self, key: &str, value: Value) {
        self.version += 1;
        self.registers.insert(key.to_string(), (self.version, value));
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.registers.get(key).map(|(_, v)| v)
    }

    pub fn snapshot(&self) -> HashMap<String, Value> {
        self.registers
            .iter()
            .map(|(k, (_, v))| (k.clone(), v.clone()))
            .collect()
    }
}
