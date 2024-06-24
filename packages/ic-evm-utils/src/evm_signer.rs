use alloy::consensus::{SignableTransaction, TxEip1559};
use alloy::hex;
use alloy::primitives::{keccak256, Address, Bytes, FixedBytes, Parity, TxKind, U256};
use alloy::signers::Signature;
use candid::Principal;

use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaKeyId, EcdsaPublicKeyArgument, SignWithEcdsaArgument,
};
use std::io::Read;
use std::str::FromStr;

pub struct SignRequest {
    pub chain_id: u64,
    pub from: Option<String>,
    pub to: TxKind,
    pub max_fee_per_gas: u128,
    pub max_priority_fee_per_gas: u128,
    pub value: U256,
    pub nonce: u64,
    pub data: Bytes,
}

pub async fn get_canister_public_key(
    key_id: EcdsaKeyId,
    canister_id: Option<Principal>,
    derivation_path: Option<Vec<Vec<u8>>>,
) -> Vec<u8> {
    let (key,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        canister_id,
        derivation_path: derivation_path.unwrap_or([].to_vec()),
        key_id,
    })
    .await
    .expect("failed to get public key");
    key.public_key
}

pub async fn sign_eip1559_transaction(
    req: SignRequest,
    key_id: EcdsaKeyId,
    ecdsa_pub_key: Vec<u8>,
) -> String {
    const EIP1559_TX_ID: u8 = 2;

    let tx = TxEip1559 {
        to: req.to,
        value: req.value,
        input: req.data,
        nonce: req.nonce,
        access_list: Default::default(),
        max_priority_fee_per_gas: req.max_priority_fee_per_gas,
        max_fee_per_gas: req.max_fee_per_gas,
        chain_id: req.chain_id,
        ..Default::default()
    };

    let tx_hash = tx.signature_hash();

    let signature = sign_with_ecdsa(SignWithEcdsaArgument {
        message_hash: tx_hash.to_vec(),
        derivation_path: [].to_vec(),
        key_id,
    })
    .await
    .expect("failed to sign the transaction")
    .0
    .signature;

    let parity = y_parity(&tx_hash, &signature, &ecdsa_pub_key);

    let typed_signature =
        Signature::from_bytes_and_parity(&signature, parity).expect("should be a valid signature");

    let signed_tx = tx.into_signed(typed_signature);

    format!("0x{}", hex::encode(&signed_tx))
}

/// Converts the public key bytes to an Ethereum address with a checksum.
pub fn pubkey_bytes_to_address(pubkey_bytes: &[u8]) -> String {
    use alloy::signers::k256::elliptic_curve::sec1::ToEncodedPoint;
    use alloy::signers::k256::PublicKey;

    let key =
        PublicKey::from_sec1_bytes(pubkey_bytes).expect("failed to parse the public key as SEC1");
    let point = key.to_encoded_point(false);
    // we re-encode the key to the decompressed representation.
    let point_bytes = point.as_bytes();
    assert_eq!(point_bytes[0], 0x04);

    let hash = keccak256(&point_bytes[1..]);

    alloy::primitives::Address::to_checksum(&Address::from_slice(&hash[12..32]), None)
}

/// Computes the parity bit allowing to recover the public key from the signature.
fn y_parity(prehash: &FixedBytes<32>, sig: &[u8], pubkey: &[u8]) -> Parity {
    use alloy::signers::k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

    let orig_key = VerifyingKey::from_sec1_bytes(pubkey).expect("failed to parse the pubkey");
    let signature = Signature::try_from(sig).unwrap();
    for parity in [0u8, 1] {
        let recid = RecoveryId::try_from(parity).unwrap();
        let recovered_key =
            VerifyingKey::recover_from_prehash(prehash.as_slice(), &signature, recid)
                .expect("failed to recover key");
        if recovered_key == orig_key {
            return Parity::Eip155(parity as u64);
        }
    }

    panic!(
        "failed to recover the parity bit from a signature; sig: {}, pubkey: {}",
        hex::encode(sig),
        hex::encode(pubkey)
    )
}
