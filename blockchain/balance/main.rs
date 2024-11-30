
use anyhow::Result;
use ethers::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let private_key: &str = "0x68619a0fb3e7fe39565a302e0a30118109eeb1fc65e12d62770e319e03e61628";
    let rpc_url = "https://bsc-dataseed.binance.org";

    // signer
    let signer: LocalWallet = private_key.parse()?;
    println!("Wallet address: {:?}", signer.address());

    // Initialize the provider
    let provider = Provider::<Http>::try_from(rpc_url)?;

    // block number
    let block_number = provider.get_block_number().await?;
    println!("Latest block number: {block_number}");

    // chain ID
    let chain_id = provider.get_chainid().await?;
    println!("Chain ID : {chain_id}");

    // balance
    let balance = provider.get_balance(signer.address(), None).await?;
    let balance_in_ether = ethers::utils::format_units(balance, 18)?;
    println!("Wallet balance : {balance_in_ether} BNB");

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
