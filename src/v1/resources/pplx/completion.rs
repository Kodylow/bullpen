use serde::{Deserialize, Serialize};

use crate::v1::models::PplxCompletionModel;
use crate::v1::resources::shared::{FinishReason, Usage};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PplxCompletionRequest {
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
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
}

impl Default for PplxCompletionRequest {
    fn default() -> Self {
        PplxCompletionRequest {
            model: PplxCompletionModel::ReplitCodeV15_3b,
            prompt: "Some example like...".to_string(),
            temperature: None,
            top_p: None,
            top_k: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PplxCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CompletionChoice {
    pub text: String,
    pub index: u32,
    pub finish_reason: FinishReason,
}
