mod models;
mod services;
mod prompt;

use std::io::{self, Read};
use services::LlmService;
use services::ollama::OllamaService;

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

    // Initialize Ollama service
    let llm_service = OllamaService::new();
    
    // Generate commit message
    let commit_message = llm_service.generate(&prompt).await?;

    // Output the commit message to stdout
    println!("{}", commit_message.trim());

    Ok(())
}
