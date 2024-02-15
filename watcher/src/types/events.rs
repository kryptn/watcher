use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSchedule {
    pub source_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateCreated {
    pub source_id: String,
    pub state_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceSignalUpdated {
    pub source_id: String,
    pub signal_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkSignalCreated {
    pub sink_id: String,
    pub signal_id: String,
}
