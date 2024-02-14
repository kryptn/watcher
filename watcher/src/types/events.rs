use serde::{Deserialize, Serialize};

use super::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledState {
    pub source_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceObserved {
    pub source_id: String,
    pub state_id: String,
    pub observation: State,
}
