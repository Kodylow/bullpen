use std::pin::Pin;

use futures::Stream;

use crate::v1::api::Modelfarm;
use crate::v1::error::APIError;
use crate::v1::resources::modelfarm::completion::{
    ModelfarmCompletionRequest, ModelfarmCompletionResponse,
};

impl Modelfarm {
    pub async fn complete(
        &self,
        modelfarm_params: ModelfarmCompletionRequest,
    ) -> Result<ModelfarmCompletionResponse, APIError> {
        let endpoint = "/v1beta/completions";
        let response = self.post(endpoint, &modelfarm_params).await?;
        let completion_response: ModelfarmCompletionResponse =
            serde_json::from_str(&response).unwrap();

        Ok(completion_response)
    }

    pub async fn stream_complete(
        &self,
        modelfarm_params: ModelfarmCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ModelfarmCompletionResponse, APIError>> + Send>>,
        APIError,
    > {
        let endpoint = "/v1beta/completions_streaming";
        Ok(self.post_stream(endpoint, &modelfarm_params).await)
    }
}
