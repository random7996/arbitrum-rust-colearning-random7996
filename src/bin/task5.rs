use alloy::{primitives::address, providers::ProviderBuilder, sol};
use anyhow::Result;

sol!(
    #[sol(rpc)]
    IERC20,
    "src/abis/STOKEN.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    println!("Try to connect to Arbitrum Sepolia RPC: {}", rpc_url);
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let erc20 = IERC20::new(
        address!("0x3b340504e73235e0910F16fb31ea1E6FceC9cCF4"),
        &provider,
    );

    println!("contract: {}", erc20.address());

    let name = erc20.name().call().await?;
    println!("name: {}", name);
    let symbol = erc20.symbol().call().await?;
    println!("symbol: {}", symbol);
    let minter = erc20.minter().call().await?;
    println!("minter: {}", minter);

    Ok(())
}
