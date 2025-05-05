#![doc = include_str!("../README.md")]
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
        let mainnet = CHAINS
            .iter()
            .find(|chain| chain.chain_id == Some(1))
            .unwrap();
        assert_eq!(mainnet.name, "Ethereum Mainnet");
    }
}
