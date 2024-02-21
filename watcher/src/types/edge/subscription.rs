use serde::{Deserialize, Serialize};

use crate::types::Item;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subscription {
    #[serde(rename = "PK")]
    pub source_id: String,
    #[serde(rename = "SK")]
    pub sink_id: String,

    // created_at: chrono::DateTime<chrono::Utc>,
    created_at: chrono::DateTime<chrono::Utc>,
    // probably want user data here
}

impl Subscription {
    pub fn new(source_id: String, sink_id: String) -> Self {
        let created_at = chrono::Utc::now();
        Self {
            source_id,
            sink_id,
            created_at,
        }
    }

    pub fn to_watcher_item(self) -> Item {
        let edge = self.into();
        Item::Subscription(edge)
    }
}

impl Into<Item> for Subscription {
    fn into(self) -> Item {
        Item::Subscription(self.into())
    }
}

#[cfg(test)]
mod test {}
