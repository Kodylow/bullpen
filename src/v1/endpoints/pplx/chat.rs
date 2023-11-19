use std::pin::Pin;

use futures::Stream;

use crate::v1::api::Pplx;
use crate::v1::error::APIError;
use crate::v1::resources::pplx::chat_completion::{
    PplxChatCompletionParameters, PplxChatCompletionResponse,
};
use crate::v1::resources::pplx::chat_completion_stream::{
    PplxChatCompletionStreamParameters, PplxChatCompletionStreamResponse,
};

pub struct Chat<'a> {
    pub client: &'a Pplx,
}

impl Pplx {
    pub fn chat(&self) -> Chat {
        Chat { client: self }
    }
}

impl Chat<'_> {
    pub async fn create(
        &self,
        pplx_params: PplxChatCompletionParameters,
    ) -> Result<PplxChatCompletionResponse, APIError> {
        let endpoint = "/chat/completions";
        let response = self.client.post(endpoint, &pplx_params).await?;
        let chat_completion_response: PplxChatCompletionResponse =
            serde_json::from_str(&response).unwrap();

        Ok(chat_completion_response)
    }

    pub async fn create_stream(
        &self,
        params: PplxChatCompletionParameters,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<PplxChatCompletionStreamResponse, APIError>> + Send>>,
        APIError,
    > {
        let endpoint = "/chat/completions";
        let stream_params = PplxChatCompletionStreamParameters {
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

        Ok(self.client.post_stream(endpoint, &stream_params).await)
    }
}
