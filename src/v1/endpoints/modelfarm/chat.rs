use std::pin::Pin;

use futures::Stream;

use crate::v1::api::Modelfarm;
use crate::v1::error::APIError;
use crate::v1::resources::modelfarm::chat_completion::{
    ModelfarmChatCompletionRequest, ModelfarmChatCompletionResponse,
};

impl Modelfarm {
    pub async fn chat(
        &self,
        modelfarm_params: ModelfarmChatCompletionRequest,
    ) -> Result<ModelfarmChatCompletionResponse, APIError> {
        let endpoint = "/v1beta/chat";
        let response = self.post(endpoint, &modelfarm_params).await?;
        let chat_completion_response: ModelfarmChatCompletionResponse =
            serde_json::from_str(&response).unwrap();

        Ok(chat_completion_response)
    }

    pub async fn stream_chat(
        &self,
        modelfarm_params: ModelfarmChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ModelfarmChatCompletionResponse, APIError>> + Send>>,
        APIError,
    > {
        let endpoint = "/v1beta/chat_streaming";
        Ok(self.post_stream(endpoint, &modelfarm_params).await)
    }
}
