use serde::{Deserialize, Serialize};

pub mod chat_completion;
pub mod completion;
pub mod embedding;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenCountMetadata {
    pub billable_tokens: i32,
    pub unbilled_tokens: i32,
    pub billable_characters: i32,
    pub unbilled_characters: i32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub input_token_count: Option<TokenCountMetadata>,
    pub output_token_count: Option<TokenCountMetadata>,
}
