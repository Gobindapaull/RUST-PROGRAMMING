use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let account_pubkey= "CeA3sPZfWWToFEBmw5n1Y93tnV66Vmp8LacLzsVprgxZ";
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAccountInfo",
        "params": [account_pubkey]
    });

    let response = client
        .post("https://api.mainnet-beta.solana.com")
        .json(&request_body)
        .send()
        .await?;

    let json: Value = response.json().await?;

    if let Some(account_info) = json["result"].as_object() {
        println!("Result : {:?}", account_info);
    } else {
        println!("Failed to parse slot result: {:?}", json);
    }

    Ok(())
}
