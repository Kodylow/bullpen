use std::pin::Pin;

use futures::Stream;
use tokio_stream::StreamExt;

use crate::api::Ollama;
use crate::error::APIError;
use crate::resources::ollama::chat::{OllamaChatRequest, OllamaChatResponse};

impl Ollama {
    pub async fn chat(
        &self,
        ollama_params: OllamaChatRequest,
    ) -> Result<OllamaChatResponse, APIError> {
        let endpoint = "/api/chat";
        let response = self.post(endpoint, &ollama_params).await?;
        println!("{}", response);
        let chat_response: Result<OllamaChatResponse, _> = serde_json::from_str(&response);

        match chat_response {
            Ok(data) => Ok(data),
            Err(_) => Err(APIError::ParseError("Invalid JSON response".to_string())),
        }
    }

    pub async fn chat_stream(
        &self,
        ollama_params: OllamaChatRequest,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<OllamaChatResponse, APIError>> + Send>>, APIError>
    {
        let endpoint = "/api/chat";
        let res = self
            .post_stream(endpoint, &ollama_params.to_stream_request())
            .await;
        let stream = res.map(|res| match res {
            Ok(res) => match serde_json::from_slice::<OllamaChatResponse>(&res) {
                Ok(chat_response) => Ok(chat_response),
                Err(_) => Err(APIError::ParseError("Invalid JSON response".to_string())),
            },
            Err(e) => Err(APIError::StreamError(e.to_string())),
        });

        Ok(Box::pin(stream))
    }
}
