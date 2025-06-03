use std::fmt::Display;

use bdk_wallet::bitcoin::{hashes::Hash, Txid};
use serde::{Deserialize, Serialize};

// Define an enum to represent different types of data.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum KenshoRequestData {
    Open(OpenVaultReq),
    Deposit(DepositVaultReq),
    Withdraw(WithdrawVaultReq),
    Borrow(BorrowVaultReq),
    Repay(RepayVaultReq),
    AccountReserve(AccountReserveReq),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventType {
    #[serde(rename = "req")]
    Request,
    #[serde(rename = "rej")]
    Reject,
    #[serde(rename = "res")]
    Result,
    #[serde(rename = "info")]
    Info,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum GuardianTopic {
    #[serde(rename = "/unit/reserve")]
    UnitReserve,
    #[serde(rename = "/vault/open")]
    VaultOpen,
    #[serde(rename = "/vault/borrow")]
    VaultBorrow,
    #[serde(rename = "/vault/repay")]
    VaultRepay,
    #[serde(rename = "/vault/repo")]
    VaultRepo,
    #[serde(rename = "/vault/deposit")]
    VaultDeposit,
    #[serde(rename = "/vault/withdraw")]
    VaultWithdraw,
    #[serde(rename = "error")]
    Error,
}

impl Display for GuardianTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl GuardianTopic {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnitReserve => "/unit/reserve",
            Self::VaultOpen => "/vault/open",
            Self::VaultBorrow => "/vault/borrow",
            Self::VaultRepay => "/vault/repay",
            Self::VaultRepo => "/vault/repo",
            Self::VaultDeposit => "/vault/deposit",
            Self::VaultWithdraw => "/vault/withdraw",
            Self::Error => "error",
        }
    }
}

// MessageEnvelope = [ type: EventType, id: string, topic: GuardianTopic, data: any ];
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsServerRequest {
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub id: String,
    pub topic: GuardianTopic,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UTXO {
    pub txid: Txid,
    pub vout: u32,
    pub value: u64,
    pub script: String,
    pub witness: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LUTXO {
    pub txid: Txid,
    pub vout: u32,
    pub value: u64,
    pub script: String,
    pub witness: Vec<String>,
    pub vault_pubkey:String,
}

impl Default for UTXO {
    fn default() -> Self {
        Self {
            txid: Txid::all_zeros(), // Use all_zeros() instead of default()
            vout: 0,
            value: 0,
            script: String::new(),
            witness: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AcctUtxo {
    pub txid: Txid,
    pub vout: u32,
    pub value: u64,
    pub script: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct OpenVaultReq {
    pub tx_feerate: u64,
    //pub vault_psbt: Option<String>,
    pub vault_pubkey: String,
    pub vault_quote: PriceQuote,
    pub acct_id: String,
    pub borrow_amount: u64,
    pub connect_input: UTXO,
    pub deposit_amount: u64,
    pub sats_address: String,
    pub sats_inputs: Vec<UTXO>,
    pub unit_address: String,
    pub unit_postage: u64,
    pub token_address: String,
    pub token_data: VaultTokenData,
    pub token_postage: u64,
    //pub issue_psbt: Option<String>,
    pub vault_txid: String,
    pub issue_txid: String,
    // pub issue_txhex: String,
    // pub vault_txhex: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct VaultTokenData {
    pub rev: u8,
    pub tag: String,
    pub ver: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TokenMetadata {
    pub gpk: String,
    pub mid: String,
    pub vpk: String,
    pub ver: u8,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DepositVaultReq {
    pub tx_feerate: u64,
    pub vault_pubkey: String,
    pub vault_quote: PriceQuote,
    pub deposit_amount: u64,
    pub sats_address: String,
    pub sats_inputs: Vec<UTXO>,
    pub vault_input: UTXO,
    pub vault_txid: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct WithdrawVaultReq {
    pub tx_feerate: u32,
    pub vault_pubkey: String,
    pub vault_quote: PriceQuote,
    pub change_amount: u64,
    pub sats_address: String,
    pub vault_input: UTXO,
    pub vault_txid: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BorrowVaultReq {
    pub tx_feerate: u64,
    pub vault_pubkey: String,
    pub vault_quote: PriceQuote,
    pub acct_id: String,
    pub borrow_amount: u64,
    pub connect_input: UTXO,
    pub deposit_amount: u64,
    pub sats_address: String,
    pub sats_inputs: Vec<UTXO>,
    pub unit_address: String,
    pub unit_postage: u64,
    pub vault_input: UTXO,
    pub vault_txid: String,
    pub issue_txid: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RepayVaultReq {
    pub tx_feerate: u64,
    pub vault_pubkey: String,
    pub vault_quote: PriceQuote,
    pub acct_id: String,
    pub connect_input: UTXO,
    pub repay_amount: u64,
    pub sats_address: String,
    pub sats_inputs: Vec<UTXO>,
    pub unit_inputs: Vec<UTXO>,
    pub unit_address: String,
    pub unit_postage: u64,
    pub vault_input: UTXO,
    pub vault_txid: String,
    pub repay_txid: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RepoVaultReq {
    pub tx_feerate: f64,
    pub vault_action: String,
  //  pub vault_psbt: String,
   // pub vault_txhex: String,
    pub vault_txid: String,
    pub vault_pubkey: String,
    pub vault_quote: PriceQuote,
    pub connect_input: UTXO,
    pub deposit_amount: u64,
    pub repo_amount: u64,
   // pub liquid_psbt: String,
  //  pub liquid_txhex: String,
    pub liquid_txid: String,
    pub liquid_inputs: Vec<LUTXO>,
    pub sats_address: String,
    pub sats_inputs: Vec<UTXO>,
    pub vault_input: UTXO,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct VaultQuote {
    pub oracle_pk: String,
    pub quote_price: u64,
    pub quote_stamp: u64,
    pub req_id: String,
    pub req_sig: String,
    pub thold_hash: String,
    pub thold_price: u32,
    pub is_expired: bool,
    #[serde(default)] // For null values
    pub eval_price: Option<u64>,
    #[serde(default)]
    pub eval_stamp: Option<u64>,
    #[serde(default)]
    pub thold_key: Option<String>,
}

// TODO: These are just placeholders, we need to add the required fields
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct LiquidateVaultReq {
    pub sats_address: String,
    pub exchange_price: u64,
    pub exchange_stamp: u64,
    pub tx_feerate: u64,
    pub vault_psbt: String,
    pub price_quote: PriceQuote,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct TransferVaultReq {
    pub sats_address: String,
    pub exchange_price: u64,
    pub exchange_stamp: u64,
    pub tx_feerate: u64,
    pub vault_psbt: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct AccountReserveReq {
    pub unit_amount: f64,
    pub vault_pubkey: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PriceQuote {
    pub curr_price: Option<u64>,
    pub curr_stamp: Option<u64>,
    pub stop_price: Option<u64>,
    pub stop_stamp: Option<u64>,
    pub is_expired: bool,
    pub quote_price: u64,
    pub quote_stamp: u64,
    pub oracle_pk: String,
    pub req_id: String,
    pub req_sig: String,
    pub req_stamp: Option<u64>,
    pub thold_hash: String,
    pub thold_key: Option<String>,
    pub thold_price: u32,
}

