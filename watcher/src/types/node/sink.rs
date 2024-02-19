use serde::{Deserialize, Serialize};

use crate::{sink::discord, types::WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sink {
    #[serde(rename = "PK")]
    pub id: String,

    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(flatten)]
    pub sink: SinkType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_code: Option<u8>,
}

impl Sink {
    pub fn to_watcher_item(self) -> WatcherItem {
        let node = self.into();
        WatcherItem::Node(node)
    }
}

impl Into<WatcherItem> for Sink {
    fn into(self) -> WatcherItem {
        WatcherItem::Node(self.into())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "sink_type", content = "sink_data")]
pub enum SinkType {
    Discord(discord::Config),
}

#[cfg(test)]
mod test {}
