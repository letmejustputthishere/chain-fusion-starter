use crate::state::{InvalidStateError, State};
use candid::types::number::Nat;
use candid::{CandidType, Deserialize};
use ethers_core::types::{H256, U256};
use ic_cdk::api::management_canister::ecdsa::EcdsaKeyId;
use std::str::FromStr;

use crate::evm_rpc::{BlockTag, RpcServices};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct InitArg {
    pub rpc_services: RpcServices,
    pub get_logs_address: Vec<String>,
    pub get_logs_topics: Option<Vec<Vec<String>>>,
    pub last_scraped_block_number: Nat,
    pub ecdsa_key_id: EcdsaKeyId,
    pub block_tag: BlockTag,
}

impl TryFrom<InitArg> for State {
    type Error = InvalidStateError;

    fn try_from(
        InitArg {
            rpc_services,
            get_logs_address,
            get_logs_topics,
            last_scraped_block_number,
            ecdsa_key_id,
            block_tag,
        }: InitArg,
    ) -> Result<Self, Self::Error> {
        // validate contract addresses
        for contract_address in &get_logs_address {
            ethers_core::types::Address::from_str(contract_address).map_err(|e| {
                InvalidStateError::InvalidEthereumContractAddress(format!("ERROR: {}", e))
            })?;
        }
        // validate get_logs topics
        if let Some(topics) = &get_logs_topics {
            for topic in topics {
                validate_topics(topic)?;
            }
        }

        let state = Self {
            rpc_services,
            get_logs_address,
            get_logs_topics,
            last_scraped_block_number,
            last_observed_block_number: None,
            logs_to_process: Default::default(),
            processed_logs: Default::default(),
            skipped_blocks: Default::default(),
            active_tasks: Default::default(),
            ecdsa_pub_key: None,
            ecdsa_key_id,
            evm_address: None,
            nonce: U256::zero(),
            block_tag,
        };
        Ok(state)
    }
}

// Function to validate a single topic
fn validate_topic(topic: &str) -> Result<ethers_core::types::TxHash, InvalidStateError> {
    H256::from_str(topic).map_err(|e| InvalidStateError::InvalidTopic(format!("ERROR: {}", e)))
}

// Function to validate multiple topics
fn validate_topics(topics: &Vec<String>) -> Result<(), InvalidStateError> {
    for topic in topics {
        validate_topic(topic)?;
    }
    Ok(())
}
