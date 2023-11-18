//! # Getting started
//!
//! Pplx Client is an unofficial async Rust library that allows you to interact
//! with the Perplexity API.
//!
//! ## Endpoints
//!
//! - Chat
//!   - [Create chat completion](#create-chat-completion)
//!   - [Create chat completion (stream)](#create-chat-completion-stream)
//!
//! # Endpoints
//!
//! ## Create chat completion
//!
//! Creates a completion for the chat message.
//!
//! **URL** `https://api.openai.com/v1/chat/completions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use pplx_client::v1::api::Client;
//! use pplx_client::v1::resources::chat_completion::{
//!     ChatCompletionParameters, ChatMessage, Role,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = ChatCompletionParameters {
//!         model: "gpt-3.5-turbo-0613".to_string(),
//!         messages: vec![
//!             ChatMessage {
//!                 role: Role::User,
//!                 content: "Hello!".to_string(),
//!                 ..Default::default()
//!             },
//!             ChatMessage {
//!                 role: Role::User,
//!                 content: "Where are you located?".to_string(),
//!                 ..Default::default()
//!             },
//!         ],
//!         temperature: None,
//!         top_p: None,
//!         n: None,
//!         stop: None,
//!         max_tokens: Some(12),
//!         presence_penalty: None,
//!         frequency_penalty: None,
//!         logit_bias: None,
//!         user: None,
//!         ..Default::default()
//!     };
//!
//!     let result = client.chat().create(parameters).await.unwrap();
//!
//!     println!("{:?}", result);
//! }
//! ```
//!
//! More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)
//!
//! ## Create chat completion (stream)
//!
//! Creates a completion for the chat message.
//!
//! **URL** `https://api.openai.com/v1/chat/completions`
//!
//! **Method** `POST`
//!
//! ```rust
//! use futures::StreamExt;
//! use pplx_client::v1::api::Client;
//! use pplx_client::v1::resources::chat_completion::{
//!     ChatCompletionParameters, ChatMessage, Role,
//! };
//!
//! #[tokio::main]
//! async fn main() {
//!     let api_key = std::env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");
//!
//!     let client = Client::new(api_key);
//!
//!     let parameters = ChatCompletionParameters {
//!         model: "gpt-3.5-turbo-0301".to_string(),
//!         messages: vec![
//!             ChatMessage {
//!                 role: Role::User,
//!                 content: "Hello!".to_string(),
//!                 ..Default::default()
//!             },
//!             ChatMessage {
//!                 role: Role::User,
//!                 content: "Where are you located?".to_string(),
//!                 ..Default::default()
//!             },
//!         ],
//!         temperature: None,
//!         top_p: None,
//!         n: None,
//!         stop: None,
//!         max_tokens: Some(12),
//!         presence_penalty: None,
//!         frequency_penalty: None,
//!         logit_bias: None,
//!         user: None,
//!         ..Default::default()
//!     };
//!
//!     let mut stream = client.chat().create_stream(parameters).await.unwrap();
//!
//!     while let Some(response) = stream.next().await {
//!         match response {
//!             Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
//!                 if let Some(content) = choice.delta.content.as_ref() {
//!                     print!("{}", content);
//!                 }
//!             }),
//!             Err(e) => eprintln!("{}", e),
//!         }
//!     }
//! }
//! ```
//!
//! More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)
//!
//!
//! ## Use model names
//!
//! [https://docs.perplexity.ai/docs/model-cards](https://docs.perplexity.ai/docs/model-cards)
//!
//! ```rust
//! use pplx_client::v1::models::PplxModel;
//!
//! assert_eq!(PplxModel::Codellama34bInstruct.to_string(),
//! "codellama-34b-instruct"); assert_eq!(PplxModel::Llama213bChat.to_string(),
//! "llama-2-13b-chat"); assert_eq!(PplxModel::Llama270bChat.to_string(),
//! "llama-2-70b-chat"); assert_eq!(PplxModel::Mistral7bInstruct.to_string(),
//! "mistral-7b-instruct"); assert_eq!(PplxModel::Openhermes2Mistral7b.
//! to_string(), "openhermes-2-mistral-7b");
//! assert_eq!(PplxModel::Openhermes25Mistral7b.to_string(),
//! "openhermes-2.5-mistral-7b"); assert_eq!(PplxModel::Pplx7bChatAlpha.
//! to_string(), "pplx-7b-chat-alpha"); assert_eq!(PplxModel::Pplx70bChatAlpha.
//! to_string(), "pplx-70b-chat-alpha"); ```
pub mod v1;
