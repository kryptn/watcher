use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::ext;
use crate::types::Item;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Source {
    #[serde(rename = "PK")]
    pub id: String,

    pub name: String,
    // endpoint_type: SourceType,
    // endpoint_data: Value,
    #[serde(flatten)]
    // pub endpoint: serde_json::Value,
    pub endpoint: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<String>,
}

impl Source {
    pub fn to_watcher_item(self) -> Item {
        let node = self.into();
        Item::Source(node)
    }
}

impl Into<Item> for Source {
    fn into(self) -> Item {
        Item::Source(self.into())
    }
}
