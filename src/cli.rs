use crate::aave::AaveLooper;
use crate::provider::SignerProvider;
use alloy_primitives::{Address, U256};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Enter a position on Aave
    EnterPosition {
        /// Amount to supply (in USDC)
        #[arg(short, long)]
        amount: u64,

        /// Leverage factor
        #[arg(short, long, default_value_t = 2)]
        leverage: u8,
    },
    /// Start running the Aave bot
    RunBot {
        /// Amount to supply (in USDC)
        #[arg(short, long)]
        amount: u64,

        /// Leverage factor
        #[arg(short, long, default_value_t = 2)]
        leverage: u8,

        /// Threshold for available liquidity (in USDC)
        #[arg(short, long, default_value_t = 100)]
        threshold: u64,
    },
}

pub async fn run_cli(provider: Arc<SignerProvider>) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::EnterPosition { amount, leverage } => {
            let aave_address = Address::from_str("0x794a61358D6845594F94dc1DB02A252b5b4814aD")?;
            let asset_address = Address::from_str("0xaf88d065e77c8cC2239327C5EDb3A432268e5831")?;
            let amount_wei = U256::from(*amount) * U256::from(10).pow(U256::from(6)); // Convert to USDC wei
            let threshold = U256::from(0); // Set threshold to 0 for immediate execution

            let looper = AaveLooper::new(
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
            let aave_address = Address::from_str("0x794a61358D6845594F94dc1DB02A252b5b4814aD")?;
            let asset_address = Address::from_str("0xaf88d065e77c8cC2239327C5EDb3A432268e5831")?;
            let amount_wei = U256::from(*amount) * U256::from(10).pow(U256::from(6)); // Convert to USDC wei
            let threshold_wei = U256::from(*threshold) * U256::from(10).pow(U256::from(6)); // Convert to USDC wei

            let telegram_token =
                std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
            let chat_id = std::env::var("CHAT_ID")
                .expect("CHAT_ID must be set")
                .parse()
                .expect("CHAT_ID should be a valid integer");

            let bot = AaveLooper::new(
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
    }

    Ok(())
}
