use std::env;

use dotenv::dotenv;
use futures_util::stream::StreamExt;
use pplx_client::v1::api::Pplx;
use pplx_client::v1::models::PplxChatModel;
use pplx_client::v1::resources::pplx::chat_completion::{
    ChatMessage, PplxChatCompletionParameters, Role,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Pplx::new(api_key);

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
                content: "Tell me a story?".to_string(),
                ..Default::default()
            },
        ],
        temperature: None,
        top_p: None,
        top_k: None,
        max_tokens: Some(100),
        presence_penalty: None,
        frequency_penalty: None,
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
