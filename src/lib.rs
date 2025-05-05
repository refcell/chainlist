//! # chainlist
//!
//! `no_std` bindings to access [Chainlist.org](https://chainlist.org/) RPC config data, programmatically.
//!
//! This library provides a simple interface to access the RPC configurations available on [Chainlist.org](https://chainlist.org/).
//!
//! ## Usage
//!
//! ```rust
//! use chainlist::{CHAINS, Chain, rpc};
//!
//! // Get the RPC configuration for Ethereum Mainnet.
//! let mainnet: &Chain = CHAINS.iter().find(|chain| chain.chain_id == Some(1)).unwrap();
//! assert_eq!(mainnet.name, "Ethereum Mainnet");
//!
//! // Use the `rpc!` macro to get the RPC config for Ethereum Mainnet.
//! let mainnet_rpc = rpc!(1);
//!
//! // Get the `Chain` RPC configuration from an alloy "NamedChain".
//! // Note, this will panic if an RPC configuration doesn't exist
//! // in the chain list for the given chain id.
//! let mainnet: Chain = alloy_chains::NamedChain::Mainnet.into();
//! assert_eq!(mainnet.chain_id, Some(alloy_chains::NamedChain::Mainnet as u64));
//! ```
//!
//! ## Features
//!
//! - `std`: Enables the use of the standard library.
//! - `online`: Provides a way to fetch the latest RPC configurations from Chainlist.org.
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/refcell/chainlist/main/assets/square.png",
    html_favicon_url = "https://raw.githubusercontent.com/refcell/chainlist/main/assets/favicon.ico",
    issue_tracker_base_url = "https://github.com/refcell/chainlist/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod macros;

mod chain;
pub use chain::{Chain, ChainRpc, Tracking};

mod list;
pub use list::ChainList;

lazy_static::lazy_static! {
    /// Chain RPC Configurations loaded from a static JSON file.
    pub static ref CHAINS: ChainList = ChainList::from_json();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chains() {
        let mainnet = CHAINS.iter().find(|chain| chain.chain_id == Some(1)).unwrap();
        assert_eq!(mainnet.name, "Ethereum Mainnet");
    }
}
