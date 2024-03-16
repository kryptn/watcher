use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "TABLE_NAME")]
    pub table_name: Option<String>,

    #[envconfig(from = "ENDPOINT_URL")]
    pub endpoint: Option<String>,

    #[envconfig(from = "SQS_QUEUE_URL")]
    pub sqs_queue_url: Option<String>,

    #[envconfig(from = "BUCKET")]
    pub bucket: Option<String>,
}

pub fn init() -> Config {
    Config::init_from_env().unwrap()
}
