use crate::v1::api::Ollama;
use crate::v1::error::APIError;
use crate::v1::resources::ollama::embedding::{OllamaEmbeddingRequest, OllamaEmbeddingResponse};

impl Ollama {
    pub async fn embed(
        &self,
        params: OllamaEmbeddingRequest,
    ) -> Result<OllamaEmbeddingResponse, APIError> {
        let endpoint = "/api/embeddings";
        let response = self.post(endpoint, &params).await?;
        let embed_response: OllamaEmbeddingResponse = serde_json::from_str(&response).unwrap();

        Ok(embed_response)
    }
}
