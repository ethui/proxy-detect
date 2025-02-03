mod eip1167;
mod error;

use alloy::{network::Network, primitives::Address, providers::Provider};
use error::DetectProxyResult;

#[derive(Debug, PartialEq)]
pub enum ProxyType {
    Eip1167(Address),
}

pub async fn detect_proxy<N, P: Provider<N>>(
    address: Address,
    provider: &P,
) -> DetectProxyResult<Option<ProxyType>>
where
    N: Network,
{
    let code = provider.get_code_at(address).await?;

    if let Some(address) = eip1167::detect_eip1167_minimal_proxy(&code) {
        return Ok(Some(ProxyType::Eip1167(address)));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{primitives::address, providers::ProviderBuilder, transports::http::reqwest::Url};
    use lazy_static::lazy_static;

    const MAINNET_EIP1167_PROXY: Address = address!("0x6d5d9b6ec51c15f45bfa4c460502403351d5b999");
    const MAINNET_EIP1167_IMPL: Address = address!("0x210fF9Ced719E9bf2444DbC3670BAC99342126fA");
    lazy_static! {
        static ref MAINNET_RPC: Url = Url::parse(
            &std::env::var("ETH_MAINNET_RPC").unwrap_or("https://eth.rpc.blxrbdn.com".to_string())
        )
        .unwrap();
    }

    #[tokio::test]
    async fn mainnet_eip1167() {
        let provider = ProviderBuilder::new().on_http(MAINNET_RPC.clone());
        let result = detect_proxy(MAINNET_EIP1167_PROXY, &provider)
            .await
            .unwrap();

        assert_eq!(result, Some(ProxyType::Eip1167(MAINNET_EIP1167_IMPL)));
    }
}
