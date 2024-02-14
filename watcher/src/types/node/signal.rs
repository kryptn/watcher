use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::WatcherItem;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Signal {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub sk: String,

    created_at: chrono::DateTime<chrono::Utc>,
    contents: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,
}

impl Signal {
    #[cfg(any(test, feature = "fake"))]
    pub fn mock() -> Self {
        let id = format!("Signal:{}", 20.fake::<String>());

        let mut fake: Signal = Faker.fake();
        fake.id = id.clone();
        fake.sk = id;
        fake
    }

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
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_broadcast_serialization() {
        let now = chrono::Utc::now();

        let broadcast = Signal {
            id: "id".to_string(),
            sk: "sk".to_string(),
            created_at: now,
            contents: "contents".to_string(),
            ttl: Some(60),
        };

        let expected = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": now,
            "contents": "contents",
            "ttl": 60
        });

        let serialized = serde_json::to_value(&broadcast).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_broadcast_deserialization() {
        let now = chrono::Utc::now();

        let json = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": now,
            "contents": "contents",
            "ttl": 60
        });

        let expected = Signal {
            id: "id".to_string(),
            sk: "sk".to_string(),
            created_at: now,
            contents: "contents".to_string(),
            ttl: Some(60),
        };

        let deserialized: Signal = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
