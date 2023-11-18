use std::pin::Pin;

use futures::Stream;
use serde::Serialize;
use serde_json::Value;

use crate::v1::api::Client;
use crate::v1::error::APIError;
use crate::v1::models::PplxChatModel;
use crate::v1::resources::chat_completion::{
    ChatCompletionParameters, ChatCompletionResponse, ChatMessage,
};
use crate::v1::resources::chat_completion_stream::ChatCompletionStreamResponse;

pub struct Chat<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn chat(&self) -> Chat {
        Chat { client: self }
    }
}

impl Chat<'_> {
    pub async fn create(
        &self,
        parameters: ChatCompletionParameters,
    ) -> Result<ChatCompletionResponse, APIError> {
        let response = self.client.post("/chat/completions", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let chat_completion_response: ChatCompletionResponse = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(chat_completion_response)
    }

    pub async fn create_stream(
        &self,
        parameters: ChatCompletionParameters,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ChatCompletionStreamResponse, APIError>> + Send>>,
        APIError,
    > {
        let stream_parameters = ChatCompletionStreamParameters {
            model: parameters.model,
            messages: parameters.messages,
            temperature: parameters.temperature,
            top_p: parameters.top_p,
            top_k: parameters.top_k,
            stream: Some(true),
            max_tokens: parameters.max_tokens,
            presence_penalty: parameters.presence_penalty,
            frequency_penalty: parameters.frequency_penalty,
        };

        Ok(self
            .client
            .post_stream("/chat/completions", &stream_parameters)
            .await)
    }
}

#[derive(Serialize, Debug)]
struct ChatCompletionStreamParameters {
    pub model: PplxChatModel,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
}
