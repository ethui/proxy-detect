use alloy::{
    network::{Network, TransactionBuilder as _},
    primitives::{b256, Address, B256},
    providers::Provider,
};

use crate::{error::DetectProxyResult, utils::u256_to_address, ProxyType};

const SAFE_INTERFACE: [B256; 1] = [
    // bytes4(keccak256("masterCopy()")) padded to 32 bytes
    b256!("a619486e00000000000000000000000000000000000000000000000000000000"),
];

pub(crate) async fn detect_safe_proxy<N, P: Provider<N>>(
    address: Address,
    provider: P,
) -> DetectProxyResult<Option<ProxyType>>
where
    N: Network,
{
    let call_0 = <N as Network>::TransactionRequest::default()
        .with_to(address)
        .with_input(SAFE_INTERFACE[0]);

    if let Ok(value) = provider.call(&call_0).await {
        let b256: B256 = B256::from_slice(&value);
        return Ok(Some(ProxyType::Safe(u256_to_address(b256.into()))));
    };

    Ok(None)
}
