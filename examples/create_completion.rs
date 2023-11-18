use std::env;

use dotenv::dotenv;
use pplx_client::v1::api::Client;
use pplx_client::v1::models::PplxCompletionModel;
use pplx_client::v1::resources::completion::CompletionParameters;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = CompletionParameters {
        model: PplxCompletionModel::ReplitCodeV15_3b,
        prompt: "The proper way to structure a rust and nix project is ".to_string(),
        temperature: None,
        top_p: None,
        top_k: None,
        max_tokens: Some(500),
        presence_penalty: None,
        frequency_penalty: None,
    };

    let result = client.completions().create(parameters).await.unwrap();

    println!("{:?}", result);
}
