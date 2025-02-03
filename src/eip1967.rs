use alloy::{
    network::Network,
    primitives::{b256, Address, Bytes, B256, U256},
    providers::Provider,
};

use crate::{error::DetectProxyResult, ProxyType};

// bytes32(uint256(keccak256('eip1967.proxy.implementation')) - 1)
const EIP1967_LOGIC_SLOT: B256 =
    b256!("0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc");
const EIP1967_BEACON_SLOT: B256 =
    b256!("0xa3f0ad74e5423aebfd80d3ef4346578335a9a72aeaee59ff6cb3582b35133d50");

pub(crate) async fn detect_eip1967_direct_proxy<N, P: Provider<N>>(
    address: Address,
    provider: P,
) -> DetectProxyResult<Option<ProxyType>>
where
    N: Network,
{
    if let Ok(Some(addr)) = storage_slot_as_address(&provider, address, EIP1967_LOGIC_SLOT).await {
        return Ok(Some(ProxyType::Eip1967Direct(addr)));
    }

    if let Ok(Some(addr)) = storage_slot_as_address(&provider, address, EIP1967_BEACON_SLOT).await {
        return Ok(Some(ProxyType::Eip1967Beacon(addr)));
    }

    Ok(None)
}

async fn storage_slot_as_address<N, P: Provider<N>>(
    provider: P,
    address: Address,
    slot: B256,
) -> DetectProxyResult<Option<Address>>
where
    N: Network,
{
    let slot = provider
        .get_storage_at(address, slot.into())
        .latest()
        .await?;

    if !slot.is_zero() {
        return Ok(Some(u256_to_address(slot)));
    }

    Ok(None)
}

fn u256_to_address(u256: U256) -> Address {
    let bytes: Bytes = u256.to_be_bytes::<32>().into();
    Address::from_slice(&bytes[12..])
}
