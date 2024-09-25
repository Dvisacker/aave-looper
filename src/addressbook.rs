use alloy_chains::{Chain, NamedChain};
use alloy_primitives::Address;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainAddresses {
    pub aave_lending_pool: String,
    pub tokens: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAddresses {
    pub arbitrum: ChainAddresses,
    pub mainnet: ChainAddresses,
}

lazy_static! {
    static ref CONTRACT_ADDRESSES: ContractAddresses = load_contract_addresses();
}

fn load_contract_addresses() -> ContractAddresses {
    let mut file =
        File::open("./src/addressbook.json").expect("Unable to open contract addresses file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read contract addresses file");
    serde_json::from_str(&contents).expect("Unable to parse contract addresses JSON")
}

pub fn get_aave_lending_pool_address(chain: Chain) -> Option<Address> {
    let chain = chain.named()?;
    match chain {
        NamedChain::Arbitrum => {
            Some(Address::from_str(&CONTRACT_ADDRESSES.arbitrum.aave_lending_pool).unwrap())
        }
        NamedChain::Mainnet => {
            Some(Address::from_str(&CONTRACT_ADDRESSES.mainnet.aave_lending_pool).unwrap())
        }
        _ => None,
    }
}

pub fn get_token_address(chain: Chain, token: &str) -> Option<Address> {
    let chain = chain.named()?;
    let token_addresses = match chain {
        NamedChain::Arbitrum => &CONTRACT_ADDRESSES.arbitrum.tokens,
        NamedChain::Mainnet => &CONTRACT_ADDRESSES.mainnet.tokens,
        _ => return None,
    };

    token_addresses
        .get(&token.to_uppercase())
        .map(|address_str| Address::from_str(address_str).unwrap())
}

pub fn get_available_tokens(chain: Chain) -> Vec<String> {
    let chain = chain.named().unwrap();
    match chain {
        NamedChain::Arbitrum => CONTRACT_ADDRESSES.arbitrum.tokens.keys().cloned().collect(),
        NamedChain::Mainnet => CONTRACT_ADDRESSES.mainnet.tokens.keys().cloned().collect(),
        _ => vec![],
    }
}
