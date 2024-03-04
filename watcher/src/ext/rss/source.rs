use crate::ext;
use serde::{de::Error, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "rss", tag = "source_type")]
pub struct Config {
    pub url: String,
}

impl Into<reqwest::RequestBuilder> for Config {
    fn into(self) -> reqwest::RequestBuilder {
        reqwest::Client::new().get(&self.url)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "rss", tag = "state_type")]
pub struct Response {
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub status: u16,
}

impl Response {
    async fn from_response(
        response: reqwest::Response,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string()))
            .collect();
        let status = response.status().as_u16();
        let body = response.text().await?;
        Ok(Response {
            headers,
            body,
            status,
        })
    }
}

pub struct Client(reqwest::Client);

impl Client {
    pub fn new() -> Self {
        Client(reqwest::Client::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_serialize() {
        let config = Config {
            url: "https://example.com".to_string(),
        };
        let serialized = serde_json::to_string(&config).unwrap();
        let expected = json!({
            "source_type": "rss",
            "url": "https://example.com",
        });
        assert_eq!(serialized, expected.to_string());
    }
}
