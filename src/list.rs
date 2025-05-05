//! List of chains.

use crate::Chain;
use alloc::vec::Vec;

/// The list of chains.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChainList(pub Vec<Chain>);

impl ChainList {
    /// Loads the [`ChainList`] from the static JSON file.
    pub fn from_json() -> Self {
        let file = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/static/chainlist.json"
        ));
        serde_json::from_str(file).expect("Failed to parse JSON file")
    }

    /// Loads the [`ChainList`] from the online Chainlist API.
    #[cfg(feature = "online")]
    pub async fn from_api() -> Result<Self, reqwest::Error> {
        let url = "https://chainlist.org/rpcs.json";
        let response = reqwest::get(url).await?;
        let chains: Vec<Chain> = response.json().await?;
        Ok(ChainList(chains))
    }
}

impl core::ops::Deref for ChainList {
    type Target = Vec<Chain>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[cfg(feature = "online")]
    async fn test_online_chain_list_from_api() {
        let chains = ChainList::from_api().await.expect("Failed to fetch data");
        assert!(!chains.0.is_empty(), "No chains found in the response");
    }

    #[test]
    fn test_chains_sepolia() {
        let chains = ChainList::from_json();
        let known_chain = chains
            .iter()
            .find(|chain| chain.chain_id == Some(11155111))
            .unwrap();
        let known_rpcs = alloc::vec![
            "https://sepolia.gateway.tenderly.co",
            "https://gateway.tenderly.co/public/sepolia",
            "https://eth-sepolia.public.blastapi.io",
            "https://sepolia.drpc.org",
        ];
        for rpc in known_rpcs {
            assert!(
                known_chain.rpc.iter().any(|chain_rpc| chain_rpc.url == rpc),
                "RPC {} not found in the Sepolia chain",
                rpc
            );
        }
    }
}
