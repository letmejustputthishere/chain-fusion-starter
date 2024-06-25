use alloy::primitives::U256;
use candid::Nat;

/// this will panic if the value is too large to fit in a U256
pub fn nat_to_u256(n: &Nat) -> U256 {
    let be_bytes = n.0.to_bytes_be();
    U256::from_be_bytes(be_bytes.try_into().unwrap())
}

/// this will panic if the value is too large to fit in a u128
pub fn nat_to_u128(n: &Nat) -> u128 {
    n.0.try_into().unwrap()
}
