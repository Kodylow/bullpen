use std::pin::Pin;

use futures::Stream;

use crate::v1::api::Pplx;
use crate::v1::error::APIError;
use crate::v1::resources::pplx::completion::{PplxCompletionParameters, PplxCompletionResponse};
use crate::v1::resources::pplx::completion_stream::PplxCompletionStreamParameters;

pub struct Completions<'a> {
    pub client: &'a Pplx,
}

impl Pplx {
    pub fn completions(&self) -> Completions {
        Completions { client: self }
    }
}

impl Completions<'_> {
    pub async fn create(
        &self,
        pplx_params: PplxCompletionParameters,
    ) -> Result<PplxCompletionResponse, APIError> {
        let endpoint = "/completions";
        let response = self.client.post(endpoint, &pplx_params).await?;
        let completion_response: PplxCompletionResponse = serde_json::from_str(&response).unwrap();

        Ok(completion_response)
    }

    pub async fn create_stream(
        &self,
        pplx_params: PplxCompletionParameters,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<PplxCompletionResponse, APIError>> + Send>>,
        APIError,
    > {
        let endpoint = "/completions";
        let stream_params = PplxCompletionStreamParameters {
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

        Ok(self.client.post_stream(endpoint, &stream_params).await)
    }
}
