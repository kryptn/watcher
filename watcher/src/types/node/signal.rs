use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::WatcherItem;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signal {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub sk: String,

    pub created_at: chrono::DateTime<chrono::Utc>,

    // this should really be serde_json::Value
    // i haven't tried to handle that with faker yet
    pub contents: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,
}

impl Signal {
    pub fn to_watcher_item(self) -> WatcherItem {
        let node = self.into();
        WatcherItem::Node(node)
    }
}

impl Into<WatcherItem> for Signal {
    fn into(self) -> WatcherItem {
        WatcherItem::Node(self.into())
    }
}

#[cfg(test)]
mod test {}
