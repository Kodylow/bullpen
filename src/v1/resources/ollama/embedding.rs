use serde::{Deserialize, Serialize};

use crate::v1::models::OllamaModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaEmbeddingRequest {
    pub model: OllamaModel,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaEmbeddingResponse {
    pub embedding: Vec<f64>,
}
