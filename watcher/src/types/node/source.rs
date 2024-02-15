use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::WatcherItem;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Source {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub _sk: String,

    pub name: String,
    // endpoint_type: SourceType,
    // endpoint_data: Value,
    #[serde(flatten)]
    pub endpoint: SourceType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<String>,
}

impl Source {
    pub fn new(
        id: String,
        name: String,
        endpoint: SourceType,
        rate: Option<String>,
        schedule_name: Option<String>,
    ) -> Self {
        Self {
            id: id.clone(),
            _sk: id,
            name,
            endpoint,
            rate,
            schedule_name,
        }
    }

    pub fn to_watcher_item(self) -> WatcherItem {
        let node = self.into();
        WatcherItem::Node(node)
    }

    #[cfg(any(test, feature = "fake"))]
    pub fn mock() -> Self {
        let id = format!("Source:{}", 20.fake::<String>());

        let mut fake: Source = Faker.fake();
        fake.id = id.clone();
        fake._sk = id;
        fake
    }
}

impl Into<WatcherItem> for Source {
    fn into(self) -> WatcherItem {
        WatcherItem::Node(self.into())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
#[serde(rename_all = "snake_case")]
pub struct Rss {
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
#[serde(rename_all = "snake_case")]
pub struct Http {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
#[serde(
    rename_all = "snake_case",
    tag = "endpoint_type",
    content = "endpoint_data"
)]
pub enum SourceType {
    Rss(Rss),
    Http(Http),
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_source_serialization() {
        let endpoint = Source {
            id: "id".to_string(),
            _sk: "sk".to_string(),
            name: "name".to_string(),
            endpoint: SourceType::Rss(Rss {
                url: "url".to_string(),
            }),
            rate: Some("rate".to_string()),
            schedule_name: Some("schedule_name".to_string()),
        };

        let expected = json!({
            "PK": "id",
            "SK": "sk",
            "name": "name",
            "endpoint_type": "rss",
            "endpoint_data": {
                "url": "url"
            },
            "rate": "rate",
            "schedule_name": "schedule_name"
        });

        let serialized = serde_json::to_value(&endpoint).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_source_deserialization() {
        let json = json!({
            "PK": "id",
            "SK": "sk",
            "name": "name",
            "endpoint_type": "rss",
            "endpoint_data": {
                "url": "url"
            },
            "rate": "rate",
            "schedule_name": "schedule_name"
        });

        let expected = Source {
            id: "id".to_string(),
            _sk: "sk".to_string(),
            name: "name".to_string(),
            endpoint: SourceType::Rss(Rss {
                url: "url".to_string(),
            }),
            rate: Some("rate".to_string()),
            schedule_name: Some("schedule_name".to_string()),
        };

        let deserialized = serde_json::from_value::<Source>(json).unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_source_fake() {
        let endpoint = Source::mock();
        println!("{:#?}", endpoint);
        assert_eq!(endpoint.id, endpoint._sk);
    }
}
