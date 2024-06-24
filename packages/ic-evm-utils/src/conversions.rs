use alloy::primitives::U256;
use candid::Nat;

/// this will panic if the value is too large to fit in a U256
pub fn nat_to_u256(n: &Nat) -> U256 {
    let be_bytes = n.0.to_bytes_be();
    U256::from_be_bytes(be_bytes.try_into().unwrap())
}
