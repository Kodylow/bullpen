use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::OllamaModel;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct OllamaChatRequest {
    pub model: OllamaModel,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

impl Default for OllamaChatRequest {
    fn default() -> Self {
        OllamaChatRequest {
            model: OllamaModel::Mistral,
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello!".to_string(),
                images: None,
            }],
            format: None,
            options: None,
            template: None,
            stream: None,
        }
    }
}

impl OllamaChatRequest {
    pub fn to_stream_request(&self) -> OllamaChatStreamRequest {
        OllamaChatStreamRequest {
            model: self.model.clone(),
            messages: self.messages.clone(),
            format: self.format.clone(),
            options: self.options.clone(),
            template: self.template.clone(),
            stream: Some(true),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct OllamaChatStreamRequest {
    pub model: OllamaModel,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    pub stream: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OllamaChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: Message,
    pub done: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_eval_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_duration: Option<u64>,
}
