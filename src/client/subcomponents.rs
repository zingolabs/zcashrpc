//! Sub-components of response messages.

use crate::ZecAmount;
use serde::{Deserialize, Serialize};

pub type ZGetNewAddressResponse = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetInfoResponse {
    balance: ZecAmount,
    blocks: u64,
    connections: u64,
    difficulty: f64,
    errors: String,
    keypoololdest: u64,
    keypoolsize: u64,
    paytxfee: ZecAmount,
    protocolversion: u64,
    proxy: String,
    relayfee: ZecAmount,
    testnet: bool,
    timeoffset: u64,
    version: u64,
    walletversion: u64,
}

pub type GenerateResponse = Vec<String>;

pub mod getblockchaininfo {
    use crate::ZecAmount;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetBlockChainInfoResponse {
        chain: String,
        blocks: u64,
        headers: u64,
        bestblockhash: String,
        difficulty: f64,
        verificationprogress: f64,
        chainwork: String,
        pruned: bool,
        size_on_disk: u64,
        commitments: u64,
        #[serde(rename = "valuePools")]
        value_pools: Vec<ValuePool>,
        softforks: Vec<Softfork>,
        upgrades: std::collections::HashMap<String, NetworkUpgradeDesc>,
        consensus: Consensus,
        pruneheight: Option<u64>,
        #[serde(rename = "fullyNotified")]
        fully_notified: Option<bool>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ValuePool {
        pub id: String,
        pub monitored: bool,
        #[serde(rename = "chainValue")]
        pub chain_value: Option<ZecAmount>,
        #[serde(rename = "chainValueZat")]
        pub chain_value_zat: Option<u64>,
        #[serde(rename = "valueDelta")]
        pub value_delta: Option<ZecAmount>,
        #[serde(rename = "valueDeltaZat")]
        pub value_delta_zat: Option<i64>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Softfork {
        pub id: String,
        pub version: i64,
        pub enforce: SoftforkMajorityDesc,
        pub reject: SoftforkMajorityDesc,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct SoftforkMajorityDesc {
        pub status: bool,
        pub found: i64,
        pub required: i64,
        pub window: serde_json::Value, // FIXME
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NetworkUpgradeDesc {
        pub name: String,
        pub activationheight: u64,
        pub status: String, // FIXME: enum-ify
        pub info: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Consensus {
        pub chaintip: String,
        pub nextblock: String,
    }
}

pub use self::getblockchaininfo::GetBlockChainInfoResponse;
