use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Metadata;
use crate::models::ModelfarmChatModel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub content: String,
    pub author: String,
}

impl Default for ChatMessage {
    fn default() -> Self {
        ChatMessage {
            content: String::new(),
            author: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatExample {
    pub input: ChatMessage,
    pub output: ChatMessage,
}

impl Default for ChatExample {
    fn default() -> Self {
        ChatExample {
            input: ChatMessage::default(),
            output: ChatMessage::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatSession {
    pub context: String,
    pub examples: Vec<ChatExample>,
    pub messages: Vec<ChatMessage>,
}

impl Default for ChatSession {
    fn default() -> Self {
        ChatSession {
            context: String::new(),
            examples: Vec::new(),
            messages: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelfarmChatParameters {
    pub prompts: Vec<ChatSession>,
    pub temperature: f64,
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: usize,
}

impl Default for ModelfarmChatParameters {
    fn default() -> Self {
        ModelfarmChatParameters {
            prompts: Vec::new(),
            temperature: 1.0,
            max_output_tokens: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelfarmChatCompletionRequest {
    pub model: ModelfarmChatModel,
    pub parameters: ModelfarmChatParameters,
}

impl Default for ModelfarmChatCompletionRequest {
    fn default() -> Self {
        ModelfarmChatCompletionRequest {
            model: ModelfarmChatModel::default(),
            parameters: ModelfarmChatParameters::default(),
        }
    }
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
