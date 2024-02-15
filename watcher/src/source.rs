use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use reqwest::Response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct SourceResult<R, T>
where
    R: Serialize + DeserializeOwned,
    T: Serialize + DeserializeOwned,
{
    pub response: R,
    pub items: Vec<T>,
}
#[allow(async_fn_in_trait)]
pub trait Source<'a, T>
where
    T: Serialize + Deserialize<'a> + Clone,
{
    type Item: Ord + PartialOrd + Eq + PartialEq + Serialize + Deserialize<'a>;
    type Metadata: Serialize + Deserialize<'a>;

    async fn fetch(&self, metadata: &Self::Metadata) -> Result<T, Box<dyn std::error::Error>>;

    async fn store(&self, key: &str, body: T) -> Result<(), Box<dyn std::error::Error>>;

    async fn witness(&self, metadata: &Self::Metadata) -> Result<T, Box<dyn std::error::Error>> {
        let response = self.fetch(metadata).await?;
        let source_id = "";
        let now = chrono::Utc::now();
        let key = format!("source-{}/state/{}", source_id, now);
        self.store(&key, response.clone()).await?;
        Ok(response)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RssMetadata {
    url: String,
}

pub struct Rss {
    client: reqwest::Client,
    s3: aws_sdk_s3::Client,
}

pub struct FeedItem {}

impl<'a, T> Source<'a, T> for Rss
where
    T: Serialize + Deserialize<'a> + Clone,
    SdkBody: From<T>,
{
    type Item = String;
    type Metadata = RssMetadata;

    async fn fetch(&self, metadata: &RssMetadata) -> Result<T, Box<dyn std::error::Error>> {
        let response = self.client.get(&metadata.url).send().await?;
        let body = response.text().await?;
        Ok(T::from(body))
    }

    async fn store(&self, key: &str, body: T) -> Result<(), Box<dyn std::error::Error>>
    where
        T:,
    {
        let stream = ByteStream::new(SdkBody::from(body));
        self.s3
            .put_object()
            .bucket("bucket")
            .key(key)
            .body(stream)
            .send()
            .await?;
        Ok(())
    }
}
