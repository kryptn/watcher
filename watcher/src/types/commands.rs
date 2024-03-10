use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "command", content = "data")]
enum Command {
    // meant to hit the remote source
    ObserveSource { source_id: String },

    SendSignal { signal_id: String, sink_id: String },
}
