use ethers::prelude::*;
use std::convert::TryFrom;
use std::time::Duration;
use tokio::time::sleep;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to an Ethereum node
    let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth")?;

    println!("Fetching Ethereum block numbers...");

    loop {
        // Fetch the latest block number
        match provider.get_block_number().await {
            Ok(block_number) => println!("Latest Ethereum block number: {}", block_number),
            Err(e) => eprintln!("Error fetching block number: {:?}", e),
        }

        // Wait for 10 seconds before checking again
        sleep(Duration::from_secs(10)).await;
    }
}
