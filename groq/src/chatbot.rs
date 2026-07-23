use dotenvy::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    env,
    io::{self, Write},
};

const API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const MODEL: &str = "llama-3.3-70b-versatile";

#[derive(Debug, Serialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
struct AssistantMessage {
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_key = env::var("GROQ_API_KEY")?;
    let client = Client::new();

    let mut messages = vec![Message {
        role: "system".to_string(),
        content: "You are a helpful AI assistant.".to_string(),
    }];

    println!("==============================");
    println!("      Groq Rust Chatbot");
    println!("Type 'exit' to quit.");
    println!("==============================");

    loop {
        print!("\nYou: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        messages.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        let response = client
            .post(API_URL)
            .bearer_auth(&api_key)
            .json(&serde_json::json!({
                "model": MODEL,
                "messages": messages,
                "temperature": 0.7,
                "max_tokens": 512
            }))
            .send()
            .await?
            .error_for_status()?;

        let chat: ChatResponse = response.json().await?;

        let reply = &chat.choices[0].message.content;

        println!("\nAI: {}\n", reply);

        messages.push(Message {
            role: "assistant".to_string(),
            content: reply.clone(),
        });
    }

    Ok(())
}
