//! Contains the types for the Chainlist data objects.

use alloc::{string::String, vec::Vec};

/// The Chain object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chain {
    /// The name of the chain.
    pub name: String,
    /// The currency used on the chain.
    #[serde(rename = "chain")]
    pub currencu: String,
    /// The icon value.
    pub icon: Option<String>,
    /// The chain ID.
    pub chain_id: Option<u64>,
    /// A list of RPC URLs.
    pub rpc: Vec<ChainRpc>,
    /// Features of the chain.
    pub features: Option<Vec<Feature>>,
    /// Faucets for the chain.
    pub faucets: Option<Vec<String>>,
    /// The native currency of the chain.
    pub native_currency: Option<NativeCurrency>,
    /// The info url for the chain.
    pub info_url: Option<String>,
    /// The short name of the chain.
    pub short_name: Option<String>,
    /// The network ID of the chain.
    pub network_id: Option<u64>,
    /// The slip44 value of the chain.
    pub slip44: Option<u64>,
    /// Explorers for the chain.
    pub explorers: Option<Vec<Explorer>>,
    /// The chain's TVL.
    pub tvl: Option<f64>,
    /// The chain slug.
    pub chain_slug: Option<String>,
}

impl From<alloy_chains::NamedChain> for Chain {
    /// Converts an [`alloy_chains::NamedChain`] to a [`Chain`].
    ///
    /// # Panics
    ///
    /// Panics if the chain ID is not found in the rpc list from chainlist.
    fn from(named: alloy_chains::NamedChain) -> Self {
        crate::CHAINS
            .0
            .iter()
            .find(|chain| chain.chain_id == Some(named as u64))
            .cloned()
            .expect("Chain ID not found")
    }
}

/// The Explorer object.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Explorer {
    /// The name of the explorer.
    pub name: String,
    /// The URL of the explorer.
    pub url: String,
    /// The icon of the explorer.
    pub icon: Option<String>,
    /// The standard of the explorer.
    pub standard: Option<String>,
}

/// The native currency of the chain.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    /// The name of the currency.
    pub name: String,
    /// The symbol of the currency.
    pub symbol: String,
    /// The decimals of the currency.
    pub decimals: u8,
}

/// Features of the chain.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    /// The name of the feature.
    pub name: String,
}

/// The ChainRpc object.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ChainRpc {
    /// The RPC URL.
    pub url: String,
    /// If the RPC is being tracked.
    pub tracking: Option<Tracking>,
    /// Whether the RPC is open source.
    pub open_source: Option<bool>,
}

/// If the RPC is tracked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tracking {
    /// No tracking
    r#None,
    /// Tracked
    Yes,
    /// Limited
    Limited,
    /// Unspecified
    Unspecified,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_chain_rpc() {
        let json = r#"{"url":"https://rpc.example.com","tracking":"yes","open_source":true}"#;
        let chain_rpc: ChainRpc = serde_json::from_str(json).unwrap();
        assert_eq!(chain_rpc.url, "https://rpc.example.com");
        assert_eq!(chain_rpc.tracking, Some(Tracking::Yes));
        assert_eq!(chain_rpc.open_source, Some(true));

        let serialized = serde_json::to_string(&chain_rpc).unwrap();
        assert_eq!(serialized, json);
    }

    #[tokio::test]
    #[cfg(feature = "online")]
    async fn test_online_deserialize() {
        let url = "https://chainlist.org/rpcs.json";
        let response = reqwest::get(url).await.expect("Failed to fetch data");
        let chains: alloc::vec::Vec<Chain> = response.json().await.expect("Failed to parse JSON");
        assert!(!chains.is_empty(), "No chains found in the response");
    }
}
