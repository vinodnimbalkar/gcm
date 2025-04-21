use crate::models::{OllamaRequest, OllamaResponse};
use crate::services::LlmService;
use reqwest::Client;
use std::env;

pub struct OllamaService {
    client: Client,
}

impl OllamaService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl LlmService for OllamaService {
    async fn generate(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let ollama_model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "gemma3".to_string());
        
        let response = match self.client
            .post("http://localhost:11434/api/generate")
            .json(&OllamaRequest {
                model: ollama_model.clone(),
                prompt: prompt.to_string(),
                stream: false,
            })
            .send()
            .await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        resp
                    } else {
                        if resp.status() == reqwest::StatusCode::NOT_FOUND {
                            eprintln!("Error: Model '{}' not found on the Ollama server", ollama_model);
                            eprintln!("To install {}: ollama pull {}", ollama_model, ollama_model);
                            eprintln!("Or set OLLAMA_MODEL environment variable to use a different model");
                        } else {
                            eprintln!("Error from Ollama server: Status {}", resp.status());
                        }
                        return Err(format!("Ollama server error: {}", resp.status()).into());
                    }
                }
                Err(e) => {
                    if e.is_connect() {
                        eprintln!("Error: Could not connect to Ollama server at http://localhost:11434");
                        eprintln!("Please make sure the Ollama server is running");
                    } else {
                        eprintln!("Error sending request to Ollama server: {}", e);
                    }
                    return Err(Box::new(e));
                }
            };

        let ollama_response: OllamaResponse = match response.json().await {
            Ok(parsed) => parsed,
            Err(e) => {
                eprintln!("Error parsing response from Ollama server: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(ollama_response.response)
    }
}