use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{faker::name::raw::*, locales::*, Dummy, Fake, Faker};

use crate::types::WatcherItem;

use super::Node;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Broadcast {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub sk: String,

    created_at: String,
    contents: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,
}

impl Broadcast {
    #[cfg(test)]
    pub fn mock() -> Self {
        let id = format!("Broadcast:{}", 20.fake::<String>());

        let mut fake: Broadcast = Faker.fake();
        fake.id = id.clone();
        fake.sk = id;
        fake
    }

    pub fn to_watcher_item(self) -> WatcherItem {
        let node = self.into();
        WatcherItem::Node(node)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_broadcast_serialization() {
        let broadcast = Broadcast {
            id: "id".to_string(),
            sk: "sk".to_string(),
            created_at: "created_at".to_string(),
            contents: "contents".to_string(),
            ttl: Some(60),
        };

        let expected = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": "created_at",
            "contents": "contents",
            "ttl": 60
        });

        let serialized = serde_json::to_value(&broadcast).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_broadcast_deserialization() {
        let json = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": "created_at",
            "contents": "contents",
            "ttl": 60
        });

        let expected = Broadcast {
            id: "id".to_string(),
            sk: "sk".to_string(),
            created_at: "created_at".to_string(),
            contents: "contents".to_string(),
            ttl: Some(60),
        };

        let deserialized: Broadcast = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
