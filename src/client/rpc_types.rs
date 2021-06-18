//procedurally generated response types, note that zcashrpc-typegen
//is in early alpha, and output is subject to change at any time.
pub mod addmultisigaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AddmultisigaddressArguments(
        crate::client::utils::ZDecimal,
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type AddmultisigaddressResponse = String;
}
pub mod addnode {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AddnodeArguments(String, String);
    pub type AddnodeResponse = ();
}
pub mod backupwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct BackupwalletArguments(String);
    pub type BackupwalletResponse = String;
}
pub mod clearbanned {
    pub type ClearbannedResponse = ();
}
pub mod createmultisig {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct CreatemultisigArguments(crate::client::utils::ZDecimal, String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct CreatemultisigResponse {
        pub address: String,
        pub redeem_script: String,
    }
}
pub mod createrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct CreaterawtransactionArguments(
        String,
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    pub type CreaterawtransactionResponse = String;
}
pub mod decoderawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DecoderawtransactionArguments(String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DecoderawtransactionResponse {
        pub expiryheight: Option<crate::client::utils::ZDecimal>,
        pub versiongroupid: Option<String>,
        pub locktime: crate::client::utils::ZDecimal,
        pub overwintered: bool,
        pub size: crate::client::utils::ZDecimal,
        pub txid: String,
        pub version: crate::client::utils::ZDecimal,
        pub vin: Vec<Vin>,
        pub vjoinsplit: Vec<Vjoinsplit>,
        pub vout: Vec<Vout>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptPubKey {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub req_sigs: crate::client::utils::ZDecimal,
        #[serde(rename = "type")]
        pub type_field: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptSig {
        pub asm: String,
        pub hex: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vin {
        pub script_sig: ScriptSig,
        pub sequence: crate::client::utils::ZDecimal,
        pub txid: String,
        pub vout: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vjoinsplit {
        pub anchor: String,
        pub ciphertexts: Vec<String>,
        pub commitments: Vec<String>,
        pub macs: Vec<String>,
        pub nullifiers: Vec<String>,
        pub onetime_pub_key: String,
        pub proof: String,
        pub random_seed: String,
        pub vpub_new: crate::client::utils::ZDecimal,
        pub vpub_old: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vout {
        pub n: crate::client::utils::ZDecimal,
        pub script_pub_key: ScriptPubKey,
        pub value: crate::client::utils::ZDecimal,
    }
}
pub mod decodescript {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DecodescriptArguments(String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DecodescriptResponse {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub p2sh: String,
        pub req_sigs: crate::client::utils::ZDecimal,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod disconnectnode {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DisconnectnodeArguments(String);
    pub type DisconnectnodeResponse = ();
}
pub mod dumpprivkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DumpprivkeyArguments(String);
    pub type DumpprivkeyResponse = String;
}
pub mod dumpwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DumpwalletArguments(String);
    pub type DumpwalletResponse = String;
}
pub mod encryptwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct EncryptwalletArguments(String);
    pub type EncryptwalletResponse = ();
}
pub mod estimatefee {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct EstimatefeeArguments(crate::client::utils::ZDecimal);
    pub type EstimatefeeResponse = crate::client::utils::ZDecimal;
}
pub mod estimatepriority {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct EstimatepriorityArguments(crate::client::utils::ZDecimal);
    pub type EstimatepriorityResponse = crate::client::utils::ZDecimal;
}
pub mod fundrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct FundrawtransactionArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct FundrawtransactionResponse {
        pub changepos: crate::client::utils::ZDecimal,
        pub fee: crate::client::utils::ZDecimal,
        pub hex: String,
    }
}
pub mod generate {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GenerateArguments(crate::client::utils::ZDecimal);
    pub type GenerateResponse = Vec<String>;
}
pub mod getaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaccountArguments(String);
    pub type GetaccountResponse = String;
}
pub mod getaccountaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaccountaddressArguments(String);
    pub type GetaccountaddressResponse = String;
}
pub mod getaddednodeinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddednodeinfoArguments(
        bool,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Addresses {
        pub address: String,
        pub connected: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddednodeinfoElement {
        pub addednode: String,
        pub addresses: Vec<Addresses>,
        pub connected: bool,
    }
    pub type GetaddednodeinfoResponse = Vec<GetaddednodeinfoElement>;
}
pub mod getaddressbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetaddressbalanceArguments {
        MultiAddress(Arg1),
        Address(String),
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Arg1 {
        pub addresses: Vec<String>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressbalanceResponse {
        pub balance: crate::client::utils::ZDecimal,
        pub received: crate::client::utils::ZDecimal,
    }
}
pub mod getaddressdeltas {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetaddressdeltasArguments {
        MultiAddress(Arg1),
        Address(String),
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Arg1 {
        pub chain_info: Option<bool>,
        pub end: Option<crate::client::utils::ZDecimal>,
        pub start: Option<crate::client::utils::ZDecimal>,
        pub addresses: Vec<String>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetaddressdeltasResponse {
        Regular(Vec<Regular>),
        Verbose {
            deltas: Vec<Deltas>,
            end: End,
            start: Start,
        },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Deltas {
        pub address: String,
        pub height: crate::client::utils::ZDecimal,
        pub index: crate::client::utils::ZDecimal,
        pub satoshis: crate::client::utils::ZDecimal,
        pub txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct End {
        pub hash: String,
        pub height: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Regular {
        pub address: String,
        pub height: crate::client::utils::ZDecimal,
        pub index: crate::client::utils::ZDecimal,
        pub satoshis: crate::client::utils::ZDecimal,
        pub txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Start {
        pub hash: String,
        pub height: crate::client::utils::ZDecimal,
    }
}
pub mod getaddressesbyaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressesbyaccountArguments(String);
    pub type GetaddressesbyaccountResponse = Vec<String>;
}
pub mod getaddressmempool {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetaddressmempoolArguments {
        MultiAddress(Arg1),
        Address(String),
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Arg1 {
        pub addresses: Vec<String>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressmempoolElement {
        pub address: String,
        pub index: crate::client::utils::ZDecimal,
        pub prevout: String,
        pub prevtxid: String,
        pub satoshis: crate::client::utils::ZDecimal,
        pub timestamp: crate::client::utils::ZDecimal,
        pub txid: String,
    }
    pub type GetaddressmempoolResponse = Vec<GetaddressmempoolElement>;
}
pub mod getaddresstxids {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetaddresstxidsArguments {
        MultiAddress(Arg1),
        Address(String),
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Arg1 {
        pub end: Option<crate::client::utils::ZDecimal>,
        pub start: Option<crate::client::utils::ZDecimal>,
        pub addresses: Vec<String>,
    }
    pub type GetaddresstxidsResponse = Vec<String>;
}
pub mod getaddressutxos {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetaddressutxosArguments {
        MultiAddress(Arg1),
        Address(String),
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Arg1 {
        pub chain_info: Option<bool>,
        pub addresses: Vec<String>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetaddressutxosResponse {
        Regular(Vec<Regular>),
        Verbose {
            hash: String,
            height: crate::client::utils::ZDecimal,
            utxos: Vec<Utxos>,
        },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Regular {
        pub address: String,
        pub height: crate::client::utils::ZDecimal,
        pub output_index: crate::client::utils::ZDecimal,
        pub satoshis: crate::client::utils::ZDecimal,
        pub script: String,
        pub txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Utxos {
        pub address: String,
        pub height: crate::client::utils::ZDecimal,
        pub output_index: crate::client::utils::ZDecimal,
        pub satoshis: crate::client::utils::ZDecimal,
        pub script: String,
        pub txid: String,
    }
}
pub mod getbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetbalanceArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type GetbalanceResponse = crate::client::utils::ZDecimal;
}
pub mod getbestblockhash {
    pub type GetbestblockhashResponse = String;
}
pub mod getblock {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetblockResponse {
        Regular(String),
        Verbose {
            bits: String,
            confirmations: crate::client::utils::ZDecimal,
            difficulty: crate::client::utils::ZDecimal,
            finalsaplingroot: String,
            hash: String,
            height: crate::client::utils::ZDecimal,
            merkleroot: String,
            nextblockhash: String,
            nonce: crate::client::utils::ZDecimal,
            previousblockhash: String,
            size: crate::client::utils::ZDecimal,
            time: crate::client::utils::ZDecimal,
            tx: Vec<String>,
            version: crate::client::utils::ZDecimal,
        },
        VeryVerbose {
            bits: String,
            confirmations: crate::client::utils::ZDecimal,
            difficulty: crate::client::utils::ZDecimal,
            finalsaplingroot: String,
            hash: String,
            height: crate::client::utils::ZDecimal,
            merkleroot: String,
            nextblockhash: String,
            nonce: crate::client::utils::ZDecimal,
            previousblockhash: String,
            size: crate::client::utils::ZDecimal,
            time: crate::client::utils::ZDecimal,
            tx: Vec<Tx>,
            version: crate::client::utils::ZDecimal,
        },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptPubKey {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub req_sigs: crate::client::utils::ZDecimal,
        #[serde(rename = "type")]
        pub type_field: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptSig {
        pub asm: String,
        pub hex: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Tx {
        pub expiryheight: Option<crate::client::utils::ZDecimal>,
        pub blockhash: String,
        pub blocktime: crate::client::utils::ZDecimal,
        pub confirmations: crate::client::utils::ZDecimal,
        pub hex: String,
        pub in_active_chain: bool,
        pub locktime: crate::client::utils::ZDecimal,
        pub size: crate::client::utils::ZDecimal,
        pub time: crate::client::utils::ZDecimal,
        pub txid: String,
        pub version: crate::client::utils::ZDecimal,
        pub vin: Vec<Vin>,
        pub vjoinsplit: Vec<Vjoinsplit>,
        pub vout: Vec<Vout>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vin {
        pub script_sig: ScriptSig,
        pub sequence: crate::client::utils::ZDecimal,
        pub txid: String,
        pub vout: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vjoinsplit {
        pub anchor: String,
        pub ciphertexts: Vec<String>,
        pub commitments: Vec<String>,
        pub macs: Vec<String>,
        pub nullifiers: Vec<String>,
        pub onetime_pub_key: String,
        pub proof: String,
        pub random_seed: String,
        pub vpub_new: crate::client::utils::ZDecimal,
        pub vpub_old: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vout {
        pub n: crate::client::utils::ZDecimal,
        pub script_pub_key: ScriptPubKey,
        pub value: crate::client::utils::ZDecimal,
    }
}
pub mod getblockchaininfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Consensus {
        pub chaintip: String,
        pub nextblock: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Enforce {
        pub found: crate::client::utils::ZDecimal,
        pub required: crate::client::utils::ZDecimal,
        pub status: bool,
        pub window: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockchaininfoResponse {
        pub bestblockhash: String,
        pub blocks: crate::client::utils::ZDecimal,
        pub chain: String,
        pub chainwork: String,
        pub commitments: crate::client::utils::ZDecimal,
        pub consensus: Consensus,
        pub difficulty: crate::client::utils::ZDecimal,
        pub estimatedheight: crate::client::utils::ZDecimal,
        pub headers: crate::client::utils::ZDecimal,
        pub initial_block_download_complete: bool,
        pub size_on_disk: crate::client::utils::ZDecimal,
        pub softforks: Vec<Softforks>,
        pub upgrades: std::collections::HashMap<String, Upgrades>,
        pub verificationprogress: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Reject {
        pub found: crate::client::utils::ZDecimal,
        pub required: crate::client::utils::ZDecimal,
        pub status: bool,
        pub window: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Softforks {
        pub enforce: Enforce,
        pub id: String,
        pub reject: Reject,
        pub version: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Upgrades {
        pub activationheight: crate::client::utils::ZDecimal,
        pub info: String,
        pub name: String,
        pub status: String,
    }
}
pub mod getblockcount {
    pub type GetblockcountResponse = crate::client::utils::ZDecimal;
}
pub mod getblockdeltas {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockdeltasArguments(String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Deltas {
        pub index: crate::client::utils::ZDecimal,
        pub inputs: Vec<Inputs>,
        pub outputs: Vec<Outputs>,
        pub txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockdeltasResponse {
        pub bits: String,
        pub chainwork: String,
        pub confirmations: crate::client::utils::ZDecimal,
        pub deltas: Vec<Deltas>,
        pub difficulty: crate::client::utils::ZDecimal,
        pub hash: String,
        pub height: crate::client::utils::ZDecimal,
        pub mediantime: crate::client::utils::ZDecimal,
        pub merkleroot: String,
        pub nextblockhash: String,
        pub nonce: String,
        pub previousblockhash: String,
        pub size: crate::client::utils::ZDecimal,
        pub time: crate::client::utils::ZDecimal,
        pub version: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Inputs {
        pub address: String,
        pub index: crate::client::utils::ZDecimal,
        pub prevout: crate::client::utils::ZDecimal,
        pub prevtxid: String,
        pub satoshis: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Outputs {
        pub address: String,
        pub index: crate::client::utils::ZDecimal,
        pub satoshis: crate::client::utils::ZDecimal,
    }
}
pub mod getblockhash {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockhashArguments(crate::client::utils::ZDecimal);
    pub type GetblockhashResponse = String;
}
pub mod getblockhashes {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockhashesArguments(
        crate::client::utils::ZDecimal,
        crate::client::utils::ZDecimal,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type GetblockhashesResponse = Vec<String>;
}
pub mod getblockheader {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockheaderArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetblockheaderResponse {
        Regular(String),
        Verbose {
            bits: String,
            confirmations: crate::client::utils::ZDecimal,
            difficulty: crate::client::utils::ZDecimal,
            finalsaplingroot: String,
            hash: String,
            height: crate::client::utils::ZDecimal,
            merkleroot: String,
            nextblockhash: String,
            nonce: crate::client::utils::ZDecimal,
            previousblockhash: String,
            time: crate::client::utils::ZDecimal,
            version: crate::client::utils::ZDecimal,
        },
    }
}
pub mod getblocksubsidy {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblocksubsidyArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Fundingstreams {
        pub address: String,
        pub recipient: String,
        pub specification: String,
        pub value: crate::client::utils::ZDecimal,
        pub value_zat: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblocksubsidyResponse {
        pub founders: crate::client::utils::ZDecimal,
        pub fundingstreams: Vec<Fundingstreams>,
        pub miner: crate::client::utils::ZDecimal,
    }
}
pub mod getblocktemplate {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblocktemplateArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Coinbasetxn {
        pub data: String,
        pub depends: Vec<crate::client::utils::ZDecimal>,
        pub fee: crate::client::utils::ZDecimal,
        pub foundersreward: crate::client::utils::ZDecimal,
        pub hash: String,
        pub required: bool,
        pub sigops: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblocktemplateResponse {
        pub bits: String,
        pub coinbasetxn: Coinbasetxn,
        pub curtime: crate::client::utils::ZDecimal,
        pub finalsaplingroothash: String,
        pub height: crate::client::utils::ZDecimal,
        pub lightclientroothash: String,
        pub longpollid: String,
        pub mintime: crate::client::utils::ZDecimal,
        pub mutable: Vec<String>,
        pub noncerange: String,
        pub previousblockhash: String,
        pub sigoplimit: crate::client::utils::ZDecimal,
        pub sizelimit: crate::client::utils::ZDecimal,
        pub target: String,
        pub transactions: Vec<Transactions>,
        pub version: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Transactions {
        pub data: String,
        pub depends: Vec<crate::client::utils::ZDecimal>,
        pub fee: crate::client::utils::ZDecimal,
        pub hash: String,
        pub required: bool,
        pub sigops: crate::client::utils::ZDecimal,
    }
}
pub mod getchaintips {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetchaintipsElement {
        pub branchlen: crate::client::utils::ZDecimal,
        pub hash: String,
        pub height: crate::client::utils::ZDecimal,
        pub status: String,
    }
    pub type GetchaintipsResponse = Vec<GetchaintipsElement>;
}
pub mod getconnectioncount {
    pub type GetconnectioncountResponse = crate::client::utils::ZDecimal;
}
pub mod getdeprecationinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetdeprecationinfoResponse {
        pub deprecationheight: crate::client::utils::ZDecimal,
        pub subversion: String,
        pub version: crate::client::utils::ZDecimal,
    }
}
pub mod getdifficulty {
    pub type GetdifficultyResponse = crate::client::utils::ZDecimal;
}
pub mod getexperimentalfeatures {
    pub type GetexperimentalfeaturesResponse = Vec<String>;
}
pub mod getgenerate {
    pub type GetgenerateResponse = bool;
}
pub mod getinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetinfoResponse {
        pub proxy: Option<String>,
        pub balance: crate::client::utils::ZDecimal,
        pub blocks: crate::client::utils::ZDecimal,
        pub connections: crate::client::utils::ZDecimal,
        pub difficulty: crate::client::utils::ZDecimal,
        pub errors: String,
        pub keypoololdest: crate::client::utils::ZDecimal,
        pub keypoolsize: crate::client::utils::ZDecimal,
        pub paytxfee: crate::client::utils::ZDecimal,
        pub protocolversion: crate::client::utils::ZDecimal,
        pub relayfee: crate::client::utils::ZDecimal,
        pub testnet: bool,
        pub timeoffset: crate::client::utils::ZDecimal,
        pub unlocked_until: crate::client::utils::ZDecimal,
        pub version: crate::client::utils::ZDecimal,
        pub walletversion: crate::client::utils::ZDecimal,
    }
}
pub mod getlocalsolps {
    pub type GetlocalsolpsResponse = crate::client::utils::ZDecimal;
}
pub mod getmemoryinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmemoryinfoResponse {
        pub locked: Locked,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Locked {
        pub chunks_free: crate::client::utils::ZDecimal,
        pub chunks_used: crate::client::utils::ZDecimal,
        pub free: crate::client::utils::ZDecimal,
        pub locked: crate::client::utils::ZDecimal,
        pub total: crate::client::utils::ZDecimal,
        pub used: crate::client::utils::ZDecimal,
    }
}
pub mod getmempoolinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmempoolinfoResponse {
        pub bytes: crate::client::utils::ZDecimal,
        pub size: crate::client::utils::ZDecimal,
        pub usage: crate::client::utils::ZDecimal,
    }
}
pub mod getmininginfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmininginfoResponse {
        pub blocks: crate::client::utils::ZDecimal,
        pub chain: String,
        pub currentblocksize: crate::client::utils::ZDecimal,
        pub currentblocktx: crate::client::utils::ZDecimal,
        pub difficulty: crate::client::utils::ZDecimal,
        pub errors: String,
        pub generate: bool,
        pub genproclimit: crate::client::utils::ZDecimal,
        pub localsolps: crate::client::utils::ZDecimal,
        pub networksolps: crate::client::utils::ZDecimal,
        pub pooledtx: crate::client::utils::ZDecimal,
        pub testnet: bool,
    }
}
pub mod getnettotals {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnettotalsResponse {
        pub timemillis: crate::client::utils::ZDecimal,
        pub totalbytesrecv: crate::client::utils::ZDecimal,
        pub totalbytessent: crate::client::utils::ZDecimal,
        pub uploadtarget: Uploadtarget,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Uploadtarget {
        pub bytes_left_in_cycle: crate::client::utils::ZDecimal,
        pub serve_historical_blocks: bool,
        pub target: crate::client::utils::ZDecimal,
        pub target_reached: bool,
        pub time_left_in_cycle: crate::client::utils::ZDecimal,
        pub timeframe: crate::client::utils::ZDecimal,
    }
}
pub mod getnetworkhashps {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnetworkhashpsArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    pub type GetnetworkhashpsResponse = crate::client::utils::ZDecimal;
}
pub mod getnetworkinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnetworkinfoResponse {
        pub connections: crate::client::utils::ZDecimal,
        pub localaddresses: Vec<Localaddresses>,
        pub localservices: String,
        pub networks: Vec<Networks>,
        pub protocolversion: crate::client::utils::ZDecimal,
        pub relayfee: crate::client::utils::ZDecimal,
        pub subversion: String,
        pub timeoffset: crate::client::utils::ZDecimal,
        pub version: crate::client::utils::ZDecimal,
        pub warnings: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Localaddresses {
        pub address: String,
        pub port: crate::client::utils::ZDecimal,
        pub score: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Networks {
        pub limited: bool,
        pub name: String,
        pub proxy: String,
        pub reachable: bool,
    }
}
pub mod getnetworksolps {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnetworksolpsArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    pub type GetnetworksolpsResponse = crate::client::utils::ZDecimal;
}
pub mod getnewaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnewaddressArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type GetnewaddressResponse = String;
}
pub mod getpeerinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetpeerinfoElement {
        pub addr: String,
        pub addrlocal: String,
        pub banscore: crate::client::utils::ZDecimal,
        pub bytesrecv: crate::client::utils::ZDecimal,
        pub bytessent: crate::client::utils::ZDecimal,
        pub conntime: crate::client::utils::ZDecimal,
        pub id: crate::client::utils::ZDecimal,
        pub inbound: bool,
        pub inflight: Vec<crate::client::utils::ZDecimal>,
        pub lastrecv: crate::client::utils::ZDecimal,
        pub lastsend: crate::client::utils::ZDecimal,
        pub pingtime: crate::client::utils::ZDecimal,
        pub pingwait: crate::client::utils::ZDecimal,
        pub relaytxes: bool,
        pub services: String,
        pub startingheight: crate::client::utils::ZDecimal,
        pub subver: String,
        pub synced_blocks: crate::client::utils::ZDecimal,
        pub synced_headers: crate::client::utils::ZDecimal,
        pub timeoffset: crate::client::utils::ZDecimal,
        pub version: crate::client::utils::ZDecimal,
    }
    pub type GetpeerinfoResponse = Vec<GetpeerinfoElement>;
}
pub mod getrawchangeaddress {
    pub type GetrawchangeaddressResponse = String;
}
pub mod getrawmempool {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetrawmempoolArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetrawmempoolResponse {
        Regular(Vec<String>),
        Verbose { transactionid: Transactionid },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Transactionid {
        pub currentpriority: crate::client::utils::ZDecimal,
        pub depends: Vec<String>,
        pub fee: crate::client::utils::ZDecimal,
        pub height: crate::client::utils::ZDecimal,
        pub size: crate::client::utils::ZDecimal,
        pub startingpriority: crate::client::utils::ZDecimal,
        pub time: crate::client::utils::ZDecimal,
    }
}
pub mod getrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetrawtransactionArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum GetrawtransactionResponse {
        Regular(String),
        Verbose {
            expiryheight: Option<crate::client::utils::ZDecimal>,
            blockhash: String,
            blocktime: crate::client::utils::ZDecimal,
            confirmations: crate::client::utils::ZDecimal,
            hex: String,
            in_active_chain: bool,
            locktime: crate::client::utils::ZDecimal,
            size: crate::client::utils::ZDecimal,
            time: crate::client::utils::ZDecimal,
            txid: String,
            version: crate::client::utils::ZDecimal,
            vin: Vec<Vin>,
            vjoinsplit: Vec<Vjoinsplit>,
            vout: Vec<Vout>,
        },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptPubKey {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub req_sigs: crate::client::utils::ZDecimal,
        #[serde(rename = "type")]
        pub type_field: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptSig {
        pub asm: String,
        pub hex: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vin {
        pub script_sig: ScriptSig,
        pub sequence: crate::client::utils::ZDecimal,
        pub txid: String,
        pub vout: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vjoinsplit {
        pub anchor: String,
        pub ciphertexts: Vec<String>,
        pub commitments: Vec<String>,
        pub macs: Vec<String>,
        pub nullifiers: Vec<String>,
        pub onetime_pub_key: String,
        pub proof: String,
        pub random_seed: String,
        pub vpub_new: crate::client::utils::ZDecimal,
        pub vpub_old: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vout {
        pub n: crate::client::utils::ZDecimal,
        pub script_pub_key: ScriptPubKey,
        pub value: crate::client::utils::ZDecimal,
    }
}
pub mod getreceivedbyaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetreceivedbyaccountArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type GetreceivedbyaccountResponse = crate::client::utils::ZDecimal;
}
pub mod getreceivedbyaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetreceivedbyaddressArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type GetreceivedbyaddressResponse = crate::client::utils::ZDecimal;
}
pub mod getspentinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetspentinfoArguments(String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetspentinfoResponse {
        pub index: crate::client::utils::ZDecimal,
        pub txid: String,
    }
}
pub mod gettransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettransactionArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Details {
        pub account: String,
        pub address: String,
        pub amount: crate::client::utils::ZDecimal,
        pub amount_zat: crate::client::utils::ZDecimal,
        pub category: String,
        pub vout: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettransactionResponse {
        pub amount: crate::client::utils::ZDecimal,
        pub amount_zat: crate::client::utils::ZDecimal,
        pub blockhash: String,
        pub blockindex: crate::client::utils::ZDecimal,
        pub blocktime: crate::client::utils::ZDecimal,
        pub confirmations: crate::client::utils::ZDecimal,
        pub details: Vec<Details>,
        pub hex: String,
        pub status: String,
        pub time: crate::client::utils::ZDecimal,
        pub timereceived: crate::client::utils::ZDecimal,
        pub txid: String,
        pub vjoinsplit: Vec<Vjoinsplit>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vjoinsplit {
        pub anchor: String,
        pub commitments: Vec<String>,
        pub macs: Vec<String>,
        pub nullifiers: Vec<String>,
        pub vpub_new: crate::client::utils::ZDecimal,
        pub vpub_old: crate::client::utils::ZDecimal,
    }
}
pub mod gettxout {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutArguments(
        String,
        crate::client::utils::ZDecimal,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutResponse {
        pub bestblock: String,
        pub coinbase: bool,
        pub confirmations: crate::client::utils::ZDecimal,
        pub script_pub_key: ScriptPubKey,
        pub value: crate::client::utils::ZDecimal,
        pub version: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptPubKey {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub req_sigs: crate::client::utils::ZDecimal,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod gettxoutproof {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutproofArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type GettxoutproofResponse = String;
}
pub mod gettxoutsetinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutsetinfoResponse {
        pub bestblock: String,
        pub bytes_serialized: crate::client::utils::ZDecimal,
        pub hash_serialized: String,
        pub height: crate::client::utils::ZDecimal,
        pub total_amount: crate::client::utils::ZDecimal,
        pub transactions: crate::client::utils::ZDecimal,
        pub txouts: crate::client::utils::ZDecimal,
    }
}
pub mod getunconfirmedbalance {
    pub type GetunconfirmedbalanceResponse = crate::client::utils::ZDecimal;
}
pub mod getwalletinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetwalletinfoResponse {
        pub balance: crate::client::utils::ZDecimal,
        pub immature_balance: crate::client::utils::ZDecimal,
        pub keypoololdest: crate::client::utils::ZDecimal,
        pub keypoolsize: crate::client::utils::ZDecimal,
        pub paytxfee: crate::client::utils::ZDecimal,
        pub seedfp: String,
        pub shielded_balance: crate::client::utils::ZDecimal,
        pub shielded_unconfirmed_balance: crate::client::utils::ZDecimal,
        pub txcount: crate::client::utils::ZDecimal,
        pub unconfirmed_balance: crate::client::utils::ZDecimal,
        pub unlocked_until: crate::client::utils::ZDecimal,
        pub walletversion: crate::client::utils::ZDecimal,
    }
}
pub mod help {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct HelpArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type HelpResponse = String;
}
pub mod importaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportaddressArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type ImportaddressResponse = ();
}
pub mod importprivkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportprivkeyArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type ImportprivkeyResponse = ();
}
pub mod importpubkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportpubkeyArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type ImportpubkeyResponse = ();
}
pub mod importwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportwalletArguments(String);
    pub type ImportwalletResponse = ();
}
pub mod keypoolrefill {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct KeypoolrefillArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    pub type KeypoolrefillResponse = ();
}
pub mod listaccounts {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListaccountsArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListaccountsResponse {
        pub account: crate::client::utils::ZDecimal,
    }
}
pub mod listaddressgroupings {
    pub type ListaddressgroupingsResponse = Vec<Vec<Vec<String>>>;
}
pub mod listbanned {
    pub type ListbannedResponse = ();
}
pub mod listlockunspent {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListlockunspentElement {
        pub txid: String,
        pub vout: crate::client::utils::ZDecimal,
    }
    pub type ListlockunspentResponse = Vec<ListlockunspentElement>;
}
pub mod listreceivedbyaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListreceivedbyaccountArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListreceivedbyaccountElement {
        pub account: String,
        pub amount: crate::client::utils::ZDecimal,
        pub amount_zat: crate::client::utils::ZDecimal,
        pub confirmations: crate::client::utils::ZDecimal,
        pub involves_watchonly: bool,
    }
    pub type ListreceivedbyaccountResponse = Vec<ListreceivedbyaccountElement>;
}
pub mod listreceivedbyaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListreceivedbyaddressArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListreceivedbyaddressElement {
        pub account: String,
        pub address: String,
        pub amount: crate::client::utils::ZDecimal,
        pub amount_zat: crate::client::utils::ZDecimal,
        pub confirmations: crate::client::utils::ZDecimal,
        pub involves_watchonly: bool,
    }
    pub type ListreceivedbyaddressResponse = Vec<ListreceivedbyaddressElement>;
}
pub mod listsinceblock {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListsinceblockArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListsinceblockResponse {
        pub lastblock: String,
        pub transactions: Vec<String>,
    }
}
pub mod listtransactions {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListtransactionsArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListtransactionsElement {
        pub account: String,
        pub address: String,
        pub amount: crate::client::utils::ZDecimal,
        pub amount_zat: crate::client::utils::ZDecimal,
        pub blockhash: String,
        pub blockindex: crate::client::utils::ZDecimal,
        pub category: String,
        pub comment: String,
        pub confirmations: crate::client::utils::ZDecimal,
        pub fee: crate::client::utils::ZDecimal,
        pub otheraccount: String,
        pub size: crate::client::utils::ZDecimal,
        pub status: String,
        pub time: crate::client::utils::ZDecimal,
        pub timereceived: crate::client::utils::ZDecimal,
        pub txid: String,
        pub vout: crate::client::utils::ZDecimal,
    }
    pub type ListtransactionsResponse = Vec<ListtransactionsElement>;
}
pub mod listunspent {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListunspentArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListunspentElement {
        pub account: String,
        pub address: String,
        pub amount: crate::client::utils::ZDecimal,
        pub amount_zat: crate::client::utils::ZDecimal,
        pub confirmations: crate::client::utils::ZDecimal,
        pub generated: bool,
        pub redeem_script: String,
        pub script_pub_key: String,
        pub spendable: bool,
        pub txid: String,
        pub vout: crate::client::utils::ZDecimal,
    }
    pub type ListunspentResponse = Vec<ListunspentElement>;
}
pub mod lockunspent {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct LockunspentArguments(bool, String);
    pub type LockunspentResponse = bool;
}
pub mod move_mod {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct MoveArguments(
        String,
        String,
        crate::client::utils::ZDecimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type MoveResponse = bool;
}
pub mod ping {
    pub type PingResponse = ();
}
pub mod prioritisetransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PrioritisetransactionArguments(
        String,
        crate::client::utils::ZDecimal,
        crate::client::utils::ZDecimal,
    );
    pub type PrioritisetransactionResponse = bool;
}
pub mod sendfrom {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SendfromArguments(
        String,
        String,
        crate::client::utils::ZDecimal,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type SendfromResponse = String;
}
pub mod sendmany {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SendmanyArguments(
        String,
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type SendmanyResponse = String;
}
pub mod sendrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SendrawtransactionArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type SendrawtransactionResponse = String;
}
pub mod sendtoaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SendtoaddressArguments(
        String,
        crate::client::utils::ZDecimal,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type SendtoaddressResponse = String;
}
pub mod setaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetaccountArguments(String, String);
    pub type SetaccountResponse = ();
}
pub mod setban {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetbanArguments(
        String,
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type SetbanResponse = ();
}
pub mod setgenerate {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetgenerateArguments(
        bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    pub type SetgenerateResponse = ();
}
pub mod setlogfilter {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetlogfilterArguments(String);
    pub type SetlogfilterResponse = ();
}
pub mod settxfee {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SettxfeeArguments(crate::client::utils::ZDecimal);
    pub type SettxfeeResponse = bool;
}
pub mod signmessage {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SignmessageArguments(String, String);
    pub type SignmessageResponse = String;
}
pub mod signrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SignrawtransactionArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Errors {
        pub error: String,
        pub script_sig: String,
        pub sequence: crate::client::utils::ZDecimal,
        pub txid: String,
        pub vout: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SignrawtransactionResponse {
        pub complete: bool,
        pub errors: Vec<Errors>,
        pub hex: String,
    }
}
pub mod stop {
    pub type StopResponse = ();
}
pub mod submitblock {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SubmitblockArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub enum SubmitblockElementResponse {
        #[serde(rename = "duplicate")]
        Duplicate,
        #[serde(rename = "duplicate-invalid")]
        DuplicateInvalid,
        #[serde(rename = "duplicate-inconclusive")]
        DuplicateInconclusive,
        #[serde(rename = "inconclusive")]
        Inconclusive,
        #[serde(rename = "rejected")]
        Rejected,
    }
}
pub mod validateaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ValidateaddressArguments(String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ValidateaddressResponse {
        pub account: String,
        pub address: String,
        pub iscompressed: bool,
        pub ismine: bool,
        pub isscript: bool,
        pub isvalid: bool,
        pub pubkey: String,
        pub script_pub_key: String,
    }
}
pub mod verifychain {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct VerifychainArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    pub type VerifychainResponse = bool;
}
pub mod verifymessage {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct VerifymessageArguments(String, String, String);
    pub type VerifymessageResponse = bool;
}
pub mod verifytxoutproof {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct VerifytxoutproofArguments(String);
    pub type VerifytxoutproofResponse = Vec<String>;
}
pub mod z_exportkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZExportkeyArguments(String);
    pub type ZExportkeyResponse = String;
}
pub mod z_exportviewingkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZExportviewingkeyArguments(String);
    pub type ZExportviewingkeyResponse = String;
}
pub mod z_exportwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZExportwalletArguments(String);
    pub type ZExportwalletResponse = String;
}
pub mod z_getbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetbalanceArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type ZGetbalanceResponse = crate::client::utils::ZDecimal;
}
pub mod z_getmigrationstatus {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetmigrationstatusResponse {
        pub time_started: Option<crate::client::utils::ZDecimal>,
        pub destination_address: String,
        pub enabled: bool,
        pub finalized_migrated_amount: crate::client::utils::ZDecimal,
        pub finalized_migration_transactions: crate::client::utils::ZDecimal,
        pub migration_txids: Vec<String>,
        pub unfinalized_migrated_amount: crate::client::utils::ZDecimal,
        pub unmigrated_amount: crate::client::utils::ZDecimal,
    }
}
pub mod z_getnewaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetnewaddressArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type ZGetnewaddressResponse = String;
}
pub mod z_getnotescount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetnotescountArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetnotescountResponse {
        pub sapling: crate::client::utils::ZDecimal,
        pub sprout: crate::client::utils::ZDecimal,
    }
}
pub mod z_getoperationresult {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetoperationresultArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<Vec<String>>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum ZGetoperationresultElement {
        Executing {
            creation_time: crate::client::utils::ZDecimal,
            id: String,
            method: String,
            params: Params,
            status: String,
        },
        Success {
            creation_time: crate::client::utils::ZDecimal,
            execution_secs: crate::client::utils::ZDecimal,
            id: String,
            method: String,
            params: Params,
            result: Result,
            status: String,
        },
        Failed {
            creation_time: crate::client::utils::ZDecimal,
            error: Error,
            id: String,
            method: String,
            params: Params,
            status: String,
        },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Amounts {
        pub address: String,
        pub amount: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Error {
        pub code: crate::client::utils::ZDecimal,
        pub message: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Params {
        pub amounts: Vec<Amounts>,
        pub fee: crate::client::utils::ZDecimal,
        pub fromaddress: String,
        pub minconf: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Result {
        pub txid: String,
    }
    pub type ZGetoperationresultResponse = Vec<ZGetoperationresultElement>;
}
pub mod z_getoperationstatus {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetoperationstatusArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<Vec<String>>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    #[serde(untagged)]
    pub enum ZGetoperationstatusElement {
        Executing {
            creation_time: crate::client::utils::ZDecimal,
            id: String,
            method: String,
            params: Params,
            status: String,
        },
        Success {
            creation_time: crate::client::utils::ZDecimal,
            execution_secs: crate::client::utils::ZDecimal,
            id: String,
            method: String,
            params: Params,
            result: Result,
            status: String,
        },
        Failed {
            creation_time: crate::client::utils::ZDecimal,
            error: Error,
            id: String,
            method: String,
            params: Params,
            status: String,
        },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Amounts {
        pub address: String,
        pub amount: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Error {
        pub code: crate::client::utils::ZDecimal,
        pub message: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Params {
        pub amounts: Vec<Amounts>,
        pub fee: crate::client::utils::ZDecimal,
        pub fromaddress: String,
        pub minconf: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Result {
        pub txid: String,
    }
    pub type ZGetoperationstatusResponse = Vec<ZGetoperationstatusElement>;
}
pub mod z_getpaymentdisclosure {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetpaymentdisclosureArguments(
        String,
        String,
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type ZGetpaymentdisclosureResponse = String;
}
pub mod z_gettotalbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGettotalbalanceArguments(
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGettotalbalanceResponse {
        pub private: crate::client::utils::ZDecimal,
        pub total: crate::client::utils::ZDecimal,
        pub transparent: crate::client::utils::ZDecimal,
    }
}
pub mod z_gettreestate {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGettreestateArguments(String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Commitments {
        pub final_root: String,
        pub final_state: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Sapling {
        pub commitments: Commitments,
        pub skip_hash: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Sprout {
        pub commitments: Commitments,
        pub skip_hash: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGettreestateResponse {
        pub hash: String,
        pub height: crate::client::utils::ZDecimal,
        pub sapling: Sapling,
        pub sprout: Sprout,
    }
}
pub mod z_importkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportkeyArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportkeyResponse {
        pub address: String,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod z_importviewingkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportviewingkeyArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportviewingkeyResponse {
        pub address: String,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod z_importwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportwalletArguments(String);
    pub type ZImportwalletResponse = ();
}
pub mod z_listaddresses {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListaddressesArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    pub type ZListaddressesResponse = Vec<String>;
}
pub mod z_listoperationids {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListoperationidsArguments(
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    pub type ZListoperationidsResponse = Vec<String>;
}
pub mod z_listreceivedbyaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListreceivedbyaddressArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListreceivedbyaddressResponse {
        pub amount: crate::client::utils::ZDecimal,
        pub amount_zat: crate::client::utils::ZDecimal,
        pub blockheight: crate::client::utils::ZDecimal,
        pub blockindex: crate::client::utils::ZDecimal,
        pub blocktime: crate::client::utils::ZDecimal,
        pub change: bool,
        pub confirmations: crate::client::utils::ZDecimal,
        pub jsindex: crate::client::utils::ZDecimal,
        pub jsoutindex: crate::client::utils::ZDecimal,
        pub memo: String,
        pub outindex: crate::client::utils::ZDecimal,
        pub txid: String,
    }
}
pub mod z_listunspent {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListunspentArguments(
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<bool>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListunspentElement {
        pub address: String,
        pub amount: crate::client::utils::ZDecimal,
        pub change: bool,
        pub confirmations: crate::client::utils::ZDecimal,
        pub jsindex: crate::client::utils::ZDecimal,
        pub jsoutindex: crate::client::utils::ZDecimal,
        pub memo: String,
        pub outindex: crate::client::utils::ZDecimal,
        pub spendable: bool,
        pub txid: String,
    }
    pub type ZListunspentResponse = Vec<ZListunspentElement>;
}
pub mod z_mergetoaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZMergetoaddressArguments(
        Vec<String>,
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")] Option<String>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZMergetoaddressResponse {
        pub merging_notes: crate::client::utils::ZDecimal,
        pub merging_shielded_value: crate::client::utils::ZDecimal,
        pub merging_transparent_value: crate::client::utils::ZDecimal,
        pub merging_u_t_x_os: crate::client::utils::ZDecimal,
        pub opid: String,
        pub remaining_notes: crate::client::utils::ZDecimal,
        pub remaining_shielded_value: crate::client::utils::ZDecimal,
        pub remaining_transparent_value: crate::client::utils::ZDecimal,
        pub remaining_u_t_x_os: crate::client::utils::ZDecimal,
    }
}
pub mod z_sendmany {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Amounts {
        pub memo: Option<String>,
        pub address: String,
        pub amount: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZSendmanyArguments(
        String,
        Vec<Amounts>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    pub type ZSendmanyResponse = String;
}
pub mod z_setmigration {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZSetmigrationArguments(bool);
    pub type ZSetmigrationResponse = ();
}
pub mod z_shieldcoinbase {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZShieldcoinbaseArguments(
        String,
        String,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
        #[serde(skip_serializing_if = "Option::is_none")]
        Option<crate::client::utils::ZDecimal>,
    );
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZShieldcoinbaseResponse {
        pub opid: String,
        pub remaining_u_t_x_os: crate::client::utils::ZDecimal,
        pub remaining_value: crate::client::utils::ZDecimal,
        pub shielding_u_t_x_os: crate::client::utils::ZDecimal,
        pub shielding_value: crate::client::utils::ZDecimal,
    }
}
pub mod z_validateaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZValidateaddressArguments(String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZValidateaddressResponse {
        pub address: String,
        pub diversifiedtransmissionkey: String,
        pub diversifier: String,
        pub ismine: bool,
        pub isvalid: bool,
        pub payingkey: String,
        pub transmissionkey: String,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod z_validatepaymentdisclosure {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZValidatepaymentdisclosureArguments(String);
    pub type ZValidatepaymentdisclosureResponse = ();
}
pub mod z_viewtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZViewtransactionArguments(String);
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Outputs {
        pub address: String,
        pub js: crate::client::utils::ZDecimal,
        pub js_output: crate::client::utils::ZDecimal,
        pub memo: String,
        pub memo_str: String,
        pub outgoing: bool,
        pub output: crate::client::utils::ZDecimal,
        #[serde(rename = "type")]
        pub type_field: String,
        pub value: crate::client::utils::ZDecimal,
        pub value_zat: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Spends {
        pub address: String,
        pub js: crate::client::utils::ZDecimal,
        pub js_output_prev: crate::client::utils::ZDecimal,
        pub js_prev: crate::client::utils::ZDecimal,
        pub js_spend: crate::client::utils::ZDecimal,
        pub output_prev: crate::client::utils::ZDecimal,
        pub spend: crate::client::utils::ZDecimal,
        pub txid_prev: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub value: crate::client::utils::ZDecimal,
        pub value_zat: crate::client::utils::ZDecimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZViewtransactionResponse {
        pub outputs: Vec<Outputs>,
        pub spends: Vec<Spends>,
        pub txid: String,
    }
}
pub mod zcbenchmark {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcbenchmarkElement {
        pub runningtime: crate::client::utils::ZDecimal,
    }
    pub type ZcbenchmarkResponse = Vec<ZcbenchmarkElement>;
}
pub mod zcrawjoinsplit {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawjoinsplitResponse {
        pub encryptednote1: String,
        pub encryptednote2: String,
        pub rawtxn: String,
    }
}
pub mod zcrawkeygen {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawkeygenResponse {
        pub zcaddress: String,
        pub zcsecretkey: String,
        pub zcviewingkey: String,
    }
}
pub mod zcrawreceive {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawreceiveResponse {
        pub amount: crate::client::utils::ZDecimal,
        pub exists: bool,
        pub note: String,
    }
}
pub mod zcsamplejoinsplit {
    pub type ZcsamplejoinsplitResponse = ();
}
