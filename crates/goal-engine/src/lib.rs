//! Goal engine — intent resolution into prioritized goals.

mod intent;
mod resolver;

pub use intent::{Goal, GoalPriority, ResolvedIntent};
pub use resolver::GoalEngine;
