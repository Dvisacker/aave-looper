use crate::provider::SignerProvider;
use alloy::sol;
use alloy::{providers::WalletProvider, transports::BoxTransport};
use alloy_primitives::aliases::U24;
use alloy_primitives::{Address, U256};
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;
use AaveLooper::AaveLooperInstance;
use ILendingPool::ILendingPoolInstance;
use IERC20::IERC20Instance;

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
        function decimals() external view returns (uint8);
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
    }
}

sol!(
    #[sol(rpc)]
    #[derive(Debug, PartialEq, Eq)]
    AaveLooper,
    "./contracts/out/AaveLooper.sol/AaveLooper.json"
);

// type Token = IERC20Instance<BoxTransport, Arc<SignerProvider>>;
type LendingPool = ILendingPoolInstance<BoxTransport, Arc<SignerProvider>>;
type Looper = AaveLooperInstance<BoxTransport, Arc<SignerProvider>>;

pub struct AaveBot {
    provider: Arc<SignerProvider>,
    signer_address: Address,
    asset_address: Address,
    lending_pool: LendingPool,
    looper_address: Address,
    looper: Looper,
    max_amount: U256,
    // telegram_bot: Bot,
    // chat_id: i64,
}

impl AaveBot {
    pub async fn new(
        provider: Arc<SignerProvider>,
        aave_address: Address,
        looper_address: Address,
        asset_address: Address,
        max_amount: U256,
        telegram_token: String,
        chat_id: i64,
    ) -> Result<Arc<Self>, Box<dyn Error>> {
        let lending_pool = ILendingPool::new(aave_address, provider.clone());
        let signer_address = provider.default_signer_address();

        // Initialize the AaveLooper contract
        let looper = AaveLooper::new(looper_address, provider.clone());

        Ok(Arc::new(Self {
            provider,
            signer_address,
            lending_pool,
            looper,
            looper_address,
            asset_address,
            max_amount,
            // telegram_bot,
            // chat_id,
        }))
    }

    pub async fn run(self: Arc<Self>) -> Result<(), Box<dyn Error>> {
        loop {
            self.monitor_and_act().await?;
            time::sleep(Duration::from_secs(600)).await; // Wait for 10 minutes before the next iteration
        }
    }

    // https://docs.aave.com/developers/core-contracts/pool#getconfiguration
    pub async fn get_caps(&self, token_address: Address) -> Result<(U256, U256), Box<dyn Error>> {
        let config = self
            .lending_pool
            .getConfiguration(token_address)
            .call()
            .await?;
        let data = config._0.data;
        let supply_cap = (data >> 116) & U256::from((1u128 << 36) - 1);
        let borrow_cap = (data >> 80) & U256::from((1u128 << 36) - 1);
        Ok((supply_cap, borrow_cap))
    }

    pub async fn monitor_and_act(&self) -> Result<(), Box<dyn Error>> {
        let reserve_data = self
            .lending_pool
            .getReserveData(self.asset_address)
            .call()
            .await?;
        let a_token_address = reserve_data._0.aTokenAddress;

        let caps = self.get_caps(self.asset_address).await?;

        println!("Supply cap: {}", caps.0);
        println!("Borrow cap: {}", caps.1);
        println!("A token address: {}", a_token_address);

        // let a_token = IERC20::IERC20Instance::new(a_token_address, self.provider.clone());
        // let IERC20::balanceOfReturn {
        //     _0: total_liquidity,
        // } = a_token.balanceOf(a_token_address).call().await?;
        // let IERC20::balanceOfReturn { _0: asset_balance } =
        //     self.asset.balanceOf(a_token_address).call().await?;

        // let available_liquidity_wei = total_liquidity - asset_balance;
        // let available_liquidity = available_liquidity_wei / U256::from(10).pow(U256::from(18));

        // println!("Available liquidity in pool: {}", available_liquidity);

        // if available_liquidity > self.threshold {
        //     self.enter_position().await?;
        //     self.send_telegram_message(format!(
        //         "Entered position. Available liquidity: {}",
        //         available_liquidity
        //     ))
        //     .await?;
        // } else {
        //     self.send_telegram_message(format!(
        //         "Available liquidity ({}) below threshold. No action taken.",
        //         available_liquidity
        //     ))
        //     .await?;
        // }
        // let result = self
        //     .aave
        //     .getUserAccountData(self.signer_address)
        //     .call()
        //     .await?;

        // let available_borrows = result.availableBorrowsBase;

        // println!("Available borrows: {}", available_borrows);

        // if available_borrows > self.threshold {
        //     self.enter_position().await?;
        //     self.send_telegram_message(format!(
        //         "Entered position. Available borrows: {}",
        //         available_borrows
        //     ))
        //     .await?;
        // } else {
        //     self.send_telegram_message(format!(
        //         "Available borrows ({}) below threshold. No action taken.",
        //         available_borrows
        //     ))
        //     .await?;
        // }

        Ok(())
    }

