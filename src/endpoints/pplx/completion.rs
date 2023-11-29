use std::pin::Pin;

use futures::Stream;

use crate::api::Pplx;
use crate::error::APIError;
use crate::resources::pplx::completion::{PplxCompletionRequest, PplxCompletionResponse};
use crate::resources::pplx::completion_stream::PplxCompletionStreamRequest;

impl Pplx {
    pub async fn create(
        &self,
        pplx_params: PplxCompletionRequest,
    ) -> Result<PplxCompletionResponse, APIError> {
        let endpoint = "/completions";
        let response = self.post(endpoint, &pplx_params).await?;
        let completion_response: PplxCompletionResponse = serde_json::from_str(&response).unwrap();

        Ok(completion_response)
    }

    pub async fn create_stream(
        &self,
        pplx_params: PplxCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<PplxCompletionResponse, APIError>> + Send>>,
        APIError,
    > {
        let endpoint = "/completions";
        let stream_params = PplxCompletionStreamRequest {
            model: pplx_params.model,
            prompt: pplx_params.prompt,
            max_tokens: pplx_params.max_tokens,
            temperature: pplx_params.temperature,
            top_p: pplx_params.top_p,
            top_k: pplx_params.top_k,
            stream: Some(true),
            presence_penalty: pplx_params.presence_penalty,
            frequency_penalty: pplx_params.frequency_penalty,
        };

        Ok(self.post_stream(endpoint, &stream_params).await)
    }
}
