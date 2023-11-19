# Bullpen

Bullpen is an unofficial async Rust library that allows you to interact with the Pplx API and Modelfarm APIs.

You can use the `Modelfarm` client without any api keys, but you will need to provide an api key for the `Pplx` client.

I'll be adding bitcoin and ecash payments as a payment option for the Pplx client soon, then you won't need an api key.

For Developers and Contributors: [Contributions](#contributions)

Forked from OpenAI Dive: https://github.com/tjardoo/pplx-client

## Modelfarm Example

```rust
#[tokio::main]
async fn main() {
    let modelfarm = Modelfarm::new();

    let chat_session = ChatSession {
        context: "You are a programmer bot".to_string(),
        examples: vec![ChatExample {
            input: ChatMessage {
                content: "1 + 1".to_string(),
                author: "user".to_string(),
            },
            output: ChatMessage {
                content: "2".to_string(),
                author: "assistant".to_string(),
            },
        }],
        messages: vec![ChatMessage {
            content: "How do I write a nix flake for a rust project?".to_string(),
            author: "user".to_string(),
        }],
    };

    let req = ModelfarmChatCompletionRequest {
        model: ModelfarmChatModel::ChatBison,
        parameters: ModelfarmChatParameters {
            prompts: vec![chat_session],
            temperature: 0.2,
            max_output_tokens: 1024,
        },
    };

    let result = modelfarm.chat(req).await.unwrap();

    println!("{:?}", result);
}
```

## Pplx Example

```rust
#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("PPLX_API_KEY").expect("$PPLX_API_KEY is not set");

    let client = Pplx::new(api_key);

    let parameters = PplxChatCompletionParameters {
        model: PplxChatModel::Mistral7bInstruct,
        messages: vec![
            ChatMessage {
                role: Role::User,
                content: "Hello!".to_string(),
                ..Default::default()
            },
            ChatMessage {
                role: Role::User,
                content: "Tell me a story?".to_string(),
                ..Default::default()
            },
        ],
        temperature: None,
        top_p: None,
        top_k: None,
        max_tokens: Some(1000),
        presence_penalty: None,
        frequency_penalty: None,
    };

    let mut stream = client.stream_chat(parameters).await.unwrap();

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
