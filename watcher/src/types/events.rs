use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "command", content = "data")]
pub enum Event {
    SourceObserved {
        source_id: String,
        state_id: String,
    },
    ChangeMeasured {
        source_id: String,
        signal_id: String,
    },
    SignalSent {
        signal_id: String,
        sink_id: String,
    },
}
