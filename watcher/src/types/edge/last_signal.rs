use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Signal, Source, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastSignal {
    #[serde(rename = "PK")]
    pub source_id: String,
    #[serde(rename = "SK")]
    pub signal_id: String,
}

impl From<(&Source, &Signal)> for LastSignal {
    fn from((endpoint, broadcast): (&Source, &Signal)) -> Self {
        LastSignal {
            source_id: endpoint.id.clone(),
            signal_id: broadcast.id.clone(),
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
