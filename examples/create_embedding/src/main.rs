use std::env;

use pplx_client::v1::api::Client;
use pplx_client::v1::resources::embedding::EmbeddingParameters;

#[tokio::main]
async fn main() {
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = EmbeddingParameters {
        model: "text-embedding-ada-002".to_string(),
        input: "The food was delicious and the waiter...".to_string(),
        user: None,
    };

    let result = client.embeddings().create(parameters).await.unwrap();

    println!("{:?}", result);
}
