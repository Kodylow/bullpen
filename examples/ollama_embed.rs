use bullpen::v1::api::Ollama;
use bullpen::v1::models::OllamaModel;
use bullpen::v1::resources::ollama::embedding::OllamaEmbeddingRequest;

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
