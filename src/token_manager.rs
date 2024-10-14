use crate::provider::SignerProvider;
use alloy::providers::Provider;
use alloy::sol;
use alloy::transports::BoxTransport;
use alloy_primitives::Address;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};
use IERC20::IERC20Instance;

sol! {
    #[derive(Debug, PartialEq, Eq)]
    #[sol(rpc)]
    contract IERC20 {
        function decimals() external view returns (uint8);
        function approve(address spender, uint256 amount) external returns (bool);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address recipient, uint256 amount) external returns (bool);
        function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
    }
}

type Token = IERC20Instance<BoxTransport, Arc<SignerProvider>>;

pub struct TokenManager {
    tokens: Mutex<HashMap<Address, Token>>,
    provider: Arc<SignerProvider>,
}

impl TokenManager {
    pub fn new(provider: Arc<SignerProvider>) -> Self {
        TokenManager {
            tokens: Mutex::new(HashMap::new()),
            provider,
        }
    }

    pub fn get(&self, address: Address) -> Result<Token, Box<dyn Error>> {
        let mut tokens = self
            .tokens
            .lock()
            .map_err(|e| format!("Failed to acquire lock: {}", e))?;

        if let Some(token) = tokens.get(&address) {
            Ok(token.clone())
        } else {
            let new_token = IERC20::new(address, self.provider.clone());
            tokens.insert(address, new_token.clone());
            Ok(new_token)
        }
    }
}
