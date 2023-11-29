use bullpen::api::Ollama;
use bullpen::models::OllamaModel;
use bullpen::resources::ollama::embedding::OllamaEmbeddingRequest;

#[tokio::main]
async fn main() {
    let ollama = Ollama::new();

    let req = OllamaEmbeddingRequest {
        model: OllamaModel::Llama2,
        prompt: "How do I write a nix flake for a rust project?".to_string(),
    };

    let res = ollama.embed(req).await.unwrap();

    println!("{:?}", res);
}
