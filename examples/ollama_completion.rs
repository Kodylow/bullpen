use bullpen::api::Ollama;
use bullpen::models::OllamaModel;
use bullpen::resources::ollama::completion::OllamaCompletionRequest;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let ollama = Ollama::new();

    let req =
        OllamaCompletionRequest {
            model: OllamaModel::Llama2,
            prompt: "How do I write a nix flake for a rust project?".to_string(),
            max_tokens: Some(100),
            temperature: Some(0.2),
            ..Default::default()
        };

    let mut stream = ollama.create_stream(req).await.unwrap();

    while let Some(response) = stream.next().await {
        print!("{}", response.unwrap().response);
    }
}
