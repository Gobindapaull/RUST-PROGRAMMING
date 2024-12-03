use ethers::prelude::*;
use std::convert::TryFrom;
use std::time::Duration;
use tokio::time::sleep;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to an Ethereum node (Infura in this case)
    let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth")?;

    // Uniswap V2 Router Contract Address
    let uniswap_router_address: Address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse()?;

    println!("Monitoring Ethereum blocks for Uniswap router interactions...");

    loop {
        // Fetch the latest block
        let latest_block = provider.get_block_with_txs(BlockId::Number(BlockNumber::Latest)).await?;

        if let Some(block) = latest_block {
            println!("Block Number: {}", block.number.unwrap());

            // Iterate over transactions in the block
            for tx in block.transactions {
                if let Some(to) = tx.to {
                    if to == uniswap_router_address {
                        println!("Uniswap Router Interaction Found!");
                        println!("Transaction Hash: {:?}", tx.hash);
                        println!("From: {:?}", tx.from);
                        println!("To: {:?}", to);
                        println!("Value: {:?}", tx.value);
                        println!("===================================")
                    }
                }
            }
        }

        // Wait for 10 seconds before checking again
        sleep(Duration::from_secs(10)).await;
    }
}

// any transactions interacting with the Uniswap router.
// @autoboyt

