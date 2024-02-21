use serde::{Deserialize, Serialize};

use crate::types::{Signal, Sink, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Send {
    #[serde(rename = "PK")]
    pub sink_id: String,
    #[serde(rename = "SK")]
    pub signal_id: String,
}

impl From<(&Signal, &Sink)> for Sent {
    fn from((broadcast, sink): (&Signal, &Sink)) -> Self {
        Sent {
            signal_id: broadcast.id.clone(),
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
