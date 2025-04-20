use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let account_pubkey = "CeA3sPZfWWToFEBmw5n1Y93tnV66Vmp8LacLzsVprgxZ";
    
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBalance",
        "params": [account_pubkey]
    });

    let response = client
        .post("https://api.mainnet-beta.solana.com")
        .json(&request_body)
        .send()
        .await?;

    let json: Value = response.json().await?;

    if let Some(balance_info) = json["result"].as_object() {
        let lamports = balance_info["value"].as_u64().unwrap_or(0);
        let sol = lamports as f64 / 1_000_000_000.0;
        println!("Balance Info: \n{:#?}", sol);
    } else {
        println!("Failed to parse Balance result: {:?}", json);
    }

    Ok(())
}
