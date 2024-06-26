//! This module provides functions for converting between different numeric types.
use candid::Nat;
use ethers_core::types::U256;
use num_traits::ToPrimitive;

/// Converts a `Nat` to a `U256`.
///
/// # Arguments
///
/// * `n` - The `Nat` to convert.
///
/// # Returns
///
/// The `U256` representation of the `Nat`.
///
/// # Panics
///
/// This function will panic if the value is too large to fit in a `U256`.
pub fn nat_to_u256(n: &Nat) -> U256 {
    let be_bytes = n.0.to_bytes_be();
    U256::from_big_endian(&be_bytes)
}

/// Converts a `Nat` to a `u128`.
///
/// # Arguments
///
/// * `n` - The `Nat` to convert.
///
/// # Returns
///
/// The `u128` representation of the `Nat`.
///
/// # Panics
///
/// This function will panic if the value is too large to fit in a `u128`.
pub fn nat_to_u128(n: &Nat) -> u128 {
    n.0.to_u128().unwrap()
}
