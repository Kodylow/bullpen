use crate::v1::api::Modelfarm;
use crate::v1::error::APIError;
use crate::v1::resources::modelfarm::embedding::{
    ModelfarmEmbeddingRequest, ModelfarmEmbeddingResponse,
};

impl Modelfarm {
    pub async fn embed(
        &self,
        params: ModelfarmEmbeddingRequest,
    ) -> Result<ModelfarmEmbeddingResponse, APIError> {
        let endpoint = "/v1beta/embedding";
        let response = self.post(endpoint, &params).await?;
        let embed_response: ModelfarmEmbeddingResponse = serde_json::from_str(&response).unwrap();

        Ok(embed_response)
    }
}
