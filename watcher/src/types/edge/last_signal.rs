use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Signal, SignalId, Source, WatcherItem};

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
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

impl Into<WatcherItem> for LastSignal {
    fn into(self) -> WatcherItem {
        WatcherItem::Edge(self.into())
    }
}

#[cfg(test)]
mod test {}
