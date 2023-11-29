use std::pin::Pin;

use anyhow::Result;
use futures::{Stream, StreamExt};
use tokio_util::bytes::Bytes;

use crate::api::Modelfarm;
use crate::error::APIError;
use crate::resources::modelfarm::chat_completion::{
    ModelfarmChatCompletionRequest, ModelfarmChatCompletionResponse,
};

impl Modelfarm {
    pub async fn chat(
        &self,
        modelfarm_params: ModelfarmChatCompletionRequest,
    ) -> Result<ModelfarmChatCompletionResponse, APIError> {
        let endpoint = "/v1beta/chat";
        println!("Sending request to {}", endpoint);
        let response = self.post(endpoint, &modelfarm_params).await?;
        let chat_completion_response: ModelfarmChatCompletionResponse =
            serde_json::from_str(&response).unwrap();

        Ok(chat_completion_response)
    }

    pub async fn stream_chat(
        &self,
        modelfarm_params: ModelfarmChatCompletionRequest,
    ) -> Result<
        Pin<Box<dyn Stream<Item = Result<ModelfarmChatCompletionResponse, anyhow::Error>> + Send>>,
    > {
        let endpoint = "/v1beta/chat_streaming";
        let res: Pin<Box<dyn Stream<Item = Result<Bytes, anyhow::Error>> + Send>> =
            self.post_stream(endpoint, &modelfarm_params).await;
        let stream = res.map(|res| {
            let res = res.map_err(anyhow::Error::from)?;
            let chat_response: ModelfarmChatCompletionResponse = serde_json::from_slice(&res)?;
            Ok(chat_response)
        });
        Ok(Box::pin(stream))
    }
}
