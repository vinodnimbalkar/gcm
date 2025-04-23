mod models;
mod prompt;
mod services;

use services::LlmService;
use services::gemini::GeminiService;
use services::ollama::OllamaService;
use std::io::{self, Read};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read git diff from stdin
    let mut diff = String::new();
    io::stdin().read_to_string(&mut diff)?;

    if diff.trim().is_empty() {
        eprintln!("No git diff provided on stdin");
        return Ok(());
    }

    // Create prompt
    let prompt = prompt::create_commit_message_prompt(&diff);

    // Check if Gemini API key is set if set call gemini else ollama
    let gemini_api_key = std::env::var("GEMINI_API_KEY").ok();
    if gemini_api_key.is_none() {
        // Initialize Ollama service
        let llm_service = OllamaService::new();

        // Generate commit message
        let commit_message = llm_service.generate(&prompt).await?;

        // Output the commit message to stdout
        println!("{}", commit_message.trim());
    } else {
        // Initialize Gemini service
        let gemini_service = GeminiService::new();

        // Generate commit message
        let commit_message = gemini_service.generate(&prompt).await?;
        
        // Output the commit message to stdout
        println!("{}", commit_message.trim());
    }

    Ok(())
}
