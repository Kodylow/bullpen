#[cfg(feature = "download")]
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::models::{
    ModelfarmChatModel, ModelfarmCompletionModel, ModelfarmEmbeddingModel, PplxChatModel,
    PplxCompletionModel,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseModel {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FinishReason {
    #[serde(rename(deserialize = "stop"))]
    StopSequenceReached,
    #[serde(rename(deserialize = "length"))]
    TokenLimitReached,
    #[serde(rename(deserialize = "content_filter"))]
    ContentFilterFlagged,
    #[serde(rename(deserialize = "function_call"))]
    FunctionCall,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum StopToken {
    String(String),
    Array(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ChatModel {
    #[serde(rename = "modelfarm")]
    Modelfarm(ModelfarmChatModel),
    #[serde(rename = "perplexity")]
    Perplexity(PplxChatModel),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum CompletionModel {
    #[serde(rename = "modelfarm")]
    Modelfarm(ModelfarmCompletionModel),
    #[serde(rename = "perplexity")]
    Perplexity(PplxCompletionModel),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum EmbeddingModel {
    #[serde(rename = "modelfarm")]
    Modelfarm(ModelfarmEmbeddingModel),
}

impl ChatModel {
    pub fn get_endpoint(&self) -> &'static str {
        match self {
            ChatModel::Modelfarm(_) => "/v1beta/chat",
            ChatModel::Perplexity(_) => "/chat/completions",
        }
    }
}

impl CompletionModel {
    pub fn get_endpoint(&self) -> &'static str {
        match self {
            CompletionModel::Modelfarm(_) => "/v1beta/text",
            CompletionModel::Perplexity(_) => "/completions",
        }
    }
}

impl EmbeddingModel {
    pub fn get_endpoint(&self) -> &'static str {
        match self {
            EmbeddingModel::Modelfarm(_) => "/v1beta/embedding",
        }
    }
}
