use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{
        Address, U256,
        utils::{format_ether, parse_ether},
    },
    providers::{Provider, ProviderBuilder},
    rpc::types::request::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(name = "task4")]
#[command(about = "Transfer ETH from address A to address B on Arbitrum testnet")]
struct Args {
    #[arg(short, long)]
    recipient: String,

    #[arg(short, long)]
    amount: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let args = Args::parse();

    let private_key_str =
        std::env::var("PRIVATE_KEY").context("PRIVATE_KEY environment variable not set")?;

    let recipient_address =
        Address::from_str(&args.recipient).context("Invalid recipient address")?;

    let amount_eth = parse_ether(&args.amount)?;

    let rpc_url = "https://arbitrum-sepolia-rpc.publicnode.com".parse()?;
    println!("Connecting to Arbitrum Sepolia RPC: {}", rpc_url);

    let signer: PrivateKeySigner = private_key_str.parse().context("Invalid private key")?;
    let from_address = signer.address();

    let wallet = EthereumWallet::from(signer);

    let provider = ProviderBuilder::new().wallet(wallet).connect_http(rpc_url);

    println!("From address: {}", from_address);
    println!("To address: {}", recipient_address);
    println!("Amount: {} ETH", args.amount);

    let balance = provider.get_balance(from_address).await?;
    let balance_eth = format_ether(balance);
    println!("Sender balance: {} ETH", balance_eth);

    let gas_price = provider.get_gas_price().await?;
    println!("Gas price: {} wei", gas_price);

    let tx_request = TransactionRequest::default()
        .with_from(from_address)
        .with_to(recipient_address)
        .with_value(amount_eth);

    let gas_limit = provider.estimate_gas(tx_request.clone()).await?;
    let gas_price = gas_price * 110 / 100;
    let gas_cost = U256::from(gas_limit) * U256::from(gas_price);

    println!("Estimated gas limit: {}", gas_limit);
    println!(
        "Estimated gas cost: {} ETH, with 10% buffer",
        format_ether(gas_cost)
    );

    if balance < amount_eth + gas_cost {
        anyhow::bail!(
            "Insufficient balance: needed {} ETH, have {} ETH",
            format_ether(amount_eth + gas_cost),
            balance_eth
        );
    }

    let tx_request = tx_request
        .with_gas_limit(gas_limit)
        .with_gas_price(gas_price);

    println!("Sending transaction...");
    let tx = provider.send_transaction(tx_request).await?;

    println!("Transaction sent! Hash: {}", tx.tx_hash());

    let receipt = provider
        .get_transaction_receipt(*tx.tx_hash())
        .await?
        .context("Transaction receipt not found")?;
    println!("Transaction confirmed in block: {:?}", receipt.block_number);

    Ok(())
}
