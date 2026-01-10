use alloy::{
    primitives::{address, utils::format_ether},
    providers::{Provider, ProviderBuilder},
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    println!("Try to connect to Arbitrum Sepolia RPC: {}", rpc_url);
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let address = address!("4f3934D2760dE4EA4e4E36D5f537bF3c9B668E01");
    let balance = provider.get_balance(address).await?;
    let eth = format_ether(balance);

    println!("Balance of address {}: {} ETH", address, eth);

    Ok(())
}
