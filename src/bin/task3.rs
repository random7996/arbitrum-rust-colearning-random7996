use alloy::{
    primitives::utils::format_ether,
    providers::{Provider, ProviderBuilder},
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    println!("Try to connect to Arbitrum Sepolia RPC: {}", rpc_url);
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let limit = 21000u128;
    let price = provider.get_gas_price().await?;
    println!("Gas price: {} wei", price);
    println!("Gas limit: {}", limit);
    let cost = limit * price;
    println!("Estimated transaction cost: {} wei", cost);
    println!("Estimated transaction cost: {} eth", format_ether(cost));

    Ok(())
}
