use crate::addressbook::{get_aave_lending_pool_address, get_token_address};
use crate::bot::AaveBot;
use crate::provider::SignerProvider;
use alloy::providers::Provider;
use alloy_chains::Chain;
use alloy_primitives::{Address, U256};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::sync::Arc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    EnterPosition {
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long, default_value_t = 1)]
        leverage: u8,
        #[arg(short, long, default_value = "USDC")]
        token: String,
    },
    RunBot {
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long, default_value_t = 2)]
        leverage: u8,
        #[arg(short, long, default_value_t = 100)]
        threshold: u64,
    },
    Supply {
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long)]
        token: String,
    },
    Borrow {
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long)]
        token: String,
    },
}

pub async fn run_cli(provider: Arc<SignerProvider>) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let id = provider.get_chain_id().await?;
    let chain = Chain::from_id(id);

    match &cli.command {
        Commands::EnterPosition {
            amount,
            leverage,
            token,
        } => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let asset_address = get_token_address(chain, &token).ok_or_else(|| {
                Box::<dyn Error>::from(format!("{} address not found for this chain", token))
            })?;
            let amount_wei = U256::from(*amount) * U256::from(10).pow(U256::from(6)); // Convert to USDC wei
            let threshold = U256::from(0); // Set threshold to 0 for immediate execution

            let looper = AaveBot::new(
                provider,
                aave_address,
                asset_address,
                amount_wei,
                *leverage,
                threshold,
                String::new(), // Empty string for Telegram token (not used in this context)
                0,             // 0 for chat_id (not used in this context)
            )
            .await?;

            println!("Entering position on Aave...");
            looper.enter_position().await?;
            println!("Position entered successfully!");
        }
        Commands::RunBot {
            amount,
            leverage,
            threshold,
        } => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let asset_address = get_token_address(chain, "usdc")
                .ok_or_else(|| Box::<dyn Error>::from("USDC address not found for this chain"))?;
            let amount_wei = U256::from(*amount) * U256::from(10).pow(U256::from(6)); // Convert to USDC wei
            let threshold_wei = U256::from(*threshold) * U256::from(10).pow(U256::from(6)); // Convert to USDC wei

            let telegram_token =
                std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
            let chat_id = std::env::var("CHAT_ID")
                .expect("CHAT_ID must be set")
                .parse()
                .expect("CHAT_ID should be a valid integer");

            let bot = AaveBot::new(
                provider,
                aave_address,
                asset_address,
                amount_wei,
                *leverage,
                threshold_wei,
                telegram_token,
                chat_id,
            )
            .await?;

            println!("Starting Aave bot...");
            bot.run().await?;
        }
        Commands::Supply { amount, token } => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let asset_address = get_token_address(chain, token).ok_or_else(|| {
                Box::<dyn Error>::from(format!("{} address not found for this chain", token))
            })?;
            let amount_wei = U256::from(*amount) * U256::from(10).pow(U256::from(6)); // Assuming 6 decimals, adjust if needed

            let bot = AaveBot::new(
                provider.clone(),
                aave_address,
                asset_address,
                amount_wei,
                1,             // Leverage not used for supply
                U256::from(0), // Threshold not used for supply
                String::new(),
                0,
            )
            .await?;

            println!("Supplying {} {} to Aave...", amount, token);
            bot.supply_tokens(asset_address, amount_wei).await?;
            println!("Supply successful!");
        }
        Commands::Borrow { amount, token } => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let asset_address = get_token_address(chain, token).ok_or_else(|| {
                Box::<dyn Error>::from(format!("{} address not found for this chain", token))
            })?;
            let amount_wei = U256::from(*amount) * U256::from(10).pow(U256::from(6)); // Assuming 6 decimals, adjust if needed

            let bot = AaveBot::new(
                provider.clone(),
                aave_address,
                asset_address,
                amount_wei,
                1,             // Leverage not used for borrow
                U256::from(0), // Threshold not used for borrow
                String::new(),
                0,
            )
            .await?;

            println!("Borrowing {} {} from Aave...", amount, token);
            bot.borrow_tokens(asset_address, amount_wei).await?;
            println!("Borrow successful!");
        }
    }

    Ok(())
}
