use serde::{Deserialize, Serialize};

use super::Observation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledObservation {
    pub source_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceObserved {
    pub source_id: String,
    pub observation_id: String,
    pub observation: Observation,
}
