use alloy::consensus::{SignableTransaction, TxEip1559, TxEnvelope};
use alloy::eips::eip2718::Encodable2718;
use alloy::hex;
use alloy::primitives::{keccak256, Address, FixedBytes, Parity};
use alloy::signers::Signature;
use candid::Principal;

use evm_rpc_canister_types::RpcServices;
use ic_cdk::api::management_canister::ecdsa::{
    ecdsa_public_key, sign_with_ecdsa, EcdsaKeyId, EcdsaPublicKeyArgument, SignWithEcdsaArgument,
};

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

pub async fn sign_eip1559_transaction(
    tx: TxEip1559,
    key_id: EcdsaKeyId,
    derivation_path: Vec<Vec<u8>>,
) -> String {
    let ecdsa_pub_key =
        get_canister_public_key(key_id.clone(), None, derivation_path.clone()).await;

    let tx_hash = tx.signature_hash();

    let r_and_s = sign_with_ecdsa(SignWithEcdsaArgument {
        message_hash: tx_hash.to_vec(),
        derivation_path,
        key_id,
    })
    .await
    .expect("failed to sign the transaction")
    .0
    .signature;

    let parity = y_parity(&tx_hash, &r_and_s, &ecdsa_pub_key);

    let signature =
        Signature::from_bytes_and_parity(&r_and_s, parity).expect("should be a valid signature");

    let signed_tx = tx.into_signed(signature);

    let tx_envelope = TxEnvelope::from(signed_tx);

    let signed_tx_bytes = tx_envelope.encoded_2718();

    format!("0x{}", hex::encode(&signed_tx_bytes))
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

pub trait IntoChainId {
    fn chain_id(&self) -> u64;
}

impl IntoChainId for RpcServices {
    fn chain_id(&self) -> u64 {
        match self {
            RpcServices::EthSepolia(_) => 11155111,
            RpcServices::Custom {
                chainId,
                services: _,
            } => *chainId,
            RpcServices::EthMainnet(_) => 1,
        }
    }
}
