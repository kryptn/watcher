use serde::{Deserialize, Serialize};

#[cfg(any(test, feature = "fake"))]
use fake::{Dummy, Fake, Faker};

use crate::types::{Source, State, WatcherItem};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(any(test, feature = "fake"), derive(PartialEq, Dummy))]
pub struct Witnessed {
    #[serde(rename = "PK")]
    pub source_id: String,
    #[serde(rename = "SK")]
    pub state_id: String,
}

impl From<(&Source, &State)> for Witnessed {
    fn from((source, state): (&Source, &State)) -> Self {
        Witnessed {
            source_id: source.id.clone(),
            state_id: state.id.clone(),
        }
    }
}

impl Witnessed {
    pub fn to_watcher_item(self) -> WatcherItem {
        let edge = self.into();
        WatcherItem::Edge(edge)
    }
}

impl Into<WatcherItem> for Witnessed {
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
    fn test_witnessed_serialization() {
        let emission = Witnessed {
            source_id: "source_id".to_string(),
            state_id: "state_id".to_string(),
        };

        let expected = json!({
            "PK": "source_id",
            "SK": "state_id",
        });

        let serialized = serde_json::to_value(&emission).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_witnessed_deserialization() {
        let expected = Witnessed {
            source_id: "source_id".to_string(),
            state_id: "state_id".to_string(),
        };

        let deserialized: Witnessed = serde_json::from_value(json!({
            "PK": "source_id",
            "SK": "state_id",
        }))
        .unwrap();
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn test_witnessed_from() {
        let source_id = "Source:TestId".to_string();
        let state_id = "State:TestId".to_string();

        let mut source = Faker.fake::<Source>();
        source.id = source_id.clone();
        let mut state = Faker.fake::<State>();
        state.id = state_id.clone();

        let expected = Witnessed {
            source_id,
            state_id,
        };

        let emission: Witnessed = (&source, &state).into();
        assert_eq!(emission, expected);
    }
}
