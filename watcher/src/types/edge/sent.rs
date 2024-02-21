use serde::{Deserialize, Serialize};

use crate::types::{Item, Signal, SignalId, Sink};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sent {
    #[serde(rename = "PK")]
    pub sink_id: String,
    #[serde(rename = "SK")]
    pub signal_id: SignalId,
}

impl From<(&Signal, &Sink)> for Sent {
    fn from((signal, sink): (&Signal, &Sink)) -> Self {
        Sent {
            signal_id: signal.id.clone(),
            sink_id: sink.id.clone(),
        }
    }
}

impl Sent {
    pub fn to_watcher_item(self) -> Item {
        let edge = self.into();
        Item::Sent(edge)
    }
}
impl Into<Item> for Sent {
    fn into(self) -> Item {
        Item::Sent(self.into())
    }
}
#[cfg(test)]
mod test {}
