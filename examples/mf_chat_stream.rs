use bullpen::v1::api::Modelfarm;
use bullpen::v1::models::ModelfarmChatModel;
use bullpen::v1::resources::modelfarm::chat_completion::{
    ChatExample, ChatMessage, ChatSession, ModelfarmChatCompletionRequest, ModelfarmChatParameters,
};
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let modelfarm = Modelfarm::new();

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
            content: "How do I write a nix flake for a rust project?".to_string(),
            author: "user".to_string(),
        }],
    };

    let req = ModelfarmChatCompletionRequest {
        model: ModelfarmChatModel::ChatBison,
        parameters: ModelfarmChatParameters {
            prompts: vec![chat_session],
            temperature: 0.2,
            max_output_tokens: 1024,
        },
    };

    let mut stream = modelfarm.stream_chat(req).await.unwrap();

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
