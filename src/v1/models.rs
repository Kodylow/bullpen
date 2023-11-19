use std::fmt::{Display, Error, Formatter, Result};

use serde::{Deserialize, Serialize};

// https://api.perplexity.ai
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PplxChatModel {
    #[serde(rename = "mistral-7b-instruct")]
    Mistral7bInstruct,
    #[serde(rename = "codellama-34b-instruct")]
    Codellama34bInstruct,
    #[serde(rename = "llama-2-13b-chat")]
    Llama213bChat,
    #[serde(rename = "llama-2-70b-chat")]
    Llama270bChat,
    #[serde(rename = "openhermes-2-mistral-7b")]
    Openhermes2Mistral7b,
    #[serde(rename = "openhermes-2.5-mistral-7b")]
    Openhermes25Mistral7b,
    #[serde(rename = "pplx-7b-chat-alpha")]
    Pplx7bChatAlpha,
    #[serde(rename = "pplx-70b-chat-alpha")]
    Pplx70bChatAlpha,
}

impl Display for PplxChatModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PplxCompletionModel {
    #[serde(rename = "replit-code-v1.5-3b")]
    ReplitCodeV15_3b,
}

impl Display for PplxCompletionModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModelfarmChatModel {
    #[serde(rename = "chat-bison")]
    ChatBison,
}

impl Display for ModelfarmChatModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModelfarmCompletionModel {
    #[serde(rename = "text-bison")]
    TextBison,
}

impl Display for ModelfarmCompletionModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ModelfarmEmbeddingModel {
    #[serde(rename = "textembedding-gecko")]
    TextEmbeddingGecko,
}
