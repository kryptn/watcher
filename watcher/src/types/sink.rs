use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Sink {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub sk: String,

    created_at: String,

    #[serde(flatten)]
    sink: SinkType,

    #[serde(skip_serializing_if = "Option::is_none")]
    last_status_code: Option<u8>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(rename_all = "snake_case")]
pub struct Discord {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
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
        let sink = Sink {
            id: "id".to_string(),
            sk: "sk".to_string(),
            created_at: "created_at".to_string(),
            sink: SinkType::Discord(Discord {
                url: "url".to_string(),
            }),
            last_status_code: Some(200),
        };

        let expected = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": "created_at",
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
        let json = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": "created_at",
            "sink_type": "discord",
            "sink_data": {
                "url": "url"
            },
            "last_status_code": 200
        });

        let expected = Sink {
            id: "id".to_string(),
            sk: "sk".to_string(),
            created_at: "created_at".to_string(),
            sink: SinkType::Discord(Discord {
                url: "url".to_string(),
            }),
            last_status_code: Some(200),
        };

        let deserialized: Sink = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
