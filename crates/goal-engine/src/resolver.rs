use crate::intent::ResolvedIntent;
use orchestrator_api::Intent;

/// Translates declarative intents into actionable goals.
pub struct GoalEngine;

impl GoalEngine {
    pub fn resolve(intent: &Intent) -> ResolvedIntent {
        ResolvedIntent::from_intent(intent)
    }

    pub fn sort_goals(mut resolved: ResolvedIntent) -> ResolvedIntent {
        resolved
            .goals
            .sort_by(|a, b| b.priority.cmp(&a.priority));
        resolved
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn critical_slo_intent() {
        let intent = Intent {
            goal: "maintain_slo".into(),
            target: "production/api".into(),
            parameters: HashMap::new(),
            slo: None,
        };
        let resolved = GoalEngine::resolve(&intent);
        assert_eq!(resolved.goals[0].priority, crate::intent::GoalPriority::Critical);
    }
}
