use chrono::prelude::*;
use chrono::serde::ts_microseconds;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::to_value;

use crate::types::Item;

#[derive(Clone, Debug)]
pub struct SignalId {
    state_id: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    latest: Option<bool>,
}

impl Serialize for SignalId {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(true) = self.latest {
            let id = format!("Signal:{}:{}", self.state_id, "latest");
            return s.serialize_str(&id);
        }

        let ts = to_value(self.created_at).unwrap();
        let ts = ts.as_str().unwrap();

        let id = format!("Signal:{}:{}", self.state_id, ts);
        s.serialize_str(&id)
    }
}

impl<'de> Deserialize<'de> for SignalId {
    fn deserialize<D>(deserializer: D) -> Result<SignalId, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        let parts: Vec<&str> = id.splitn(4, ':').collect();
        let state_id = format!("State:{}", parts[2].to_string());

        let part_3 = parts[3].to_string();

        if part_3 == "latest" {
            return Ok(SignalId {
                state_id,
                created_at: None,
                latest: Some(true),
            });
        }

        let created_at = Some(part_3.parse().unwrap());

        Ok(SignalId {
            state_id,
            created_at,
            latest: None,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Signal {
    #[serde(rename = "PK")]
    pub id: SignalId,

    pub created_at: chrono::DateTime<chrono::Utc>,

    // this should really be serde_json::Value
    // i haven't tried to handle that with faker yet
    pub contents: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,
}

impl Into<Item> for Signal {
    fn into(self) -> Item {
        Item::Signal(self)
    }
}

#[cfg(test)]
mod test {

    use serde_json::json;

    use super::*;

    #[test]
    fn test_signal_serialize() {
        let signal = Signal {
            id: SignalId {
                state_id: "state_id".to_string(),
                created_at: Some(chrono::Utc::now()),
                latest: None,
            },
            created_at: chrono::Utc::now(),
            contents: "contents".to_string(),
            ttl: Some(60),
        };

        let serialized = serde_json::to_string(&signal).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn test_signal_deserialize() {
        let serialized = json!({
            "PK": "Signal:State:state_id:2024-02-19T07:34:20.987349Z",
            "SK": "Signal:State:state_id:2024-02-19T07:34:20.987349Z",
            "created_at": "2024-02-19T07:34:20.987452Z",
            "contents": "contents",
            "ttl": 60,
            "latest": true
        })
        .to_string();

        let mut d = serde_json::Deserializer::from_str(&serialized);
        let result: Result<Signal, _> = serde_path_to_error::deserialize(&mut d);
        // let out = serde_path_to_error::deserialize(d);

        println!("{:?}", result);
    }

    #[test]
    fn test_signal_id_serialize() {
        let signal_id = SignalId {
            state_id: "state_id".to_string(),
            created_at: Some(chrono::Utc::now()),
            latest: Some(true),
        };

        let serialized = serde_json::to_string(&signal_id).unwrap();
        println!("{}", serialized);
    }
}
