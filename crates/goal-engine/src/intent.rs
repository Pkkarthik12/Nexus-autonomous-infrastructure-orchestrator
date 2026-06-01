use orchestrator_api::Intent;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GoalPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

#[derive(Debug, Clone)]
pub struct Goal {
    pub id: Uuid,
    pub name: String,
    pub target: String,
    pub priority: GoalPriority,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedIntent {
    pub intent_id: Uuid,
    pub goals: Vec<Goal>,
}

impl ResolvedIntent {
    pub fn from_intent(intent: &Intent) -> Self {
        let intent_id = Uuid::new_v4();
        let priority = match intent.goal.as_str() {
            "maintain_slo" | "remediate_outage" => GoalPriority::Critical,
            "scale_to_capacity" => GoalPriority::High,
            _ => GoalPriority::Normal,
        };
        let goals = vec![Goal {
            id: Uuid::new_v4(),
            name: intent.goal.clone(),
            target: intent.target.clone(),
            priority,
            constraints: intent
                .parameters
                .iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect(),
        }];
        Self { intent_id, goals }
    }
}
