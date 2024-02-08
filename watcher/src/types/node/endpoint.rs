use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Endpoint {
    #[serde(rename = "PK")]
    pub id: String,
    #[serde(rename = "SK")]
    pub sk: String,

    pub name: String,
    // endpoint_type: EndpointType,
    // endpoint_data: Value,
    #[serde(flatten)]
    pub endpoint: EndpointType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(rename_all = "snake_case")]
pub struct Rss {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(rename_all = "snake_case")]
pub struct Http {
    pub url: String,
    pub method: String,
    pub headers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
#[serde(
    rename_all = "snake_case",
    tag = "endpoint_type",
    content = "endpoint_data"
)]
pub enum EndpointType {
    Rss(Rss),
    Http(Http),
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_endpoint_serialization() {
        let endpoint = Endpoint {
            id: "id".to_string(),
            sk: "sk".to_string(),
            name: "name".to_string(),
            endpoint: EndpointType::Rss(Rss {
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
    fn test_endpoint_deserialization() {
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

        let expected = Endpoint {
            id: "id".to_string(),
            sk: "sk".to_string(),
            name: "name".to_string(),
            endpoint: EndpointType::Rss(Rss {
                url: "url".to_string(),
            }),
            rate: Some("rate".to_string()),
            schedule_name: Some("schedule_name".to_string()),
        };

        let deserialized = serde_json::from_value::<Endpoint>(json).unwrap();
        assert_eq!(deserialized, expected);
    }
}
