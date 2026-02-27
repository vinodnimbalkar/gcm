use crate::models::{GeminiRequest, GeminiResponse, Contents, Part};
use crate::services::LlmService;
use reqwest::Client;
use std::env;

pub struct GeminiService {
    client: Client,
}

impl GeminiService {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl LlmService for GeminiService {
    async fn generate(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let gemini_api_key = env::var("GEMINI_API_KEY").unwrap_or_else(|_| {
            eprintln!("GEMINI_API_KEY environment variable not set");
            std::process::exit(1);
        });

        let gemini_request = GeminiRequest {
            contents: vec![Contents {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
        };

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:generateContent?key={}", gemini_api_key);

        let response = match self
            .client
            .post(url)
            .json(&gemini_request)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    resp
                } else {
                    eprintln!("Error from Gemini server: Status {}", resp.status());
                    return Err(format!("Gemini server error: {}", resp.status()).into());
                }
            }
            Err(e) => {
                eprintln!("Error sending request to Gemini server: {}", e);
                return Err(Box::new(e));
            }
        };

        let gemini_response: GeminiResponse = match response.json().await {
            Ok(parsed) => parsed,
            Err(e) => {
                eprintln!("Error parsing response from Gemini server: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(gemini_response.candidates[0].content.parts[0].text.clone())
    }
}
