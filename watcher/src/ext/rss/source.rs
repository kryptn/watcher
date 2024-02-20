use crate::ext::Source;
use serde::{Deserialize, Serialize};

pub struct Config {
    pub url: String,
}

impl Into<reqwest::RequestBuilder> for Config {
    fn into(self) -> reqwest::RequestBuilder {
        reqwest::Client::new().get(&self.url)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub status: u16,
}

pub struct SourceClient(reqwest::Client);

impl Source for SourceClient {
    type Metadata = ResponseMetadata;
    type Output = serde_json::Value;

    async fn fetch<T>(
        &self,
        config: impl Into<reqwest::RequestBuilder>,
    ) -> Result<(Self::Output, Self::Metadata), Box<dyn std::error::Error>> {
        let req: reqwest::RequestBuilder = config.into();
        let response = req.send().await?;

        let headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.as_str().to_string(), v.to_str().unwrap().to_string()))
            .collect();
        let status = response.status().as_u16();
        let body = response.text().await?;
        let metadata = ResponseMetadata {
            headers,
            body: body.clone(),
            status,
        };

        let out = body.into();
        Ok((out, metadata))
    }
}
