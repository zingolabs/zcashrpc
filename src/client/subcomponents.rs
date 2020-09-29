//! Sub-components of response messages.

use crate::ZecAmount;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetInfoResponse {
    balance: ZecAmount,
    blocks: Decimal,
    connections: Decimal,
    difficulty: Decimal,
    errors: String,
    keypoololdest: Decimal,
    keypoolsize: Decimal,
    paytxfee: ZecAmount,
    protocolversion: Decimal,
    proxy: String,
    relayfee: ZecAmount,
    testnet: bool,
    timeoffset: Decimal,
    version: Decimal,
    walletversion: Decimal,
}

pub mod getblockchaininfo {
    use crate::ZecAmount;
    use rust_decimal::Decimal;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct GetBlockChainInfoResponse {
        chain: String,
        blocks: Decimal,
        headers: Decimal,
        bestblockhash: String,
        difficulty: Decimal,
        verificationprogress: Decimal,
        chainwork: String,
        pruned: bool,
        size_on_disk: Decimal,
        commitments: Decimal,
        #[serde(rename = "valuePools")]
        value_pools: Vec<ValuePool>,
        softforks: Vec<Softfork>,
        upgrades: std::collections::HashMap<String, NetworkUpgradeDesc>,
        consensus: Consensus,
        pruneheight: Option<Decimal>,
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
        pub chain_value_zat: Option<Decimal>,
        #[serde(rename = "valueDelta")]
        pub value_delta: Option<ZecAmount>,
        #[serde(rename = "valueDeltaZat")]
        pub value_delta_zat: Option<Decimal>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Softfork {
        pub id: String,
        pub version: Decimal,
        pub enforce: SoftforkMajorityDesc,
        pub reject: SoftforkMajorityDesc,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct SoftforkMajorityDesc {
        pub status: bool,
        pub found: Decimal,
        pub required: Decimal,
        pub window: serde_json::Value, // FIXME
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct NetworkUpgradeDesc {
        pub name: String,
        pub activationheight: Decimal,
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
pub type ZGetNewAddressResponse = String;
pub type GenerateResponse = Vec<String>;

#[cfg(test)]
mod tests {
    #[test]
    fn serialize_and_deserialize_empty() {
        let foo = "";
        let jfoo = serde_json::json!(foo);
        assert_eq!(foo, jfoo);
        assert_ne!(format!("{}", jfoo), format!("{}", foo));
    }

    #[test]
    fn serialize_and_deserialize_getinfo_response() {
        use rust_decimal::Decimal;
        use std::str::FromStr as _;
        let example_get_info_result = super::GetInfoResponse {
            balance: Decimal::from_str("3201.875").unwrap(),
            blocks: Decimal::from_str("1107").unwrap(),
            connections: Decimal::from_str("0").unwrap(),
            difficulty: Decimal::from_str("1.00000202656215").unwrap(),
            errors: "WARNING: ...".to_string(),
            keypoololdest: Decimal::from_str("1598484639").unwrap(),
            keypoolsize: Decimal::from_str("101").unwrap(),
            paytxfee: Decimal::from_str("0").unwrap(),
            protocolversion: Decimal::from_str("170012").unwrap(),
            proxy: "".to_string(),
            relayfee: Decimal::from_str("0.000001").unwrap(),
            testnet: false,
            timeoffset: Decimal::from_str("0").unwrap(),
            version: Decimal::from_str("4000025").unwrap(),
            walletversion: Decimal::from_str("60000").unwrap(),
        };
    }

    #[test]
    fn read_json_from_file() {
        use std::path::Path;
        let filename = &std::fs::canonicalize(Path::new(file!())).unwrap();
        let thisfile = Path::new(filename);
        let package_root = thisfile
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap();
        let getinfo_resp_json =
            package_root.join("json_data/GetInfoResponse.json");
        dbg!(&getinfo_resp_json);
        let file = std::fs::File::open(getinfo_resp_json).unwrap();
        dbg!(file);
    }
}
