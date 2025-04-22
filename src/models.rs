use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct OllamaRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Deserialize, Debug)]
pub struct OllamaResponse {
    pub response: String,
}

// Gemini API request and response structures
#[derive(Serialize, Debug)]
pub struct GeminiRequest {
    pub contents: Vec<Contents>,
}

#[derive(Serialize, Debug)]
pub struct Contents {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
}

