use std::env;

use bullpen::api::Pplx;
use bullpen::models::PplxChatModel;
use bullpen::resources::pplx::chat_completion::{ChatMessage, PplxChatCompletionRequest, Role};
use dotenv::dotenv;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Pplx::new(api_key);

    let parameters = PplxChatCompletionRequest {
        model: PplxChatModel::Mixtral8x7bInstruct,
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: "Hello!".to_string(),
                ..Default::default()
            },
            ChatMessage {
                role: Role::User,
                content: "Tell me a story? An extremely long and detailed one?".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let mut stream = client.stream_chat(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                if let Some(content) = choice.delta.content.as_ref() {
                    print!("{}", content);
                }
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
