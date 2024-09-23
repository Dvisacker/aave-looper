use alloy_chains::{Chain, NamedChain};

pub struct ChainConfig {
    pub chain: Chain,
    pub chain_id: u64,
    pub explorer_url: String,
    pub explorer_api_key: String,
    pub explorer_api_url: String,
}

pub async fn get_config(chain: Chain) -> ChainConfig {
    let chain_id = chain.id();
    let chain = NamedChain::try_from(chain_id);

    match chain {
        Ok(NamedChain::Mainnet) => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Mainnet),
                chain_id: 1,
                explorer_url: "https://etherscan.io".to_string(),
                explorer_api_key: "TCZS3DYFANPFZRPFY338CCKHTMF5QNMCG9".to_string(),
                explorer_api_url: "https://api.etherscan.io/api".to_string(),
            };
        }
        Ok(NamedChain::Arbitrum) => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Arbitrum),
                chain_id: 42161,
                explorer_url: "https://arbiscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api.arbiscan.io/api".to_string(),
            };
        }
        Ok(NamedChain::Optimism) => {
            return ChainConfig {
                chain: Chain::from_named(NamedChain::Optimism),
                chain_id: 10,
                explorer_url: "https://optimistic.etherscan.io".to_string(),
                explorer_api_key: "".to_string(),
                explorer_api_url: "https://api-optimistic.etherscan.io/api".to_string(),
            };
        }
        _ => panic!("Chain not supported"),
    }
}
