//! Macro to construct the Rpc Configuration given the chain id.

/// The `rpc!` macro is used to instantiate a `Chain` RPC configuration
// from the given chain id. If the chain id is not found, it will panic.
#[macro_export]
macro_rules! rpc {
    ($chain_id:expr) => {{
        $crate::CHAINS
            .iter()
            .find(|chain| chain.chain_id == Some($chain_id))
            .expect("Chain ID not found")
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rpc_macro() {
        let chain = rpc!(1);
        assert_eq!(chain.name, "Ethereum Mainnet");
    }
}
