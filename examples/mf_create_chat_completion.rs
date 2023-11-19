use std::env;

use dotenv::dotenv;
use futures_util::stream::StreamExt;
use pplx_client::v1::api::Client;
use pplx_client::v1::models::ModelfarmChatModel;
use pplx_client::v1::resources::modelfarm::chat_completion::{
    ChatExample, ChatMessage, ChatSession, ModelfarmChatCompletionParameters,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Client::new(api_key);

    let chat_session = ChatSession {
        context: "You are a programmer bot".to_string(),
        examples: vec![ChatExample {
            input: ChatMessage {
                content: "1 + 1".to_string(),
                author: "user".to_string(),
            },
            output: ChatMessage {
                content: "2".to_string(),
                author: "assistant".to_string(),
            },
        }],
        messages: vec![ChatMessage {
            content: "How do I write a nix flake for a rust
    project?"
                .to_string(),
            author: "user".to_string(),
        }],
    };

    let parameters = ModelfarmChatCompletionParameters {
        model: ModelfarmChatModel::TextBison,
        parameters: chat_session.model_dump(),
    };

    let mut stream = client.chat().mf_create_stream(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.responses.iter().for_each(|response| {
                response.candidates.iter().for_each(|candidate| {
                    print!("{}", candidate.message.content);
                });
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
