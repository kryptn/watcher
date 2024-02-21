use serde::{Deserialize, Serialize};

use crate::{ext::discord, types::Item};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Sink {
    #[serde(rename = "PK")]
    pub id: String,

    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(flatten)]
    pub sink: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_code: Option<u8>,
}

impl Sink {
    pub fn to_watcher_item(self) -> Item {
        let node = self.into();
        Item::Sink(node)
    }
}

impl Into<Item> for Sink {
    fn into(self) -> Item {
        Item::Sink(self.into())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case", tag = "sink_type", content = "sink_data")]
pub enum SinkType {
    Discord(discord::Config),
}

#[cfg(test)]
mod test {}
