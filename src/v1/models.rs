use std::fmt::{Display, Error, Formatter, Result};

use serde::{Deserialize, Serialize};

// https://api.perplexity.ai
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PplxModel {
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

impl Display for PplxModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", serde_json::to_string(self).map_err(|_| Error)?)
    }
}
