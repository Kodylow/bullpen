use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionStreamResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CompletionStreamChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionStreamChoice {
    pub text: String,
    pub index: u32,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
