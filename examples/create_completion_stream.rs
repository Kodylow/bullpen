use std::env;

use dotenv::dotenv;
use futures_util::stream::StreamExt;
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
        prompt: "Hello! How are you?".to_string(),
        temperature: None,
        top_p: None,
        top_k: None,
        max_tokens: Some(100),
        presence_penalty: None,
        frequency_penalty: None,
    };

    let mut stream = client
        .completions()
        .create_stream(parameters)
        .await
        .unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                print!("{}", choice.text);
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
