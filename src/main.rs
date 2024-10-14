use alloy::{network::EthereumWallet, signers::local::PrivateKeySigner};
use alloy_chains::{Chain, NamedChain};
use provider::get_provider;
use std::error::Error;

pub mod addressbook;
pub mod bot;
pub mod cli;
pub mod config;
pub mod provider;
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
    cli::run_cli(provider).await?;

    Ok(())
}
