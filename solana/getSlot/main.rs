use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSlot"
    });

    let response = client
        .post("https://api.mainnet-beta.solana.com")
        .json(&request_body)
        .send()
        .await?;

    let json: Value = response.json().await?;

    if let Some(current_slot) = json["result"].as_u64() {
        println!("Current Slot: {}", current_slot);
    } else {
        println!("Failed to parse slot result: {:?}", json);
    }

    Ok(())
}
