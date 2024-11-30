
use anyhow::Result;
use ethers::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let private_key: &str = "0x7c1ea45d46fd1c06deb770309010472e41f8299338c651b0153031f6e2598ad6";

    let rpc_url = "https://bsc-dataseed.binance.org";

    // signer
    let signer: LocalWallet = private_key.parse()?;
    let signer = signer.with_chain_id(56u64);
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


    // gas price
    let gas_price = provider.get_gas_price().await?;
    let gas_price_in_ether = ethers::utils::format_units(gas_price, 9)?;
    println!("Gas Price : {gas_price_in_ether} GWEI");

    // amount
    let value_in_wei = ethers::utils::parse_units("0.00001", 18)?;
    // recipient
    let recipient: Address = "0x422B0755EABeA90Cc2C5674F8Bba65C861962fdD".parse()?;

    // transaction
    let tx = TransactionRequest::new()
        .to(recipient)
        .value(value_in_wei)
        .gas(21001)
        .gas_price(gas_price)
        .chain_id(56);

            // connect to proivder
    let client = SignerMiddleware::new(provider, signer);

    // send the transaction
    let pending_tx = client.send_transaction(tx, None).await?;
    
    // wait for the transaction to be mined
    let receipt = pending_tx.await?;

    // print the transaction details
    if let Some(receipt) = receipt {
        println!("Transaction hash: {:?}", receipt.transaction_hash);
    }

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
// bnb transfer bot using rust
// @autoboyt
