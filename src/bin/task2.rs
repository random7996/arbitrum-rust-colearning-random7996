use alloy::{
    primitives::{Address, utils::format_ether},
    providers::{Provider, ProviderBuilder},
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    println!("Try to connect to Arbitrum Sepolia RPC: {}", rpc_url);
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let mut args = std::env::args();
    args.next(); // ignore filepath
    let address = args.next().expect("Please input your EVM address.");
    let address = Address::parse_checksummed(address, None)?;

    let balance = provider.get_balance(address).await?;
    let eth = format_ether(balance);

    println!("Balance of address {}: {} ETH", address, eth);

    Ok(())
}
