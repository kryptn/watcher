use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledObservation {
    pub endpoint_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointBroadcast {
    pub endpoint_id: String,
    pub observation_id: String,
    pub sink_id: String,
}
