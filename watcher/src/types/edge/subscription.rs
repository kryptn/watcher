use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{faker::name::raw::*, locales::*, Dummy, Fake, Faker};

use crate::types::WatcherItem;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(any(test, feature = "fake"), derive(Debug, PartialEq, Dummy))]
pub struct Subscription {
    #[serde(rename = "PK")]
    pub endpoint_id: String,
    #[serde(rename = "SK")]
    pub observation_id: String,

    created_at: String,
    // probably want user data here
}

impl Subscription {
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_subscription_serialization() {
        let subscription = Subscription {
            endpoint_id: "endpoint_id".to_string(),
            observation_id: "observation_id".to_string(),
            created_at: "created_at".to_string(),
        };

        let expected = json!({
            "PK": "endpoint_id",
            "SK": "observation_id",
            "created_at": "created_at",
        });

        let serialized = serde_json::to_value(&subscription).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_subscription_deserialization() {
        let expected = Subscription {
            endpoint_id: "endpoint_id".to_string(),
            observation_id: "observation_id".to_string(),
            created_at: "created_at".to_string(),
        };

        let deserialized: Subscription = serde_json::from_value(json!({
            "PK": "endpoint_id",
            "SK": "observation_id",
            "created_at": "created_at",
        }))
        .unwrap();
        assert_eq!(deserialized, expected);
    }
}
