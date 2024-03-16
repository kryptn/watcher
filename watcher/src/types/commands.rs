use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "command", content = "data")]
pub enum Command {
    ObserveSource {
        source_id: String,
    },
    SendSignal {
        signal_id: String,
        sink_id: String,
    },

    Subscribe {
        source_id: String,
        sink_id: String,
    },
    Unsubscribe {
        source_id: String,
        sink_id: String,
    },

    AddSource {
        name: String,
        config: serde_json::Value,
    },
    DeleteSource {
        source_id: String,
    },
    AddSink {
        name: String,
        config: serde_json::Value,
    },
    DeleteSink {
        sink_id: String,
    },
}
