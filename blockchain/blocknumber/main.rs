use anyhow::Result;
use ethers::providers::{Http, Middleware, Provider};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://bsc-dataseed.binance.org";

    // Initialize the provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    let block_number = provider.get_block_number().await;
    println!("Latest block number: {:?}", block_number);

    Ok(())
}

// [package]
// name = "auto_bot_rust"
// version = "0.1.0"
// edition = "2021"

// [dependencies]
// anyhow = "1.0"
// ethers = { version = "1.0" }
// tokio = { version = "1", features = ["full"] }

// cargo build
// cargo run
