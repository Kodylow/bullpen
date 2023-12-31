use std::pin::Pin;

use futures::stream::StreamExt;
use futures::{Stream, TryStreamExt};
use reqwest::multipart::Form;
use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio_util::bytes::Bytes;

use crate::error::APIError;

const PPLX_API_ENDPOINT: &str = "https://api.perplexity.ai";
const MODELFARM_API_ENDPOINT: &str = "https://proxy-modelfarm.replit.app";
const OLLAMA_API_ENDPOINT: &str = "http://localhost:11434";

pub struct Pplx {
    pub http_client: reqwest::Client,
    pub base_url: String,
    pub api_key: String,
}

impl Pplx {
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: PPLX_API_ENDPOINT.to_string(),
            api_key,
        }
    }

    pub async fn get(&self, path: &str) -> Result<String, APIError> {
        let url = format!("{}{}", &self.base_url, path);

        let response = self
            .http_client
            .get(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .bearer_auth(&self.api_key)
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        let response_text = response.text().await.unwrap();

        #[cfg(feature = "log")]
        log::trace!("{}", response_text);

        Ok(response_text)
    }

    pub async fn post<T: Serialize>(&self, path: &str, parameters: &T) -> Result<String, APIError> {
        let url = format!("{}{}", &self.base_url, path);

        let response =
            self.http_client
                .post(url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .bearer_auth(&self.api_key)
                .json(&parameters)
                .send()
                .await
                .unwrap();

        if !response.status().is_success() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn delete(&self, path: &str) -> Result<String, APIError> {
        let url = format!("{}{}", &self.base_url, path);

        let response = self
            .http_client
            .delete(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .bearer_auth(&self.api_key)
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post_with_form(&self, path: &str, form: Form) -> Result<String, APIError> {
        let url = format!("{}{}", &self.base_url, path);

        let response = self
            .http_client
            .post(url)
            // .header(reqwest::header::CONTENT_TYPE, "multipart/form-data")
            .bearer_auth(&self.api_key)
            .multipart(form)
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post_stream<I, O>(
        &self,
        path: &str,
        parameters: &I,
    ) -> Pin<Box<dyn Stream<Item = Result<O, APIError>> + Send>>
    where
        I: Serialize,
        O: DeserializeOwned + std::marker::Send + 'static,
    {
        let url = format!("{}{}", &self.base_url, path);

        let event_source = self
            .http_client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .bearer_auth(&self.api_key)
            .json(&parameters)
            .eventsource()
            .unwrap();

        Pplx::process_stream::<O>(event_source).await
    }

    pub async fn process_stream<O>(
        mut event_source: EventSource,
    ) -> Pin<Box<dyn Stream<Item = Result<O, APIError>> + Send>>
    where
        O: DeserializeOwned + Send + 'static,
    {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        #[derive(serde::Deserialize)]
        struct StreamErrorWrapper {
            error: StreamError,
        }

        #[derive(serde::Deserialize)]
        struct StreamError {
            message: String,
            #[serde(rename = "type")]
            error_type: String,
            _param: Option<serde_json::Value>,
            _code: Option<u8>,
        }

        tokio::spawn(async move {
            while let Some(event_result) = event_source.next().await {
                match event_result {
                    Ok(event) => match event {
                        Event::Open => continue,
                        Event::Message(message) => {
                            if message.data == "[DONE]" {
                                break;
                            }

                            let response = match serde_json::from_str::<O>(&message.data) {
                                Ok(result) => Ok(result),
                                Err(error) => {
                                    // Try to parse an error message from the stream
                                    match serde_json::from_str::<StreamErrorWrapper>(&message.data) {
                                        Ok(error_wrapper) => Err(APIError::StreamError(format!("OpenAI {}: {}",  error_wrapper.error.error_type, error_wrapper.error.message))),
                                        Err(_) => Err(APIError::StreamError(format!("OpenAI error parsing event stream: {}\nstream data: {}", error.to_string(), message.data))),
                                    }
                                }
                            };

                            if let Err(_error) = tx.send(response) {
                                break;
                            }
                        }
                    },
                    Err(_error) => {
                        // if let Err(_error) =
                        // tx.send(Err(APIError::StreamError(error.
                        // to_string()))) {
                        //     break;
                        // }
                    }
                }
            }

            event_source.close();
        });

        Box::pin(tokio_stream::wrappers::UnboundedReceiverStream::new(rx))
    }
}

pub struct Modelfarm {
    pub http_client: reqwest::Client,
    pub base_url: String,
}

impl Modelfarm {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: MODELFARM_API_ENDPOINT.to_string(),
        }
    }

    pub async fn get(&self, path: &str) -> Result<String, APIError> {
        let url = format!("{}{}", &self.base_url, path);

        let response = self
            .http_client
            .get(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post<T: Serialize>(&self, path: &str, parameters: &T) -> Result<String, APIError> {
        let url = format!("{}{}", &self.base_url, path);

        let response = self
            .http_client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&parameters)
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post_stream<I>(
        &self,
        path: &str,
        parameters: &I,
    ) -> Pin<Box<dyn Stream<Item = anyhow::Result<Bytes>> + Send>>
    where
        I: Serialize,
    {
        let url = format!("{}{}", &self.base_url, path);

        let request = self
            .http_client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&parameters)
            .build()
            .unwrap();

        Box::pin(
            self.http_client
                .execute(request)
                .await
                .map_err(anyhow::Error::from)
                .unwrap()
                .bytes_stream()
                .map_err(anyhow::Error::from),
        )
    }
}

pub struct Ollama {
    pub http_client: reqwest::Client,
    pub base_url: String,
}

impl Ollama {
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            base_url: OLLAMA_API_ENDPOINT.to_string(),
        }
    }

    pub async fn get(&self, path: &str) -> Result<String, APIError> {
        let url = format!("{}{}", &self.base_url, path);

        let response = self
            .http_client
            .get(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap();

        if response.status().is_server_error() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        let response_text = response.text().await.unwrap();

        Ok(response_text)
    }

    pub async fn post<T: Serialize>(&self, path: &str, parameters: &T) -> Result<String, APIError> {
        let url = format!("{}{}", &self.base_url, path);

        let response = self
            .http_client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&parameters)
            .send()
            .await
            .unwrap();

        if !response.status().is_success() {
            return Err(APIError::EndpointError(response.text().await.unwrap()));
        }

        Ok(response.text().await.unwrap())
    }

    pub async fn post_stream<I>(
        &self,
        path: &str,
        parameters: &I,
    ) -> Pin<Box<dyn Stream<Item = anyhow::Result<Bytes>> + Send>>
    where
        I: Serialize,
    {
        let url = format!("{}{}", &self.base_url, path);

        let request = self
            .http_client
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&parameters)
            .build()
            .unwrap();

        Box::pin(
            self.http_client
                .execute(request)
                .await
                .map_err(anyhow::Error::from)
                .unwrap()
                .bytes_stream()
                .map_err(anyhow::Error::from),
        )
    }
}
