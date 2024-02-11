use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::WatcherItem;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Subscription {
    #[serde(rename = "PK")]
    pub endpoint_id: String,
    #[serde(rename = "SK")]
    pub sink_id: String,

    // created_at: chrono::DateTime<chrono::Utc>,
    created_at: chrono::DateTime<chrono::Utc>,
    // probably want user data here
}

impl Subscription {
    pub fn new(endpoint_id: String, sink_id: String) -> Self {
        let created_at = chrono::Utc::now();
        Self {
            endpoint_id,
            sink_id,
            created_at,
        }
    }

    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

impl Into<WatcherItem> for Subscription {
    fn into(self) -> WatcherItem {
        WatcherItem::Edge(self.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_subscription_serialization() {
        let now = chrono::Utc::now();

        let subscription = Subscription {
            endpoint_id: "endpoint_id".to_string(),
            sink_id: "sink_id".to_string(),
            created_at: now,
        };

        let expected = json!({
            "PK": "endpoint_id",
            "SK": "sink_id",
            "created_at": now,
        });

        let serialized = serde_json::to_value(&subscription).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_subscription_deserialization() {
        let now = chrono::Utc::now();

        let expected = Subscription {
            endpoint_id: "endpoint_id".to_string(),
            sink_id: "sink_id".to_string(),
            created_at: now,
        };

        let deserialized: Subscription = serde_json::from_value(json!({
            "PK": "endpoint_id",
            "SK": "sink_id",
            "created_at": now,
        }))
        .unwrap();
        assert_eq!(deserialized, expected);
    }
}
