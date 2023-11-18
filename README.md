# Pplx Client

Pplx Client is an unofficial async Rust library that allows you to interact with the Perplexity API.

For Developers and Contributors: [Contributions](#contributions)

Forked from OpenAI Dive: https://github.com/tjardoo/pplx-client

## Endpoints

- Chat
  - [Create chat completion](#create-chat-completion)
  - [Create chat completion (stream)](#create-chat-completion-stream)
- Completions
  - [Create completion](#create-completion)
  - [Create completion (stream)](#create-completion-stream)
- Embeddings
  - [Create embedding](#create-embedding)

### Create chat completion

Creates a completion for the chat message.

**URL** `https://api.perplexity.ai/chat/completions`

**Method** `POST`

```rust
use pplx_client::v1::api::Client;
use pplx_client::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage, Role};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: "gpt-3.5-turbo-0613".to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: "Hello!".to_string(),
                ..Default::default(),
            },
            ChatMessage {
                role: Role::User,
                content: "Where are you located?".to_string(),
                ..Default::default(),
            },
        ],
        temperature: None,
        top_p: None,
        n: None,
        stop: None,
        max_tokens: Some(12),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        // or use ..Default::default()
    };

    let result = client.chat().create(parameters).await.unwrap();

    println!("{:?}", result);
}
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

### Create chat completion (stream)

> Feature `stream` required

Creates a completion for the chat message.

**URL** `https://api.perplexity.ai/chat/completions`

**Method** `POST`

```rust
use futures::StreamExt;
use pplx_client::v1::api::Client;
use pplx_client::v1::resources::chat_completion::{ChatCompletionParameters, ChatMessage, Role};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Client::new(api_key);

    let parameters = ChatCompletionParameters {
        model: "mistral-7b-instruct".to_string(),
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: "Hello!".to_string(),
                ..Default::default(),
            },
            ChatMessage {
                role: Role::User,
                content: "Where are you located?".to_string(),
                ..Default::default(),
            },
        ],
        temperature: None,
        top_p: None,
        n: None,
        stop: None,
        max_tokens: Some(12),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
    };

    let mut stream = client.chat().create_stream(parameters).await.unwrap();

    while let Some(response) = stream.next().await {
        match response {
            Ok(chat_response) => chat_response.choices.iter().for_each(|choice| {
                if let Some(content) = choice.delta.content.as_ref() {
                    print!("{}", content);
                }
            }),
            Err(e) => eprintln!("{}", e),
        }
    }
}
```

More information: [Create chat completion](https://platform.openai.com/docs/api-reference/chat/create)

## Set API key

Add the PPLX API key to your environment variables.

```sh
# Windows PowerShell
$Env:PPLX_API_KEY='sk-...'

# Windows cmd
set PPLX_API_KEY=sk-...

# Linux/macOS
export PPLX_API_KEY='sk-...'
```

## Add proxy

This crate uses `reqwest` as HTTP Client. Reqwest has proxies enabled by default. You can set the proxy via the system environment variable or by overriding the default client.

### Example: set system environment variable

You can set the proxy in the system environment variables ([https://docs.rs/reqwest/latest/reqwest/#proxies](https://docs.rs/reqwest/latest/reqwest/#proxies)).

```sh
export HTTPS_PROXY=socks5://127.0.0.1:1086
```

### Example: overriding the default client

```rust
use pplx_client::v1::api::Client;

let http_client = reqwest::Client::builder()
    .proxy(reqwest::Proxy::https("socks5://127.0.0.1:1086")?)
    .build()?;

let api_key = std::env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

let client = Client {
    http_client,
    base_url: "https://api.perplexity.ai".to_string(),
    api_key,
};
```

## Use model names

[https://docs.perplexity.ai/docs/model-cards](https://docs.perplexity.ai/docs/model-cards)

```rust
use pplx_client::v1::models::PplxChatModel;

assert_eq!(PplxChatModel::Codellama34bInstruct.to_string(), "codellama-34b-instruct");
assert_eq!(PplxChatModel::Llama213bChat.to_string(), "llama-2-13b-chat");
assert_eq!(PplxChatModel::Llama270bChat.to_string(), "llama-2-70b-chat");
assert_eq!(PplxChatModel::Mistral7bInstruct.to_string(), "mistral-7b-instruct");
assert_eq!(PplxChatModel::Openhermes2Mistral7b.to_string(), "openhermes-2-mistral-7b");
assert_eq!(PplxChatModel::Openhermes25Mistral7b.to_string(), "openhermes-2.5-mistral-7b");
assert_eq!(PplxChatModel::Pplx7bChatAlpha.to_string(), "pplx-7b-chat-alpha");
assert_eq!(PplxChatModel::Pplx70bChatAlpha.to_string(), "pplx-70b-chat-alpha");
```

## Contributions

Contributions are welcome! Please open an issue or submit a pull request.

### Developer Environment

This is a stable rust 2021 project. You can install the latest stable rust toolchain with `rustup`:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This project also uses `Nix Flakes` to manage the development environment. You can install `Nix` with:

```sh
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

After installing `Nix`, you can enter the development environment with:

```sh
nix develop
```

Then build the project and run tests using `just` commands:

```bash
# Available just recipes:
    build *ARGS="--workspace --all-targets" # run `cargo build` on everything
    b *ARGS="--workspace --all-targets" # alias for `build`
    check *ARGS="--workspace --all-targets" # run `cargo check` on everything
    c *ARGS="--workspace --all-targets" # alias for `check`
    clippy *ARGS="--locked --offline --workspace --all-targets" # run `cargo clippy` on everything
    clippy-fix *ARGS="--locked --offline --workspace --all-targets" # run `cargo clippy --fix` on everything
    final-check          # run all checks recommended before opening a PR
    format               # run code formatters
    lint                 # run lints (git pre-commit hook)
    semgrep              # run `semgrep`
    test                 # run tests
    t                    # alias for `test`
    typos *PARAMS        # check typos
    typos-fix-all        # fix all typos
    watch *ARGS="-x run" # run and restart on changes
```
