use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Observation {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub sk: String,
    created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    s3_key: Option<String>,
    headers: Vec<(String, String)>,
    status_code: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<u64>,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_observation_serialization() {
        let observation = Observation {
            id: "id".to_string(),
            sk: "sk".to_string(),
            created_at: "created_at".to_string(),
            s3_key: Some("s3_key".to_string()),
            headers: vec![("key".to_string(), "value".to_string())],
            status_code: 200,
            ttl: Some(60),
        };

        let expected = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": "created_at",
            "s3_key": "s3_key",
            "headers": [
                ("key", "value"),
            ],
            "status_code": 200,
            "ttl": 60
        });

        let serialized = serde_json::to_value(&observation).unwrap();
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_observation_deserialization() {
        let json = json!({
            "PK": "id",
            "SK": "sk",
            "created_at": "created_at",
            "s3_key": "s3_key",
            "headers": [
                ("key", "value"),
            ],
            "status_code": 200,
            "ttl": 60
        });

        let expected = Observation {
            id: "id".to_string(),
            sk: "sk".to_string(),
            created_at: "created_at".to_string(),
            s3_key: Some("s3_key".to_string()),
            headers: vec![("key".to_string(), "value".to_string())],
            status_code: 200,
            ttl: Some(60),
        };

        let deserialized: Observation = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
