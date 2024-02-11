use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::WatcherItem;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Sink {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub _sk: String,

    created_at: chrono::DateTime<chrono::Utc>,

    #[serde(flatten)]
    sink: SinkType,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_status_code: Option<u8>,
}

impl Sink {
    pub fn to_watcher_item(self) -> WatcherItem {
        let node = self.into();
        WatcherItem::Node(node)
    }

    #[cfg(any(test, feature = "fake"))]
    pub fn mock() -> Self {
        let id = format!("Sink:{}", 20.fake::<String>());

        let mut fake: Self = Faker.fake();
        fake.id = id.clone();
        fake._sk = id;
        fake
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
#[serde(rename_all = "snake_case")]
pub struct Discord {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
#[serde(rename_all = "snake_case", tag = "sink_type", content = "sink_data")]
pub enum SinkType {
    Discord(Discord),
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_sink_serialization() {
        let now = chrono::Utc::now();

        let sink = Sink {
            id: "id".to_string(),
            _sk: "sk".to_string(),
            created_at: now,
            sink: SinkType::Discord(Discord {
                url: "url".to_string(),
            }),
            last_status_code: Some(200),
        };

        let expected = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": now,
            "sink_type": "discord",
            "sink_data": {
                "url": "url"
            },
            "last_status_code": 200
        });

        let serialized = serde_json::to_value(&sink).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_sink_deserialization() {
        let now = chrono::Utc::now();

        let json = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": now,
            "sink_type": "discord",
            "sink_data": {
                "url": "url"
            },
            "last_status_code": 200
        });

        let expected = Sink {
            id: "id".to_string(),
            _sk: "sk".to_string(),
            created_at: now,
            sink: SinkType::Discord(Discord {
                url: "url".to_string(),
            }),
            last_status_code: Some(200),
        };

        let deserialized: Sink = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
