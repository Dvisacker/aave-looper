use crate::addressbook::{get_aave_lending_pool_address, get_token_address};
use crate::bot::AaveBot;
use crate::provider::SignerProvider;
use alloy::providers::Provider;
use alloy_chains::Chain;
use alloy_primitives::{Address, U256};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
    Repay {
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long)]
        token: String,
    },
    Leverage {
        #[arg(short, long)]
        amount: u64,
        #[arg(short, long)]
        supply_asset: String,
        #[arg(short, long)]
        borrow_asset: String,
        #[arg(short, long, default_value_t = 2)]
        leverage: u8,
    },
    Deleverage {
        #[arg(short, long)]
        supply_asset: String,
        #[arg(short, long)]
        borrow_asset: String,
    },
    Withdraw {
        #[arg(short, long)]
        asset: String,
    },
    Monitor,
    Snipe {
        #[arg(long)]
        supply_asset: String,
        #[arg(long)]
        borrow_asset: String,
        #[arg(long)]
        supply_threshold: u64,
        #[arg(long)]
        borrow_threshold: u64,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        iterations: u8,
    },
}

pub async fn run_cli(
    provider: Arc<SignerProvider>,
    monitor_interval: Duration,
) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let id = provider.get_chain_id().await?;
    let chain = Chain::from_id(id);
    let looper_address: Address = "0x5119C3d14c892D710311bE3f102619df669BD62C".parse()?;

    match &cli.command {
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
                looper_address,
                asset_address,
                amount_wei,
                telegram_token,
                chat_id,
                monitor_interval,
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
                looper_address,
                asset_address,
                amount_wei,
                String::new(),
                0,
                monitor_interval,
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
                looper_address,
                asset_address,
                U256::from(0), // Threshold not used for borrow
                String::new(),
                0,
                monitor_interval,
            )
            .await?;

            println!("Borrowing {} {} from Aave...", amount, token);
            bot.borrow_tokens(asset_address, amount_wei).await?;
            println!("Borrow successful!");
        }
        Commands::Repay { amount, token } => {
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
                looper_address,
                asset_address,
                U256::from(0), // Threshold not used for repay
                String::new(),
                0,
                monitor_interval,
            )
            .await?;

            bot.approve_tokens(asset_address, aave_address, amount_wei)
                .await?;

            println!("Repaying {} {} to Aave...", amount, token);
            bot.repay_tokens(asset_address, amount_wei).await?;
            println!("Repay successful!");
        }
        Commands::Leverage {
            amount,
            supply_asset,
            borrow_asset,
            leverage,
        } => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let supply_asset_address =
                get_token_address(chain, &supply_asset).ok_or_else(|| {
                    Box::<dyn Error>::from(format!(
                        "{} address not found for this chain",
                        supply_asset
                    ))
                })?;
            let borrow_asset_address =
                get_token_address(chain, &borrow_asset).ok_or_else(|| {
                    Box::<dyn Error>::from(format!(
                        "{} address not found for this chain",
                        borrow_asset
                    ))
                })?;
            let amount_wei = U256::from(*amount) * U256::from(10).pow(U256::from(4)); // Assuming 6 decimals, adjust if needed

            let bot = AaveBot::new(
                provider.clone(),
                aave_address,
                looper_address,
                supply_asset_address,
                U256::from(0), // Threshold not used for this operation
                String::new(),
                0,
                monitor_interval,
            )
            .await?;

            println!(
                "Increasing leverage by borrowing {} {} and supplying it back...",
                amount, borrow_asset
            );
            bot.leverage(
                supply_asset_address,
                borrow_asset_address,
                amount_wei,
                *leverage,
            )
            .await?;
            println!("Leverage increased successfully!");
        }
        Commands::Deleverage {
            supply_asset,
            borrow_asset,
        } => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let supply_asset_address = get_token_address(chain, supply_asset).ok_or_else(|| {
                Box::<dyn Error>::from(format!("{} address not found for this chain", supply_asset))
            })?;
            let borrow_asset_address = get_token_address(chain, borrow_asset).ok_or_else(|| {
                Box::<dyn Error>::from(format!("{} address not found for this chain", borrow_asset))
            })?;

            let bot = AaveBot::new(
                provider.clone(),
                aave_address,
                looper_address,
                supply_asset_address,
                U256::ZERO, // Not used for deleverage
                String::new(),
                0,
                monitor_interval,
            )
            .await?;

            println!("Deleveraging position...");
            bot.deleverage(supply_asset_address, borrow_asset_address)
                .await?;
        }
        Commands::Withdraw { asset } => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let asset_address = get_token_address(chain, asset).ok_or_else(|| {
                Box::<dyn Error>::from(format!("{} address not found for this chain", asset))
            })?;

            let bot = AaveBot::new(
                provider.clone(),
                aave_address,
                looper_address,
                asset_address,
                U256::ZERO, // Not used for withdraw
                String::new(),
                0,
                monitor_interval,
            )
            .await?;

            println!("Withdrawing {} from AaveLooper...", asset);
            bot.withdraw(asset_address).await?;
            println!("Withdrawal successful!");
        }
        Commands::Monitor => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let asset_address = get_token_address(chain, "usdc")
                .ok_or_else(|| Box::<dyn Error>::from("USDC address not found for this chain"))?;

            let telegram_token =
                std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
            let chat_id = std::env::var("CHAT_ID")
                .expect("CHAT_ID must be set")
                .parse()
                .expect("CHAT_ID should be a valid integer");

            let bot = AaveBot::new(
                provider,
                aave_address,
                looper_address,
                asset_address,
                U256::ZERO, // Not used for monitoring
                telegram_token,
                chat_id,
                monitor_interval,
            )
            .await?;

            println!("Starting monitoring...");
            bot.monitor().await?;
            println!("Monitoring complete.");
        }
        Commands::Snipe {
            supply_asset,
            borrow_asset,
            supply_threshold,
            borrow_threshold,
            amount,
            iterations,
        } => {
            let aave_address = get_aave_lending_pool_address(chain).ok_or_else(|| {
                Box::<dyn Error>::from("Aave lending pool address not found for this chain")
            })?;
            let supply_asset_address = get_token_address(chain, supply_asset).ok_or_else(|| {
                Box::<dyn Error>::from(format!("{} address not found for this chain", supply_asset))
            })?;
            let borrow_asset_address = get_token_address(chain, borrow_asset).ok_or_else(|| {
                Box::<dyn Error>::from(format!("{} address not found for this chain", borrow_asset))
            })?;

            let telegram_token =
                std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
            let chat_id = std::env::var("CHAT_ID")
                .expect("CHAT_ID must be set")
                .parse()
                .expect("CHAT_ID should be a valid integer");

            let bot = AaveBot::new(
                provider,
                aave_address,
                looper_address,
                supply_asset_address,
                U256::ZERO, // Not used for sniping
                telegram_token,
                chat_id,
                monitor_interval,
            )
            .await?;

            println!("Starting sniping...");
            bot.snipe(
                supply_asset_address,
                borrow_asset_address,
                U256::from(*supply_threshold),
                U256::from(*borrow_threshold),
                U256::from(*amount),
                *iterations,
            )
            .await?;
            println!("Sniping complete.");
        }
    }

    Ok(())
}
