use bullpen::api::Modelfarm;
use bullpen::models::ModelfarmChatModel;
use bullpen::resources::modelfarm::chat::{
    ChatExample, ChatMessage, ChatSession, ModelfarmChatCompletionRequest, ModelfarmChatParameters,
};

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

    let req =
        ModelfarmChatCompletionRequest {
            model: ModelfarmChatModel::ChatBison,
            parameters: ModelfarmChatParameters {
                prompts: vec![chat_session],
                temperature: 0.2,
                max_output_tokens: 1024,
            },
        };

    let result = modelfarm.chat(req).await.unwrap();

    println!("{:?}", result);
}
