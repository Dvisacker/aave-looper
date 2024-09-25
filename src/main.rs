use alloy::{
    network::EthereumWallet, providers::WalletProvider, signers::local::PrivateKeySigner, sol,
    transports::BoxTransport,
};
use alloy_chains::{Chain, NamedChain};
use alloy_primitives::{Address, U256};
use provider::{get_provider, SignerProvider};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use teloxide::prelude::*;
use tokio::time;
use ILendingPool::ILendingPoolInstance;
use IERC20::IERC20Instance;

pub mod config;
pub mod provider;

sol!(
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    ILendingPool,
    "./ILendingPool.json"
);

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IERC20 {
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
    }
}

struct AaveLooper {
    provider: Arc<SignerProvider>,
    signer_address: Address,
    asset_address: Address,
    aave: ILendingPoolInstance<BoxTransport, Arc<SignerProvider>>,
    asset: IERC20Instance<BoxTransport, Arc<SignerProvider>>,
    amount: U256,
    leverage: u8,
    threshold: U256,
    // telegram_bot: Bot,
    // chat_id: i64,
}

impl AaveLooper {
    async fn new(
        provider: Arc<SignerProvider>,
        aave_address: Address,
        asset_address: Address,
        amount: U256,
        leverage: u8,
        threshold: U256,
        telegram_token: String,
        chat_id: i64,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let aave = ILendingPool::new(aave_address, provider.clone());
        let asset = IERC20::new(asset_address, provider.clone());
        let signer_address = provider.default_signer_address();
        // let telegram_bot = Bot::new(telegram_token);

        Ok(Arc::new(Self {
            provider,
            signer_address,
            aave,
            asset,
            asset_address,
            amount,
            leverage,
            threshold,
            // telegram_bot,
            // chat_id,
        }))
    }

    async fn run(self: Arc<Self>) -> Result<(), Box<dyn Error>> {
        loop {
            self.monitor_and_act().await?;
            time::sleep(Duration::from_secs(600)).await; // Wait for 10 minutes before the next iteration
        }
    }

    async fn monitor_and_act(&self) -> Result<(), Box<dyn Error>> {
        let reserve_data = self.aave.getReserveData(self.asset_address).call().await?;
        println!("Reserve data: {:?}", reserve_data);
        let a_token_address = reserve_data._0.aTokenAddress;

        let a_token = IERC20::IERC20Instance::new(a_token_address, self.provider.clone());
        let IERC20::balanceOfReturn {
            _0: total_liquidity,
        } = a_token.balanceOf(a_token_address).call().await?;
        let IERC20::balanceOfReturn { _0: asset_balance } =
            self.asset.balanceOf(a_token_address).call().await?;

        let available_liquidity_wei = total_liquidity - asset_balance;
        let available_liquidity = available_liquidity_wei / U256::from(10).pow(U256::from(18));

        println!("Available liquidity in pool: {}", available_liquidity);

        if available_liquidity > self.threshold {
            self.enter_position().await?;
            self.send_telegram_message(format!(
                "Entered position. Available liquidity: {}",
                available_liquidity
            ))
            .await?;
        } else {
            self.send_telegram_message(format!(
                "Available liquidity ({}) below threshold. No action taken.",
                available_liquidity
            ))
            .await?;
        }
        let result = self
            .aave
            .getUserAccountData(self.signer_address)
            .call()
            .await?;

        let available_borrows = result.availableBorrowsBase;

        println!("Available borrows: {}", available_borrows);

        if available_borrows > self.threshold {
            self.enter_position().await?;
            self.send_telegram_message(format!(
                "Entered position. Available borrows: {}",
                available_borrows
            ))
            .await?;
        } else {
            self.send_telegram_message(format!(
                "Available borrows ({}) below threshold. No action taken.",
                available_borrows
            ))
            .await?;
        }

        Ok(())
    }

    async fn enter_position(&self) -> Result<(), Box<dyn Error>> {
        // Approve AAVE to spend our tokens
        let tx = self.asset.approve(*self.aave.address(), self.amount);
        let receipt = tx.send().await?.get_receipt().await?;
        println!("Approved AAVE to spend tokens: {:?}", receipt);

        // Supply assets to AAVE
        // let tx = self
        //     .aave
        //     .supply(self.asset_address, self.amount, self.signer_address, 0);
        // let receipt = tx.send().await?.get_receipt().await?;
        // println!("Supplied assets to AAVE: {:?}", receipt);

        // // Calculate borrow amount based on leverage
        // let borrow_amount = self.amount * U256::from(self.leverage - 1) / U256::from(self.leverage);

        // // Borrow assets from AAVE
        // let tx = self.aave.borrow(
        //     self.asset_address,
        //     U256::from(borrow_amount),
        //     U256::from(2),
        //     0,
        //     self.signer_address,
        // );
        // let receipt = tx.send().await?.get_receipt().await?;
        // println!("Borrowed assets from AAVE: {:?}", receipt);

        Ok(())
    }

    async fn send_telegram_message(&self, message: String) -> Result<(), Box<dyn Error>> {
        // self.telegram_bot
        //     .send_message(ChatId(self.chat_id), message)
        //     .await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let signer: PrivateKeySigner = std::env::var("PRIVATE_KEY")
        .expect("PRIVATE_KEY must be set")
        .parse()
        .expect("should parse private key");
    let telegram_token =
        std::env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let chat_id = std::env::var("CHAT_ID")
        .expect("CHAT_ID must be set")
        .parse()
        .expect("CHAT_ID should be a valid integer");

    let wallet = EthereumWallet::new(signer);
    let provider = get_provider(Chain::from_named(NamedChain::Arbitrum), wallet).await;

    let aave_address = Address::from_str("0x794a61358D6845594F94dc1DB02A252b5b4814aD")?; // AAVE v3 lending pool address
    let asset_address = Address::from_str("0x82aF49447D8a07e3bd95BD0d56f35241523fBab1")?; // USDC address
    let amount = U256::from(1000) * U256::from(10).pow(U256::from(6)); // 1000 USDC
    let leverage = 2; // 2x leverage
    let threshold = U256::from(100) * U256::from(10).pow(U256::from(6)); // 100 USDC threshold

    let bot = AaveLooper::new(
        provider,
        aave_address,
        asset_address,
        amount,
        leverage,
        threshold,
        telegram_token,
        chat_id,
    )
    .await?;
    bot.run().await?;

    Ok(())
}
