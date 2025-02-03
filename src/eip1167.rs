use alloy::{
    hex,
    primitives::{bytes, Address, Bytes},
};

const EIP1167_PREFIX: Bytes = bytes!("363d3d373d3d3d363d");
const EIP1167_SUFFIX: Bytes = bytes!("57fd5bf3");
const EIP1167_SUFFIX_OFFSET_FROM_ADDRESS_END: usize = 11;

pub(crate) fn detect_eip1167_minimal_proxy(code: &Bytes) -> Option<Address> {
    if !code.starts_with(&EIP1167_PREFIX) {
        return None;
    }

    // detect length of address (20 bytes non-optimized, 0 < N < 20 bytes for vanity addresses)
    // push1 ... push20 use opcode 0x60 ... 0x73
    let address_len = code[EIP1167_PREFIX.len()] as usize - 0x5f;

    if !(1..=20).contains(&address_len) {
        return None;
    }

    let address_pos = EIP1167_PREFIX.len() + 1;
    let suffix = &code[address_pos + address_len + EIP1167_SUFFIX_OFFSET_FROM_ADDRESS_END..];

    dbg!(&hex::encode(suffix));
    if !suffix.starts_with(&EIP1167_SUFFIX) {
        return None;
    }

    let address_hex = &code[address_pos..address_pos + address_len];
    let address = Address::from_slice(address_hex);

    Some(address)
}

#[cfg(test)]
mod tests {
    use alloy::primitives::address;

    use super::*;

    #[test]
    fn parse_eip1167_code() {
        let bytecode: Bytes = bytes!("363d3d373d3d3d363d73f62849f9a0b5bf2913b396098f7c7019b51a820a5af43d82803e903d91602b57fd5bf3000000000000000000000000000000000000000000000000000000000000007a6900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        let address: Address = address!("f62849f9a0b5bf2913b396098f7c7019b51a820a");

        assert_eq!(detect_eip1167_minimal_proxy(&bytecode), Some(address));
    }
}
