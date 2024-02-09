pub struct Repository {
    table_name: String,
    client: aws_sdk_dynamodb::Client,
}

impl Repository {
    pub fn new(table_name: String, client: aws_sdk_dynamodb::Client) -> Self {
        Self { table_name, client }
    }

    

}
