use alloy::{
    network::{Network, TransactionBuilder as _},
    primitives::{b256, Address, B256},
    providers::Provider,
};

use crate::{error::DetectProxyResult, utils::u256_to_address};

const EIP_SAFE_INTERFACE: [B256; 1] = [
    // bytes4(keccak256("masterCopy()")) padded to 32 bytes
    b256!("0xa619486e00000000000000000000000000000000000000000000000000000000"),
];

pub(crate) async fn detect_safe_proxy<N, P: Provider<N>>(
    address: Address,
    provider: P,
) -> DetectProxyResult<Option<Address>>
where
    N: Network,
{
    let call_0 = <N as Network>::TransactionRequest::default()
        .with_to(address)
        .with_input(EIP_SAFE_INTERFACE[0]);

    if let Ok(value) = provider.call(&call_0).await {
        let b256: B256 = B256::from_slice(&value);
        return Ok(Some(u256_to_address(b256.into())));
    };

    Ok(None)
}
