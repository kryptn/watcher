use serde::{Deserialize, Serialize};

use crate::types::{Item, Signal, SignalId, Source};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastSignal {
    #[serde(rename = "PK")]
    pub source_id: String,
    #[serde(rename = "SK")]
    pub signal_id: SignalId,
}

impl From<(&Source, &Signal)> for LastSignal {
    fn from((endpoint, signal): (&Source, &Signal)) -> Self {
        LastSignal {
            source_id: endpoint.id.clone(),
            signal_id: signal.id.clone(),
        }
    }
}

impl LastSignal {
    pub fn to_watcher_item(self) -> Item {
        let edge = self.into();
        Item::LastSignal(edge)
    }
}

impl Into<Item> for LastSignal {
    fn into(self) -> Item {
        Item::LastSignal(self.into())
    }
}

#[cfg(test)]
mod test {}
