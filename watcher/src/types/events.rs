use serde::{Deserialize, Serialize};

use super::Observation;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledObservation {
    pub endpoint_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointObserved {
    pub endpoint_id: String,
    pub observation_id: String,
    pub observation: Observation,
}

