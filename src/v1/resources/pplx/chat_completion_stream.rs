use serde::{Deserialize, Serialize};

use super::chat_completion::ChatMessage;
use crate::v1::models::PplxChatModel;
use crate::v1::resources::pplx::chat_completion::Role;
use crate::v1::resources::shared::FinishReason;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PplxChatCompletionStreamRequest {
    pub model: PplxChatModel,
    pub messages: Vec<ChatMessage>,
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
pub struct PplxChatCompletionStreamResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<DeltaField>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeltaField {
    pub delta: DeltaValue,
    pub index: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub finish_reason: Option<FinishReason>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeltaValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<Role>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content: Option<String>,
}
