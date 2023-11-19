use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::v1::models::PplxChatModel;
use crate::v1::resources::shared::{FinishReason, Usage};

#[derive(Serialize, Debug, Clone)]
pub struct PplxSimpleChatCompletionParameters {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PplxChatCompletionRequest {
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
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
}

impl Default for PplxChatCompletionRequest {
    fn default() -> Self {
        PplxChatCompletionRequest {
            model: PplxChatModel::Mistral7bInstruct,
            messages: vec![ChatMessage {
                role: Role::User,
                content: "Hello!".to_string(),
                name: None,
            }],
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
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for ChatMessage {
    fn default() -> Self {
        ChatMessage {
            role: Role::User,
            content: "".to_string(),
            name: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PplxChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub message: ChatMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub finish_reason: Option<FinishReason>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Function,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "system" => Ok(Role::System),
            "user" => Ok(Role::User),
            "assistant" => Ok(Role::Assistant),
            _ => Err(format!("{} is not a valid Role", s)),
        }
    }
}
