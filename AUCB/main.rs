use ethers::{
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::TransactionRequest,
};
use std::{env, sync::Arc, time::Duration};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Load environment variables
    let rpc_url = env::var("BSC_RPC_URL").expect("BSC_RPC_URL not set");
    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set");
    let recipient = env::var("RECIPIENT_ADDRESS").expect("RECIPIENT_ADDRESS not set");

    // Connect to Binance Smart Chain node
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet = private_key.parse::<LocalWallet>()?.with_chain_id(56u64); // BSC Mainnet Chain ID
    let client = Arc::new(SignerMiddleware::new(provider, wallet.clone()));

    loop {
        let address = wallet.address();
        println!("Wallet address: {address}");

        let balance = client.get_balance(address, None).await?;
        let balance_bnb = balance.as_u128() as f64 / 1e18;
        println!("Current balance: {:.6} BNB", balance_bnb);


        if balance >= U256::zero() {
            println!("Withdrawing...");

            let gas_price = client.get_gas_price().await?;
            let gas_limit = U256::from(21000); // Standard gas limit for simple transfers
            let gas_cost = gas_price * gas_limit;
            let gas_cont_bnb = gas_cost.as_u128() as f64 / 1e18;
            println!("Gas Fees : {gas_cont_bnb}");

            if balance <= gas_cost {
                println!("Insufficient balance for gas fees. Skipping withdrawal.");
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }

            let available_balance = balance.saturating_sub(gas_cost);
            let tx = TransactionRequest::new()
                .to(recipient.parse::<Address>()?)
                .value(available_balance) // Sending available balance after gas fees
                .gas_price(gas_price)
                .gas(gas_limit)
                .from(address);

            let pending_tx = client.send_transaction(tx, None).await?;
            let receipt = pending_tx.await?;
            if let Some(receipt) = receipt {
                println!("Withdrawal complete: {:?}", receipt.transaction_hash);
            } else {
                println!("Transaction not yet confirmed.");
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
