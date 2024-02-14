use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::WatcherItem;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct State {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub _sk: String,
    created_at: chrono::DateTime<chrono::Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key: Option<String>,
    headers: Vec<(String, String)>,
    status_code: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,
}

impl State {
    pub fn to_watcher_item(self) -> WatcherItem {
        let node = self.into();
        WatcherItem::Node(node)
    }

    #[cfg(any(test, feature = "fake"))]
    pub fn mock() -> Self {
        let id = format!("State:{}", 20.fake::<String>());

        let mut fake: State = Faker.fake();
        fake.id = id.clone();
        fake._sk = id;
        fake
    }
}

impl Into<WatcherItem> for State {
    fn into(self) -> WatcherItem {
        WatcherItem::Node(self.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_observation_serialization() {
        let now = chrono::Utc::now();

        let observation = State {
            id: "id".to_string(),
            _sk: "sk".to_string(),
            created_at: now,
            s3_key: Some("s3_key".to_string()),
            headers: vec![("key".to_string(), "value".to_string())],
            status_code: 200,
            ttl: Some(60),
        };

        let expected = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": now,
            "s3_key": "s3_key",
            "headers": [
                ("key", "value"),
            ],
            "status_code": 200,
            "ttl": 60
        });

        let serialized = serde_json::to_value(&observation).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_observation_deserialization() {
        let now = chrono::Utc::now();

        let json = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": now,
            "s3_key": "s3_key",
            "headers": [
                ("key", "value"),
            ],
            "status_code": 200,
            "ttl": 60
        });

        let expected = State {
            id: "id".to_string(),
            _sk: "sk".to_string(),
            created_at: now,
            s3_key: Some("s3_key".to_string()),
            headers: vec![("key".to_string(), "value".to_string())],
            status_code: 200,
            ttl: Some(60),
        };

        let deserialized: State = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
