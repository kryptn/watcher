use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Signal, SignalId, Sink, WatcherItem};

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
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}
impl Into<WatcherItem> for Sent {
    fn into(self) -> WatcherItem {
        WatcherItem::Edge(self.into())
    }
}
#[cfg(test)]
mod test {}
