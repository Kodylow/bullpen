use std::pin::Pin;

use futures::Stream;

use crate::api::Pplx;
use crate::error::APIError;
use crate::resources::pplx::chat_completion::{
    PplxChatCompletionRequest, PplxChatCompletionResponse,
};
use crate::resources::pplx::chat_completion_stream::{
    PplxChatCompletionStreamRequest, PplxChatCompletionStreamResponse,
};

impl Pplx {
    pub async fn chat(
        &self,
        pplx_params: PplxChatCompletionRequest,
    ) -> Result<PplxChatCompletionResponse, APIError> {
        let endpoint = "/chat/completions";
        let response = self.post(endpoint, &pplx_params).await?;
        let chat_completion_response: PplxChatCompletionResponse =
            serde_json::from_str(&response).unwrap();

        Ok(chat_completion_response)
    }

    pub async fn stream_chat(
        &self,
        params: PplxChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<PplxChatCompletionStreamResponse, APIError>> + Send>>,
        APIError,
    > {
        let endpoint = "/chat/completions";
        let stream_params = PplxChatCompletionStreamRequest {
            model: params.model,
            messages: params.messages,
            temperature: params.temperature,
            top_p: params.top_p,
            top_k: params.top_k,
            stream: Some(true),
            max_tokens: params.max_tokens,
            presence_penalty: params.presence_penalty,
            frequency_penalty: params.frequency_penalty,
        };

        Ok(self.post_stream(endpoint, &stream_params).await)
    }
}
