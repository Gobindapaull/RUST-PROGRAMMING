use dotenvy::dotenv;
use std::env;
use ethers::prelude::*;
use tokio;
use ethers::providers::{Provider, Http};
use ethers::utils::format_units;
use std::convert::TryFrom;

#[tokio::main]
async fn main() -> eyre::Result<()>{
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("RPC URL not set");
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let gas_price = provider.get_gas_price().await?;
    let gas_price_bnb = format_units(gas_price, "gwei")?;
    println!("Gas Price: {} GWEI", gas_price_bnb);

    Ok(())
}
