pub mod ollama;
pub mod gemini;

// Trait for any LLM service
pub trait LlmService {
    async fn generate(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
}