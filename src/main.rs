use aave::AaveLooper;
use alloy::{
    network::EthereumWallet, providers::WalletProvider, signers::local::PrivateKeySigner, sol,
    transports::BoxTransport,
};
use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{Address, U256};
use provider::{get_provider, SignerProvider};
use std::error::Error;
use std::str::FromStr;

pub mod aave;
pub mod cli;
pub mod config;
pub mod provider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let signer: PrivateKeySigner = std::env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set")
        .parse()
        .expect("should parse private key");

    let wallet = EthereumWallet::new(signer);
    let provider = get_provider(Chain::from_named(NamedChain::Arbitrum), wallet).await;
    cli::run_cli(provider).await?;

    Ok(())
}
