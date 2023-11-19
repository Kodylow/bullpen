use std::env;

use bullpen::v1::api::Pplx;
use bullpen::v1::models::PplxChatModel;
use bullpen::v1::resources::pplx::chat_completion::{ChatMessage, PplxChatCompletionRequest, Role};
use dotenv::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let pplx = Pplx::new(api_key);

    let parameters = PplxChatCompletionRequest {
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

    let result = pplx.chat(parameters).await.unwrap();

    println!("{:?}", result);
}
