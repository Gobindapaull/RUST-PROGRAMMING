use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // First, get the current slot (latest block)
    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSlot",
        "params": []
    });

    let response = client
        .post("https://api.mainnet-beta.solana.com")
        .json(&request_body)
        .send()
        .await?;

    let json: Value = response.json().await?;

    if let Some(current_slot) = json["result"].as_u64() {
        println!("Latest Slot (Block): {}", current_slot);

        // Now use the slot to fetch the block information, including maxSupportedTransactionVersion
        let block_request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlock",
            "params": [
                current_slot,
                {
                    "encoding": "jsonParsed",
                    "transactionDetails": "full",
                    "rewards": false,
                    "maxSupportedTransactionVersion": 0
                }
            ]
        });

        let block_response = client
            .post("https://api.mainnet-beta.solana.com")
            .json(&block_request_body)
            .send()
            .await?;

        let block_json: Value = block_response.json().await?;

        if let Some(block_info) = block_json["result"].as_object() {
            println!("Latest Block Information: \n{:#?}", block_info);
        } else {
            println!("Failed to parse block result: {:?}", block_json);
        }
    } else {
        println!("Failed to fetch the current slot: {:?}", json);
    }

    Ok(())
}
