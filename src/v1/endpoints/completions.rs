use std::collections::HashMap;
use std::pin::Pin;

use futures::Stream;
use serde::Serialize;
use serde_json::Value;

use crate::v1::api::Client;
use crate::v1::error::APIError;
#[cfg(feature = "simple")]
use crate::v1::resources::completion::SimpleCompletionParameters;
use crate::v1::resources::completion::{CompletionParameters, CompletionResponse};
use crate::v1::resources::completion_stream::CompletionStreamResponse;
use crate::v1::resources::shared::StopToken;
pub struct Completions<'a> {
    pub client: &'a Client,
}

impl Client {
    pub fn completions(&self) -> Completions {
        Completions { client: self }
    }
}

impl Completions<'_> {
    pub async fn create(
        &self,
        parameters: CompletionParameters,
    ) -> Result<CompletionResponse, APIError> {
        let response = self.client.post("/completions", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let completion_response: CompletionResponse = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(completion_response)
    }

    #[cfg(feature = "simple")]
    pub async fn create_simple(
        &self,
        parameters: SimpleCompletionParameters,
    ) -> Result<CompletionResponse, APIError> {
        let response = self.client.post("/completions", &parameters).await?;

        let value: Value = serde_json::from_str(&response).unwrap();
        let completion_response: CompletionResponse = serde_json::from_value(value)
            .map_err(|error| APIError::ParseError(error.to_string()))?;

        Ok(completion_response)
    }

    pub async fn create_stream(
        &self,
        parameters: CompletionParameters,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<CompletionStreamResponse, APIError>> + Send>>,
        APIError,
    > {
        let stream_parameters = CompletionStreamParameters {
            model: parameters.model,
            prompt: parameters.prompt,
            suffix: None,
            max_tokens: Some(50),
            temperature: parameters.temperature,
            top_p: parameters.top_p,
            n: parameters.n,
            stream: true,
            logprobs: parameters.logprobs,
            echo: parameters.echo,
            stop: parameters.stop,
            presence_penalty: parameters.presence_penalty,
            frequency_penalty: parameters.frequency_penalty,
            best_of: parameters.best_of,
            logit_bias: parameters.logit_bias,
        };

        Ok(self
            .client
            .post_stream("/completions", &stream_parameters)
            .await)
    }
}

#[derive(Serialize, Debug)]
struct CompletionStreamParameters {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopToken>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>,
}
