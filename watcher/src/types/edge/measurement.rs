use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Source, State, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Measurement {
    #[serde(rename = "PK")]
    pub source_id: String,
    #[serde(rename = "SK")]
    pub state_id: String,
}

impl From<(&Source, &State)> for Measurement {
    fn from((endpoint, observation): (&Source, &State)) -> Self {
        Measurement {
            source_id: endpoint.id.clone(),
            state_id: observation.id.clone(),
        }
    }
}

impl Measurement {
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

impl Into<WatcherItem> for Measurement {
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
    fn test_measurement_serialization() {
        let measurement = Measurement {
            source_id: "source_id".to_string(),
            state_id: "state_id".to_string(),
        };

        let expected = json!({
            "PK": "source_id",
            "SK": "state_id",
        });

        let serialized = serde_json::to_value(&measurement).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_measurement_deserialization() {
        let expected = Measurement {
            source_id: "source_id".to_string(),
            state_id: "state_id".to_string(),
        };

        let deserialized: Measurement = serde_json::from_value(json!({
            "PK": "source_id",
            "SK": "state_id",
        }))
        .unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_measurement_from_endpoint_observation() {
        let source_id = "Source:TestId".to_string();
        let state_id = "State:TestId".to_string();

        let mut endpoint = Faker.fake::<Source>();
        endpoint.id = source_id.clone();
        let mut observation = Faker.fake::<State>();
        observation.id = state_id.clone();

        let expected = Measurement {
            source_id,
            state_id,
        };

        let measurement: Measurement = (&endpoint, &observation).into();
        assert_eq!(measurement, expected);
    }
}
