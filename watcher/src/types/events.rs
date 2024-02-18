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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "event_type")]
pub enum Event {
    SourceSchedule(SourceSchedule),
    StateCreated(StateCreated),
    SourceSignalUpdated(SourceSignalUpdated),
    SinkSignalCreated(SinkSignalCreated),
}

impl From<SourceSchedule> for Event {
    fn from(event: SourceSchedule) -> Self {
        Event::SourceSchedule(event)
    }
}

impl From<StateCreated> for Event {
    fn from(event: StateCreated) -> Self {
        Event::StateCreated(event)
    }
}

impl From<SourceSignalUpdated> for Event {
    fn from(event: SourceSignalUpdated) -> Self {
        Event::SourceSignalUpdated(event)
    }
}

impl From<SinkSignalCreated> for Event {
    fn from(event: SinkSignalCreated) -> Self {
        Event::SinkSignalCreated(event)
    }
}
