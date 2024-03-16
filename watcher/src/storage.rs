use aws_sdk_s3::primitives::{ByteStream, SdkBody};

pub trait Storable {
    fn manifest(&self) -> Vec<(String, impl Into<SdkBody>)>;
}

pub struct Client {
    inner: aws_sdk_s3::Client,
    bucket: String,
}

impl Client {
    pub async fn new(bucket: String) -> Self {
        let config = aws_config::load_from_env().await;

        let inner = aws_sdk_s3::Client::new(&config);
        Client { inner, bucket }
    }

    pub async fn store<S: Storable>(&self, s: S) -> Result<(), Box<dyn std::error::Error>> {
        let manifest = s.manifest();
        for (key, body) in manifest {
            let body = ByteStream::new(body.into());

            self.inner
                .put_object()
                .bucket(&self.bucket)
                .key(&key)
                .body(body)
                .send()
                .await?;
        }

        Ok(())
    }
}
