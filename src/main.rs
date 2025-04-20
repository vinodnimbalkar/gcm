use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use std::env;
use tokio;

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read git diff from stdin
    let mut diff = String::new();
    io::stdin().read_to_string(&mut diff)?;

    if diff.trim().is_empty() {
        eprintln!("No git diff provided on stdin");
        return Ok(());
    }

    // Construct prompt for the LLM
    let prompt = format!(
        "Generate a Git commit message with its description and an appropriate emoji based on the provided Git diff.

        **Rules:**
        - Respond with only the commit message and description.
        - No explanations, no markdown formatting, no code block (```) wrappers.
        - Follow this format:

        `<type>: <emoji><message>`  
        `<detailed description>`

        **Example output:**

        feat: 🎉Initial commit  
        Initial commit description

        **Commit types and corresponding emojis:**

        | Type                        | Emoji             |
        |-----------------------------|-------------------|
        | Initial commit              | 🎉 `:tada:`        |
        | Version tag                 | 🔖 `:bookmark:`    |
        | New feature                 | ✨ `:sparkles:`     |
        | Bugfix                      | 🐛 `:bug:`         |
        | Metadata                    | 📇 `:card_index:`  |
        | Documentation               | 📚 `:books:`       |
        | Documenting source code     | 💡 `:bulb:`        |
        | Performance                 | 🐎 `:racehorse:`   |
        | Cosmetic                    | 💄 `:lipstick:`    |
        | Tests                       | 🚨 `:rotating_light:` |
        | Adding a test               | ✅ `:white_check_mark:` |
        | Make a test pass            | ✔️ `:heavy_check_mark:` |
        | General update              | ⚡ `:zap:`          |
        | Improve format/structure    | 🎨 `:art:`         |
        | Refactor code               | 🔨 `:hammer:`      |
        | Removing code/files         | 🔥 `:fire:`        |
        | Continuous Integration      | 💚 `:green_heart:` |
        | Security                    | 🔒 `:lock:`        |
        | Upgrading dependencies      | ⬆️ `:arrow_up:`     |
        | Downgrading dependencies    | ⬇️ `:arrow_down:`   |
        | Lint                        | 👕 `:shirt:`       |
        | Translation                 | 👽 `:alien:`       |
        | Text                        | ✏️ `:pencil:`       |
        | Critical hotfix             | 🚑 `:ambulance:`   |
        | Deploying stuff             | 🚀 `:rocket:`      |
        | Fixing on MacOS             | 🍎 `:apple:`       |
        | Fixing on Linux             | 🐧 `:penguin:`     |
        | Fixing on Windows           | 🏁 `:checkered_flag:` |
        | Work in progress            | 🚧 `:construction:` |
        | Adding CI build system      | 👷 `:construction_worker:` |
        | Analytics or tracking code  | 📈 `:chart_with_upwards_trend:` |
        | Removing a dependency       | ➖ `:heavy_minus_sign:` |
        | Adding a dependency         | ➕ `:heavy_plus_sign:` |
        | Docker                      | 🐳 `:whale:`        |
        | Configuration files         | 🔧 `:wrench:`       |
        | Package.json in JS          | 📦 `:package:`      |
        | Merging branches            | 🔀 `:twisted_rightwards_arrows:` |
        | Bad code / needs improvement| 💩 `:hankey:`       |
        | Reverting changes           | ⏪ `:rewind:`       |
        | Breaking changes            | 💥 `:boom:`         |
        | Code review changes         | 👌 `:ok_hand:`      |
        | Accessibility               | ♿ `:wheelchair:`   |
        | Move/rename repository      | 🚚 `:truck:`        |

        Now respond with the commit message for the following Git diff:\n\n{}",
            diff
        );

    let commit_message = generate_with_ollama(&prompt).await?;

    // Output the commit message to stdout
    println!("{}", commit_message.trim());

    Ok(())
}

async fn generate_with_ollama(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let ollama_model = env::var("OLLAMA_MODEL").unwrap_or_else(|_| "gemma3".to_string());
    
    let response = match client
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
