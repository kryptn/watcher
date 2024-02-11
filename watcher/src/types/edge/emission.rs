use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Broadcast, Endpoint, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Emission {
    #[serde(rename = "PK")]
    pub endpoint_id: String,
    #[serde(rename = "SK")]
    pub broadcast_id: String,
}

impl From<(&Endpoint, &Broadcast)> for Emission {
    fn from((endpoint, broadcast): (&Endpoint, &Broadcast)) -> Self {
        Emission {
            endpoint_id: endpoint.id.clone(),
            broadcast_id: broadcast.id.clone(),
        }
    }
}

impl Emission {
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

impl Into<WatcherItem> for Emission {
    fn into(self) -> WatcherItem {
        WatcherItem::Edge(self.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    use fake::{Fake, Faker};

    #[test]
    fn test_emission_serialization() {
        let emission = Emission {
            endpoint_id: "endpoint_id".to_string(),
            broadcast_id: "broadcast_id".to_string(),
        };

        let expected = json!({
            "PK": "endpoint_id",
            "SK": "broadcast_id",
        });

        let serialized = serde_json::to_value(&emission).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_emission_deserialization() {
        let expected = Emission {
            endpoint_id: "endpoint_id".to_string(),
            broadcast_id: "broadcast_id".to_string(),
        };

        let deserialized: Emission = serde_json::from_value(json!({
            "PK": "endpoint_id",
            "SK": "broadcast_id",
        }))
        .unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_emission_from() {
        let endpoint_id = "Endpoint:TestId".to_string();
        let broadcast_id = "Broadcast:TestId".to_string();

        let mut endpoint = Faker.fake::<Endpoint>();
        endpoint.id = endpoint_id.clone();
        let mut broadcast = Faker.fake::<Broadcast>();
        broadcast.id = broadcast_id.clone();

        let expected = Emission {
            endpoint_id,
            broadcast_id,
        };

        let emission: Emission = (&endpoint, &broadcast).into();
        assert_eq!(emission, expected);
    }
}
