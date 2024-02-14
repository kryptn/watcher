use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Signal, Source, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Emission {
    #[serde(rename = "PK")]
    pub source_id: String,
    #[serde(rename = "SK")]
    pub signal_id: String,
}

impl From<(&Source, &Signal)> for Emission {
    fn from((endpoint, broadcast): (&Source, &Signal)) -> Self {
        Emission {
            source_id: endpoint.id.clone(),
            signal_id: broadcast.id.clone(),
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
            source_id: "source_id".to_string(),
            signal_id: "signal_id".to_string(),
        };

        let expected = json!({
            "PK": "source_id",
            "SK": "signal_id",
        });

        let serialized = serde_json::to_value(&emission).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_emission_deserialization() {
        let expected = Emission {
            source_id: "source_id".to_string(),
            signal_id: "signal_id".to_string(),
        };

        let deserialized: Emission = serde_json::from_value(json!({
            "PK": "source_id",
            "SK": "signal_id",
        }))
        .unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_emission_from() {
        let source_id = "Source:TestId".to_string();
        let signal_id = "Signal:TestId".to_string();

        let mut endpoint = Faker.fake::<Source>();
        endpoint.id = source_id.clone();
        let mut broadcast = Faker.fake::<Signal>();
        broadcast.id = signal_id.clone();

        let expected = Emission {
            source_id,
            signal_id,
        };

        let emission: Emission = (&endpoint, &broadcast).into();
        assert_eq!(emission, expected);
    }
}
