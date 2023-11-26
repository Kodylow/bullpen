use bullpen::v1::api::Ollama;
use bullpen::v1::models::OllamaModel;
use bullpen::v1::resources::ollama::completion::OllamaCompletionRequest;

#[tokio::main]
async fn main() {
    let ollama = Ollama::new();

    let req = OllamaCompletionRequest {
        model: OllamaModel::Llama2,
        prompt: "How do I write a nix flake for a rust project?".to_string(),
        max_tokens: Some(100),
        temperature: Some(0.2),
        top_p: None,
        top_k: None,
        presence_penalty: None,
        frequency_penalty: None,
    };

    let result = ollama.create(req).await.unwrap();

    println!("{:?}", result);
}
