//! This module provides functions for signing EIP-1559 transactions using t-ECDSA, getting the public key of the canister, and converting the public key to an Ethereum address.
use candid::Principal;
use ethers_core::abi::ethereum_types::{Address, U256};
use ethers_core::types::transaction::eip1559::Eip1559TransactionRequest;
use ethers_core::types::Signature;
use ethers_core::utils::{hex, keccak256};

use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaKeyId, EcdsaPublicKeyArgument, SignWithEcdsaArgument,
};

/// A signed transaction.
type SignedTransaction = String;

/// Gets the canister's ECDSA public key.
///
/// # Arguments
///
/// * `key_id` - The ID of the ECDSA key.
/// * `derivation_path` - The derivation path of the ECDSA key.
/// * `canister_id` - The ID of the canister.
///
/// # Returns
///
/// The public key of the ECDSA key.
pub async fn get_canister_public_key(
    key_id: EcdsaKeyId,
    canister_id: Option<Principal>,
    derivation_path: Vec<Vec<u8>>,
) -> Vec<u8> {
    let (key,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id,
        derivation_path,
        key_id,
    })
    .await
    .expect("failed to get public key");
    key.public_key
}

/// Signs an EIP-1559 transaction.
///
/// # Arguments
///
/// * `tx` - The EIP-1559 transaction to sign.
/// * `key_id` - The ID of the ECDSA key.
/// * `derivation_path` - The derivation path of the ECDSA key.
///
/// # Returns
///
/// The signed transaction.
pub async fn sign_eip1559_transaction(
    tx: Eip1559TransactionRequest,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
) -> SignedTransaction {
    const EIP1559_TX_ID: u8 = 2;

    let ecdsa_pub_key =
        get_canister_public_key(key_id.clone(), None, derivation_path.clone()).await;

    let mut unsigned_tx_bytes = tx.rlp().to_vec();
    unsigned_tx_bytes.insert(0, EIP1559_TX_ID);

    let txhash = keccak256(&unsigned_tx_bytes);

    let signature = sign_with_ecdsa(SignWithEcdsaArgument {
        message_hash: txhash.to_vec(),
        derivation_path,
        key_id,
    })
    .await
    .expect("failed to sign the transaction")
    .0
    .signature;

    let signature = Signature {
        v: y_parity(&txhash, &signature, &ecdsa_pub_key),
        r: U256::from_big_endian(&signature[0..32]),
        s: U256::from_big_endian(&signature[32..64]),
    };

    let mut signed_tx_bytes = tx.rlp_signed(&signature).to_vec();
    signed_tx_bytes.insert(0, EIP1559_TX_ID);

    format!("0x{}", hex::encode(&signed_tx_bytes))
}

/// Converts the public key bytes to an Ethereum address with a checksum.
///
/// # Arguments
///
/// * `pubkey_bytes` - The public key bytes.
///
/// # Returns
///
/// The Ethereum address with a checksum.
pub fn pubkey_bytes_to_address(pubkey_bytes: &[u8]) -> String {
    use ethers_core::k256::elliptic_curve::sec1::ToEncodedPoint;
    use ethers_core::k256::PublicKey;

    let key =
        PublicKey::from_sec1_bytes(pubkey_bytes).expect("failed to parse the public key as SEC1");
    let point = key.to_encoded_point(false);
    // we re-encode the key to the decompressed representation.
    let point_bytes = point.as_bytes();
    assert_eq!(point_bytes[0], 0x04);

    let hash = keccak256(&point_bytes[1..]);

    ethers_core::utils::to_checksum(&Address::from_slice(&hash[12..32]), None)
}

/// Computes the parity bit allowing to recover the public key from the signature.
///
/// # Arguments
///
/// * `prehash` - The prehash of the message.
/// * `sig` - The signature.
/// * `pubkey` - The public key.
///
/// # Returns
///
/// The parity bit.
fn y_parity(prehash: &[u8], sig: &[u8], pubkey: &[u8]) -> u64 {
    use ethers_core::k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

    let orig_key = VerifyingKey::from_sec1_bytes(pubkey).expect("failed to parse the pubkey");
    let signature = Signature::try_from(sig).unwrap();
    for parity in [0u8, 1] {
        let recid = RecoveryId::try_from(parity).unwrap();
        let recovered_key = VerifyingKey::recover_from_prehash(prehash, &signature, recid)
            .expect("failed to recover key");
        if recovered_key == orig_key {
            return parity as u64;
        }
    }

    panic!(
        "failed to recover the parity bit from a signature; sig: {}, pubkey: {}",
        hex::encode(sig),
        hex::encode(pubkey)
    )
}
