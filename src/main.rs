use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
use alloy_chains::{Chain, NamedChain};
use provider::get_provider;
use std::error::Error;
use std::time::Duration;

pub mod addressbook;
pub mod bot;
pub mod cli;
pub mod config;
pub mod provider;
pub mod telegram;
pub mod token_manager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let signer: PrivateKeySigner = std::env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set")
        .parse()
        .expect("should parse private key");

    let wallet = EthereumWallet::new(signer);
    let provider = get_provider(Chain::from_named(NamedChain::Arbitrum), wallet).await;

    // Read monitor interval from environment variable, default to 10 minutes if not set
    let monitor_interval = std::env::var("MONITOR_INTERVAL_SECONDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(600));

    cli::run_cli(provider, monitor_interval).await?;

    Ok(())
}
