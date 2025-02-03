use alloy::{
    network::{Ethereum, Network},
    primitives::Address,
    providers::Provider,
};

pub enum ProxyType {
    Eip1167(Address),
}

pub async fn detect_proxy<N, P: Provider<N>>(address: Address, provider: &P) -> Option<ProxyType>
where
    N: Network,
{
    let code = provider.get_code_at(address).await;
    None

    //if let Some(address) = detect_eip1167_minimal_proxy(code){

    //todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::{primitives::address, providers::ProviderBuilder, transports::http::reqwest::Url};

    const MAINNET_USDC: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
    const MAINNET_RPC: &str = "https://eth.llamarpc.com";

    #[tokio::test]
    async fn mainnet_usdc() {
        //let url = Url::parse(MAINNET_RPC).unwrap();
        //let provider = ProviderBuilder::new().on_http(url);
        //let result = detect_proxy(MAINNET_USDC, &provider).await;
        //assert!(false);
    }
}
