use reqwest::Client;
use dotenvy::dotenv;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("GROQ_API_KEY")?;

    let client = Client::new();

    let response = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "llama-3.3-70b-versatile",
            "messages": [
                {
                    "role": "user",
                    "content": "Explain Rust ownership in one paragraph."
                }
            ]
        }))
        .send()
        .await?;

    let body: serde_json::Value = response.json().await?;

    println!(
        "{}",
        body["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response")
    );

    Ok(())
}
