use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{faker::name::raw::*, locales::*, Dummy, Fake, Faker};

use crate::types::{Endpoint, Observation, WatcherItem};

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(any(test, feature = "fake"), derive(Debug, PartialEq, Dummy))]
pub struct Measurement {
    #[serde(rename = "PK")]
    pub endpoint_id: String,
    #[serde(rename = "SK")]
    pub observation_id: String,
}

impl From<(&Endpoint, &Observation)> for Measurement {
    fn from((endpoint, observation): (&Endpoint, &Observation)) -> Self {
        Measurement {
            endpoint_id: endpoint.id.clone(),
            observation_id: observation.id.clone(),
        }
    }
}

impl Measurement {
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
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
            endpoint_id: "endpoint_id".to_string(),
            observation_id: "observation_id".to_string(),
        };

        let expected = json!({
            "PK": "endpoint_id",
            "SK": "observation_id",
        });

        let serialized = serde_json::to_value(&measurement).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_measurement_deserialization() {
        let expected = Measurement {
            endpoint_id: "endpoint_id".to_string(),
            observation_id: "observation_id".to_string(),
        };

        let deserialized: Measurement = serde_json::from_value(json!({
            "PK": "endpoint_id",
            "SK": "observation_id",
        }))
        .unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_measurement_from_endpoint_observation() {
        let endpoint_id = "Endpoint:TestId".to_string();
        let observation_id = "Observation:TestId".to_string();

        let mut endpoint = Faker.fake::<Endpoint>();
        endpoint.id = endpoint_id.clone();
        let mut observation = Faker.fake::<Observation>();
        observation.id = observation_id.clone();

        let expected = Measurement {
            endpoint_id,
            observation_id,
        };

        let measurement: Measurement = (&endpoint, &observation).into();
        assert_eq!(measurement, expected);
    }
}
