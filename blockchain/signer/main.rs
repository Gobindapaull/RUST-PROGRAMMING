
use anyhow::Result;
use ethers::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let private_key: &str = "0xcff790139bb57fc898a102cddfd22dd37fb1402717e3daed202294325dac4994";
    let rpc_url = "https://bsc-dataseed.binance.org";

    // signer
    let signer: LocalWallet = private_key.parse()?;
    println!("Wallet address: {:?}", signer.address());

    // Initialize the provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // block number
    let block_number = provider.get_block_number().await;
    println!("Latest block number: {:?}", block_number);

    // chain ID
    let chain_id = provider.get_chainid().await;
    println!("Chain ID : {:?}", chain_id);

    Ok(())
}

// [package]
// name = "auto_bot_rust"
// version = "0.1.0"
// edition = "2021"

// [dependencies]
// anyhow = "1.0"
// ethers = "2.0.14"
// tokio = { version = "1", features = ["full"] }

// cargo build
// cargo run
