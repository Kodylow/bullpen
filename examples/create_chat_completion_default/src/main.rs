use std::env;

use pplx_client::v1::api::Client;
use pplx_client::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage, Role};

#[tokio::main]
async fn main() {
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-0613".to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: "Hello!".to_string(),
                ..Default::default()
            },
            ChatMessage {
                role: Role::User,
                content: "Where are you located?".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:?}", result);
}
