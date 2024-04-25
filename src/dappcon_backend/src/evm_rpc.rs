// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(
    dead_code,
    unused_imports,
    non_snake_case,
    clippy::large_enum_variant,
    clippy::enum_variant_names
)]
use candid::{self, CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub nodesInSubnet: u32,
}
#[derive(CandidType, Deserialize)]
pub enum Auth {
    RegisterProvider,
    FreeRpc,
    PriorityRpc,
    Manage,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum EthSepoliaService {
    Alchemy,
    BlockPi,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct HttpHeader {
    pub value: String,
    pub name: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RpcApi {
    pub url: String,
    pub headers: Option<Vec<HttpHeader>>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum EthMainnetService {
    Alchemy,
    BlockPi,
    Cloudflare,
    PublicNode,
    Ankr,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum RpcServices {
    EthSepolia(Option<Vec<EthSepoliaService>>),
    Custom { chainId: u64, services: Vec<RpcApi> },
    EthMainnet(Option<Vec<EthMainnetService>>),
}

#[derive(CandidType, Deserialize)]
pub struct RpcConfig {
    pub responseSizeEstimate: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum BlockTag {
    Earliest,
    Safe,
    Finalized,
    Latest,
    Number(candid::Nat),
    Pending,
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistoryArgs {
    pub blockCount: candid::Nat,
    pub newestBlock: BlockTag,
    pub rewardPercentiles: Option<serde_bytes::ByteBuf>,
}

#[derive(CandidType, Deserialize)]
pub struct FeeHistory {
    pub reward: Vec<Vec<candid::Nat>>,
    pub gasUsedRatio: Vec<f64>,
    pub oldestBlock: candid::Nat,
    pub baseFeePerGas: Vec<candid::Nat>,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ProviderError {
    TooFewCycles {
        expected: candid::Nat,
        received: candid::Nat,
    },
    MissingRequiredProvider,
    ProviderNotFound,
    NoPermission,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum ValidationError {
    CredentialPathNotAllowed,
    HostNotAllowed(String),
    CredentialHeaderNotAllowed,
    UrlParseError(String),
    Custom(String),
    InvalidHex(String),
}

#[derive(CandidType, Deserialize, Debug, PartialEq)]
pub enum RejectionCode {
    NoError,
    CanisterError,
    SysTransient,
    DestinationInvalid,
    Unknown,
    SysFatal,
    CanisterReject,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum HttpOutcallError {
    IcError {
        code: RejectionCode,
        message: String,
    },
    InvalidHttpJsonRpcResponse {
        status: u16,
        body: String,
        parsingError: Option<String>,
    },
}

#[derive(CandidType, Deserialize, Debug)]
pub enum RpcError {
    JsonRpcError(JsonRpcError),
    ProviderError(ProviderError),
    ValidationError(ValidationError),
    HttpOutcallError(HttpOutcallError),
}

#[derive(CandidType, Deserialize)]
pub enum FeeHistoryResult {
    Ok(Option<FeeHistory>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum RpcService {
    EthSepolia(EthSepoliaService),
    Custom(RpcApi),
    EthMainnet(EthMainnetService),
    Chain(u64),
    Provider(u64),
}

#[derive(CandidType, Deserialize)]
pub enum MultiFeeHistoryResult {
    Consistent(FeeHistoryResult),
    Inconsistent(Vec<(RpcService, FeeHistoryResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct Block {
    pub miner: String,
    pub totalDifficulty: candid::Nat,
    pub receiptsRoot: String,
    pub stateRoot: String,
    pub hash: String,
    pub difficulty: candid::Nat,
    pub size: candid::Nat,
    pub uncles: Vec<String>,
    pub baseFeePerGas: candid::Nat,
    pub extraData: String,
    pub transactionsRoot: Option<String>,
    pub sha3Uncles: String,
    pub nonce: candid::Nat,
    pub number: candid::Nat,
    pub timestamp: candid::Nat,
    pub transactions: Vec<String>,
    pub gasLimit: candid::Nat,
    pub logsBloom: String,
    pub parentHash: String,
    pub gasUsed: candid::Nat,
    pub mixHash: String,
}

#[derive(CandidType, Deserialize)]
pub enum GetBlockByNumberResult {
    Ok(Block),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetBlockByNumberResult {
    Consistent(GetBlockByNumberResult),
    Inconsistent(Vec<(RpcService, GetBlockByNumberResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetLogsArgs {
    pub fromBlock: Option<BlockTag>,
    pub toBlock: Option<BlockTag>,
    pub addresses: Vec<String>,
    pub topics: Option<Vec<Vec<String>>>,
}

#[derive(CandidType, Deserialize, Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub transactionHash: Option<String>,
    pub blockNumber: Option<candid::Nat>,
    pub data: String,
    pub blockHash: Option<String>,
    pub transactionIndex: Option<candid::Nat>,
    pub topics: Vec<String>,
    pub address: String,
    pub logIndex: Option<candid::Nat>,
    pub removed: bool,
}

#[derive(CandidType, Deserialize)]
pub enum GetLogsResult {
    Ok(Vec<LogEntry>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetLogsResult {
    Consistent(GetLogsResult),
    Inconsistent(Vec<(RpcService, GetLogsResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct GetTransactionCountArgs {
    pub address: String,
    pub block: BlockTag,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionCountResult {
    Ok(candid::Nat),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionCountResult {
    Consistent(GetTransactionCountResult),
    Inconsistent(Vec<(RpcService, GetTransactionCountResult)>),
}

#[derive(CandidType, Deserialize)]
pub struct TransactionReceipt {
    pub to: String,
    pub status: candid::Nat,
    pub transactionHash: String,
    pub blockNumber: candid::Nat,
    pub from: String,
    pub logs: Vec<LogEntry>,
    pub blockHash: String,
    pub r#type: String,
    pub transactionIndex: candid::Nat,
    pub effectiveGasPrice: candid::Nat,
    pub logsBloom: String,
    pub contractAddress: Option<String>,
    pub gasUsed: candid::Nat,
}

#[derive(CandidType, Deserialize)]
pub enum GetTransactionReceiptResult {
    Ok(Option<TransactionReceipt>),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiGetTransactionReceiptResult {
    Consistent(GetTransactionReceiptResult),
    Inconsistent(Vec<(RpcService, GetTransactionReceiptResult)>),
}

#[derive(CandidType, Deserialize)]
pub enum SendRawTransactionStatus {
    Ok(Option<String>),
    NonceTooLow,
    NonceTooHigh,
    InsufficientFunds,
}

#[derive(CandidType, Deserialize)]
pub enum SendRawTransactionResult {
    Ok(SendRawTransactionStatus),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum MultiSendRawTransactionResult {
    Consistent(SendRawTransactionResult),
    Inconsistent(Vec<(RpcService, SendRawTransactionResult)>),
}

pub type ProviderId = u64;
#[derive(CandidType, Deserialize)]
pub struct Metrics {
    pub cyclesWithdrawn: candid::Nat,
    pub responses: Vec<((String, String, String), u64)>,
    pub errNoPermission: u64,
    pub inconsistentResponses: Vec<((String, String), u64)>,
    pub cyclesCharged: Vec<((String, String), candid::Nat)>,
    pub requests: Vec<((String, String), u64)>,
    pub errHttpOutcall: Vec<((String, String), u64)>,
    pub errHostNotAllowed: Vec<(String, u64)>,
}

#[derive(CandidType, Deserialize)]
pub struct ProviderView {
    pub cyclesPerCall: u64,
    pub owner: Principal,
    pub hostname: String,
    pub primary: bool,
    pub chainId: u64,
    pub cyclesPerMessageByte: u64,
    pub providerId: u64,
}

#[derive(CandidType, Deserialize)]
pub struct ManageProviderArgs {
    pub service: Option<RpcService>,
    pub primary: Option<bool>,
    pub providerId: u64,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterProviderArgs {
    pub cyclesPerCall: u64,
    pub credentialPath: String,
    pub hostname: String,
    pub credentialHeaders: Option<Vec<HttpHeader>>,
    pub chainId: u64,
    pub cyclesPerMessageByte: u64,
}

#[derive(CandidType, Deserialize)]
pub enum RequestResult {
    Ok(String),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub enum RequestCostResult {
    Ok(candid::Nat),
    Err(RpcError),
}

#[derive(CandidType, Deserialize)]
pub struct UpdateProviderArgs {
    pub cyclesPerCall: Option<u64>,
    pub credentialPath: Option<String>,
    pub hostname: Option<String>,
    pub credentialHeaders: Option<Vec<HttpHeader>>,
    pub primary: Option<bool>,
    pub cyclesPerMessageByte: Option<u64>,
    pub providerId: u64,
}

pub struct Service(pub Principal);
impl Service {
    pub async fn authorize(&self, arg0: Principal, arg1: Auth) -> Result<(bool,)> {
        ic_cdk::call(self.0, "authorize", (arg0, arg1)).await
    }
    pub async fn deauthorize(&self, arg0: Principal, arg1: Auth) -> Result<(bool,)> {
        ic_cdk::call(self.0, "deauthorize", (arg0, arg1)).await
    }
    pub async fn eth_fee_history(
        &self,
        arg0: RpcServices,
        arg1: Option<RpcConfig>,
        arg2: FeeHistoryArgs,
        cycles: u128,
    ) -> Result<(MultiFeeHistoryResult,)> {
        ic_cdk::api::call::call_with_payment128(
            self.0,
            "eth_feeHistory",
            (arg0, arg1, arg2),
            cycles,
        )
        .await
    }
    pub async fn eth_get_block_by_number(
        &self,
        arg0: RpcServices,
        arg1: Option<RpcConfig>,
        arg2: BlockTag,
        cycles: u128,
    ) -> Result<(MultiGetBlockByNumberResult,)> {
        ic_cdk::api::call::call_with_payment128(
            self.0,
            "eth_getBlockByNumber",
            (arg0, arg1, arg2),
            cycles,
        )
        .await
    }
    pub async fn eth_get_logs(
        &self,
        arg0: RpcServices,
        arg1: Option<RpcConfig>,
        arg2: GetLogsArgs,
        cycles: u128,
    ) -> Result<(MultiGetLogsResult,)> {
        ic_cdk::api::call::call_with_payment128(self.0, "eth_getLogs", (arg0, arg1, arg2), cycles)
            .await
    }
    pub async fn eth_get_transaction_count(
        &self,
        arg0: RpcServices,
        arg1: Option<RpcConfig>,
        arg2: GetTransactionCountArgs,
    ) -> Result<(MultiGetTransactionCountResult,)> {
        ic_cdk::call(self.0, "eth_getTransactionCount", (arg0, arg1, arg2)).await
    }
    pub async fn eth_get_transaction_receipt(
        &self,
        arg0: RpcServices,
        arg1: Option<RpcConfig>,
        arg2: String,
    ) -> Result<(MultiGetTransactionReceiptResult,)> {
        ic_cdk::call(self.0, "eth_getTransactionReceipt", (arg0, arg1, arg2)).await
    }
    pub async fn eth_send_raw_transaction(
        &self,
        arg0: RpcServices,
        arg1: Option<RpcConfig>,
        arg2: String,
        cycles: u128,
    ) -> Result<(MultiSendRawTransactionResult,)> {
        ic_cdk::api::call::call_with_payment128(
            self.0,
            "eth_sendRawTransaction",
            (arg0, arg1, arg2),
            cycles,
        )
        .await
    }
    pub async fn get_accumulated_cycle_count(&self, arg0: ProviderId) -> Result<(candid::Nat,)> {
        ic_cdk::call(self.0, "getAccumulatedCycleCount", (arg0,)).await
    }
    pub async fn get_authorized(&self, arg0: Auth) -> Result<(Vec<Principal>,)> {
        ic_cdk::call(self.0, "getAuthorized", (arg0,)).await
    }
    pub async fn get_metrics(&self) -> Result<(Metrics,)> {
        ic_cdk::call(self.0, "getMetrics", ()).await
    }
    pub async fn get_nodes_in_subnet(&self) -> Result<(u32,)> {
        ic_cdk::call(self.0, "getNodesInSubnet", ()).await
    }
    pub async fn get_open_rpc_access(&self) -> Result<(bool,)> {
        ic_cdk::call(self.0, "getOpenRpcAccess", ()).await
    }
    pub async fn get_providers(&self) -> Result<(Vec<ProviderView>,)> {
        ic_cdk::call(self.0, "getProviders", ()).await
    }
    pub async fn get_service_provider_map(&self) -> Result<(Vec<(RpcService, u64)>,)> {
        ic_cdk::call(self.0, "getServiceProviderMap", ()).await
    }
    pub async fn manage_provider(&self, arg0: ManageProviderArgs) -> Result<()> {
        ic_cdk::call(self.0, "manageProvider", (arg0,)).await
    }
    pub async fn register_provider(&self, arg0: RegisterProviderArgs) -> Result<(u64,)> {
        ic_cdk::call(self.0, "registerProvider", (arg0,)).await
    }
    pub async fn request(
        &self,
        arg0: RpcService,
        arg1: String,
        arg2: u64,
    ) -> Result<(RequestResult,)> {
        ic_cdk::call(self.0, "request", (arg0, arg1, arg2)).await
    }
    pub async fn request_cost(
        &self,
        arg0: RpcService,
        arg1: String,
        arg2: u64,
    ) -> Result<(RequestCostResult,)> {
        ic_cdk::call(self.0, "requestCost", (arg0, arg1, arg2)).await
    }
    pub async fn set_open_rpc_access(&self, arg0: bool) -> Result<()> {
        ic_cdk::call(self.0, "setOpenRpcAccess", (arg0,)).await
    }
    pub async fn unregister_provider(&self, arg0: ProviderId) -> Result<(bool,)> {
        ic_cdk::call(self.0, "unregisterProvider", (arg0,)).await
    }
    pub async fn update_provider(&self, arg0: UpdateProviderArgs) -> Result<()> {
        ic_cdk::call(self.0, "updateProvider", (arg0,)).await
    }
    pub async fn withdraw_accumulated_cycles(
        &self,
        arg0: ProviderId,
        arg1: Principal,
    ) -> Result<()> {
        ic_cdk::call(self.0, "withdrawAccumulatedCycles", (arg0, arg1)).await
    }
}

pub const CANISTER_ID: Principal =
    Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01"); // 7hfb6-caaaa-aaaar-qadga-cai
pub const EVM_RPC: Service = Service(CANISTER_ID);