    pub async fn approve_tokens(
        &self,
        asset_address: Address,
        spender_address: Address,
        amount: U256,
    ) -> Result<(), Box<dyn Error>> {
        let token = IERC20::new(asset_address, self.provider.clone());
        let tx = token.approve(spender_address, amount);
        let receipt = tx.send().await?.get_receipt().await?;
        println!("Approved AAVE to spend tokens: {:?}", receipt);
        Ok(())
    }

    pub async fn supply_tokens(
        &self,
        token_address: Address,
        amount: U256,
    ) -> Result<(), Box<dyn Error>> {
        let tx = self
            .lending_pool
            .supply_0(token_address, amount, self.signer_address, 0);
        let receipt = tx.send().await?.get_receipt().await?;
        println!("Supplied assets to AAVE: {:?}", receipt);
        Ok(())
    }

    pub async fn borrow_tokens(
        &self,
        token_address: Address,
        amount: U256,
    ) -> Result<(), Box<dyn Error>> {
        let tx = self.lending_pool.borrow_0(
            token_address,
            amount,
            U256::from(2),
            0,
            self.signer_address,
        );
        let receipt = tx.send().await?.get_receipt().await?;
        println!("Borrowed assets from AAVE: {:?}", receipt);
        Ok(())
    }

    pub async fn repay_tokens(
        &self,
        token_address: Address,
        amount: U256,
    ) -> Result<(), Box<dyn Error>> {
        let tx =
            self.lending_pool
                .repay_1(token_address, amount, U256::from(2), self.signer_address);
        let receipt = tx.send().await?.get_receipt().await?;
        println!("Repaid assets to AAVE: {:?}", receipt);
        Ok(())
    }

    async fn send_telegram_message(&self, _message: String) -> Result<(), Box<dyn Error>> {
        // self.telegram_bot
        //     .send_message(ChatId(self.chat_id), message)
        //     .await?;
        Ok(())
    }

    pub async fn leverage(
        &self,
        supply_asset: Address,
        borrow_asset: Address,
        amount: U256,
        iterations: u8,
    ) -> Result<(), Box<dyn Error>> {
        self.approve_tokens(supply_asset, self.looper_address, amount)
            .await?;

        let tx = self.looper.leverage(
            supply_asset,
            borrow_asset,
            amount,
            U256::from(iterations),
            U24::from(500), // (0.3% fee tier for Uniswap V3, adjust if needed)
        );

        let receipt = tx.send().await?.get_receipt().await?;
        println!("Increased leverage: {:?}", receipt);

        Ok(())
    }

    pub async fn deleverage(
        &self,
        supply_asset: Address,
        borrow_asset: Address,
    ) -> Result<(), Box<dyn Error>> {
        let tx =
            self.looper
                .exitPosition(supply_asset, borrow_asset, U256::from(10), U24::from(500));

        let receipt = tx.send().await?.get_receipt().await?;
        println!("Exit position: {:?}", receipt);
        Ok(())
    }

    pub async fn withdraw(&self, asset_address: Address) -> Result<(), Box<dyn Error>> {
        let tx = self.looper._withdrawToOwner(asset_address);
        let receipt = tx.send().await?.get_receipt().await?;
        println!("Withdrawn from AaveLooper: {:?}", receipt);
        Ok(())
    }
}
