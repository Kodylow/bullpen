use std::env;

use dotenv::dotenv;
use pplx_client::v1::api::Client;
use pplx_client::v1::models::PplxChatModel;
use pplx_client::v1::resources::pplx::chat_completion::{
    ChatMessage, PplxChatCompletionParameters, Role,
};
#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = PplxChatCompletionParameters {
        model: PplxChatModel::Mistral7bInstruct,
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
        temperature: None,
        top_p: None,
        top_k: None,
        max_tokens: Some(12),
        presence_penalty: None,
        frequency_penalty: None,
    };

    let result = client.chat().pplx_create(parameters).await.unwrap();

    println!("{:?}", result);
}
