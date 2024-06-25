use candid::Nat;
use ethers_core::types::U256;
use num_traits::ToPrimitive;

/// this will panic if the value is too large to fit in a U256
pub fn nat_to_u256(n: &Nat) -> U256 {
    let be_bytes = n.0.to_bytes_be();
    U256::from_big_endian(&be_bytes)
}

/// this will panic if the value is too large to fit in a u128
pub fn nat_to_u128(n: &Nat) -> u128 {
    n.0.to_u128().unwrap()
}
