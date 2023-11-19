use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Metadata;
use crate::v1::models::ModelfarmChatModel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatExample {
    pub input: ChatMessage,
    pub output: ChatMessage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatSession {
    pub context: String,
    pub examples: Vec<ChatExample>,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameters {
    prompts: Vec<ChatSession>,
    temperature: f64,
    max_output_tokens: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelfarmChatCompletionRequest {
    pub model: ModelfarmChatModel,
    pub parameters: Parameters,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    pub message: ChatMessage,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatPromptResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelfarmChatCompletionResponse {
    pub metadata: Option<Metadata>,
    pub responses: Vec<ChatPromptResponse>,
}
