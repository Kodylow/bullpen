use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Metadata;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameters {
    pub prompts: Vec<String>,
    pub temperature: f64,
    pub max_output_tokens: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelfarmCompletionRequest {
    pub model: String,
    pub parameters: Parameters,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub content: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelfarmCompletionResponse {
    pub metadata: Option<Metadata>,
    pub responses: Vec<PromptResponse>,
}
