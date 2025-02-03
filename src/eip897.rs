use alloy::{
    network::{Network, TransactionBuilder as _},
    primitives::{b256, Address, B256},
    providers::Provider,
};

use crate::{error::DetectProxyResult, utils::u256_to_address};

const EIP_897_INTERFACE: [B256; 2] = [
    // bytes4(keccak256("implementation()")) padded to 32 bytes
    b256!("0x5c60da1b00000000000000000000000000000000000000000000000000000000"),
    // bytes4(keccak256("proxyType()")) padded to 32 bytes
    b256!("0x4555d5c900000000000000000000000000000000000000000000000000000000"),
];

pub(crate) async fn detect_eip897_proxy<N, P: Provider<N>>(
    address: Address,
    provider: P,
) -> DetectProxyResult<Option<Address>>
where
    N: Network,
{
    let call_0 = <N as Network>::TransactionRequest::default()
        .with_to(address)
        .with_input(EIP_897_INTERFACE[0]);

    if let Ok(value) = provider.call(&call_0).await {
        let b256: B256 = B256::from_slice(&value);
        return Ok(Some(u256_to_address(b256.into())));
    };

    Ok(None)
}
