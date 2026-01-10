use alloy::providers::{Provider, ProviderBuilder};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    println!("Try to connect to Arbitrum Sepolia RPC: {}", rpc_url);
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let block_number = provider.get_block_number().await?;
    println!("Latest block number: {}", block_number);

    Ok(())
}
