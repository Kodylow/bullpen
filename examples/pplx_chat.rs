use std::env;

use bullpen::api::Pplx;
use bullpen::models::PplxChatModel;
use bullpen::resources::pplx::chat_completion::{ChatMessage, PplxChatCompletionRequest, Role};
use dotenv::dotenv;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let pplx = Pplx::new(api_key);

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
                content: "Where are you located?".to_string(),
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    let result = pplx.chat(parameters).await.unwrap();

    println!("{:?}", result);
}
