use serde::{Deserialize, Serialize};

use crate::v1::models::PplxCompletionModel;

#[derive(Serialize, Debug, Clone)]
pub struct PplxCompletionStreamParameters {
    pub model: PplxCompletionModel,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PplxCompletionStreamResponse {
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
