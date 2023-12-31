use std::pin::Pin;

use futures::Stream;
use tokio_stream::StreamExt;

use crate::api::Ollama;
use crate::error::APIError;
use crate::resources::ollama::completion::{OllamaCompletionRequest, OllamaCompletionResponse};

impl Ollama {
    pub async fn create(
        &self,
        ollama_params: OllamaCompletionRequest,
    ) -> Result<OllamaCompletionResponse, APIError> {
        let endpoint = "/api/generate";
        let response = self.post(endpoint, &ollama_params).await?;
        println!("{}", response);
        let completion_response: Result<OllamaCompletionResponse, _> =
            serde_json::from_str(&response);

        match completion_response {
            Ok(data) => Ok(data),
            Err(_) => Err(APIError::ParseError("Invalid JSON response".to_string())),
        }
    }

    pub async fn create_stream(
        &self,
        ollama_params: OllamaCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<OllamaCompletionResponse, APIError>> + Send>>,
        APIError,
    > {
        let endpoint = "/api/generate";
        let res = self
            .post_stream(endpoint, &ollama_params.to_stream_request())
            .await;
        let stream = res.map(|res| match res {
            Ok(res) => match serde_json::from_slice::<OllamaCompletionResponse>(&res) {
                Ok(completion_response) => Ok(completion_response),
                Err(_) => Err(APIError::ParseError("Invalid JSON response".to_string())),
            },
            Err(e) => Err(APIError::StreamError(e.to_string())),
        });

        Ok(Box::pin(stream))
    }
}
