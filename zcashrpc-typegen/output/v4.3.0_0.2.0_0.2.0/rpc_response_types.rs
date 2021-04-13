//procedurally generated response types, note that zcashrpc-typegen
//is in early alpha, and output is subject to change at any time.
pub mod addmultisigaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AddmultisigaddressArguments {
        pub three_account: Option<String>,
        pub one_nrequired: rust_decimal::Decimal,
        pub two_keysobject: String,
    }
    pub type AddmultisigaddressResponse = String;
}
pub mod addnode {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AddnodeArguments {
        pub one_node: String,
        pub two_command: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct AddnodeResponse;
}
pub mod backupwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct BackupwalletArguments {
        pub one_destination: String,
    }
    pub type BackupwalletResponse = String;
}
pub mod clearbanned {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ClearbannedArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ClearbannedResponse;
}
pub mod createmultisig {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct CreatemultisigArguments {
        pub one_nrequired: rust_decimal::Decimal,
        pub two_keys: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct CreatemultisigResponse {
        pub address: String,
        pub redeem_script: String,
    }
}
pub mod createrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct CreaterawtransactionArguments {
        pub four_expiryheight: Option<rust_decimal::Decimal>,
        pub three_locktime: Option<rust_decimal::Decimal>,
        pub one_transactions: String,
        pub two_addresses: String,
    }
    pub type CreaterawtransactionResponse = String;
}
pub mod decoderawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DecoderawtransactionArguments {
        pub one_hex: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DecoderawtransactionResponse {
        pub expiryheight: Option<rust_decimal::Decimal>,
        pub versiongroupid: Option<String>,
        pub locktime: rust_decimal::Decimal,
        pub overwintered: bool,
        pub size: rust_decimal::Decimal,
        pub txid: String,
        pub version: rust_decimal::Decimal,
        pub vin: Vec<Vin>,
        pub vjoinsplit: Vec<Vjoinsplit>,
        pub vout: Vec<Vout>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptPubKey {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub req_sigs: rust_decimal::Decimal,
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
        pub sequence: rust_decimal::Decimal,
        pub txid: String,
        pub vout: rust_decimal::Decimal,
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
        pub vpub_new: rust_decimal::Decimal,
        pub vpub_old: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vout {
        pub n: rust_decimal::Decimal,
        pub script_pub_key: ScriptPubKey,
        pub value: rust_decimal::Decimal,
    }
}
pub mod decodescript {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DecodescriptArguments {
        pub one_hex: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DecodescriptResponse {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub p2sh: String,
        pub req_sigs: rust_decimal::Decimal,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod disconnectnode {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DisconnectnodeArguments {
        pub one_node: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DisconnectnodeResponse;
}
pub mod dumpprivkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DumpprivkeyArguments {
        pub one_t_addr: String,
    }
    pub type DumpprivkeyResponse = String;
}
pub mod dumpwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct DumpwalletArguments {
        pub one_filename: String,
    }
    pub type DumpwalletResponse = String;
}
pub mod encryptwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct EncryptwalletArguments {
        pub one_passphrase: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct EncryptwalletResponse;
}
pub mod estimatefee {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct EstimatefeeArguments {
        pub one_nblocks: rust_decimal::Decimal,
    }
    pub type EstimatefeeResponse = rust_decimal::Decimal;
}
pub mod estimatepriority {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct EstimatepriorityArguments {
        pub one_nblocks: rust_decimal::Decimal,
    }
    pub type EstimatepriorityResponse = rust_decimal::Decimal;
}
pub mod fundrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct FundrawtransactionArguments {
        pub two_include_watching: Option<bool>,
        pub one_hexstring: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct FundrawtransactionResponse {
        pub changepos: rust_decimal::Decimal,
        pub fee: rust_decimal::Decimal,
        pub hex: String,
    }
}
pub mod generate {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GenerateArguments {
        pub one_numblocks: rust_decimal::Decimal,
    }
    pub type GenerateResponse = Vec<String>;
}
pub mod getaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaccountArguments {
        pub one_zcashaddress: String,
    }
    pub type GetaccountResponse = String;
}
pub mod getaccountaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaccountaddressArguments {
        pub one_account: String,
    }
    pub type GetaccountaddressResponse = String;
}
pub mod getaddednodeinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddednodeinfoArguments {
        pub two_node: Option<String>,
        pub one_dns: bool,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Addresses {
        pub address: String,
        pub connected: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Getaddednodeinfo {
        pub addednode: String,
        pub addresses: Vec<Addresses>,
        pub connected: bool,
    }
    pub type GetaddednodeinfoResponse = Vec<Getaddednodeinfo>;
}
pub mod getaddressbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressbalanceArguments {
        pub one: One,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressbalanceArguments {
        pub one_address: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct One {
        pub addresses: Vec<String>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressbalanceResponse {
        pub balance: String,
        pub received: String,
    }
}
pub mod getaddressdeltas {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressdeltasArguments {
        pub one: One,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressdeltasArguments {
        pub one_address: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct One {
        pub chain_info: Option<bool>,
        pub end: Option<rust_decimal::Decimal>,
        pub start: Option<rust_decimal::Decimal>,
        pub addresses: Vec<String>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
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
        pub height: rust_decimal::Decimal,
        pub index: rust_decimal::Decimal,
        pub satoshis: rust_decimal::Decimal,
        pub txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct End {
        pub hash: String,
        pub height: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Regular {
        pub address: String,
        pub height: rust_decimal::Decimal,
        pub index: rust_decimal::Decimal,
        pub satoshis: rust_decimal::Decimal,
        pub txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Start {
        pub hash: String,
        pub height: rust_decimal::Decimal,
    }
}
pub mod getaddressesbyaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressesbyaccountArguments {
        pub one_account: String,
    }
    pub type GetaddressesbyaccountResponse = Vec<String>;
}
pub mod getaddressmempool {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressmempoolArguments {
        pub one: One,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressmempoolArguments {
        pub one_address: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct One {
        pub addresses: Vec<String>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Getaddressmempool {
        pub address: String,
        pub index: rust_decimal::Decimal,
        pub prevout: String,
        pub prevtxid: String,
        pub satoshis: rust_decimal::Decimal,
        pub timestamp: rust_decimal::Decimal,
        pub txid: String,
    }
    pub type GetaddressmempoolResponse = Vec<Getaddressmempool>;
}
pub mod getaddresstxids {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddresstxidsArguments {
        pub one: One,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddresstxidsArguments {
        pub one_address: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct One {
        pub end: Option<rust_decimal::Decimal>,
        pub start: Option<rust_decimal::Decimal>,
        pub addresses: Vec<String>,
    }
    pub type GetaddresstxidsResponse = Vec<String>;
}
pub mod getaddressutxos {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressutxosArguments {
        pub one: One,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetaddressutxosArguments {
        pub one_address: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct One {
        pub chain_info: Option<bool>,
        pub addresses: Vec<String>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub enum GetaddressutxosResponse {
        Regular(Vec<Regular>),
        Verbose {
            hash: String,
            height: rust_decimal::Decimal,
            utxos: Vec<Utxos>,
        },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Regular {
        pub address: String,
        pub height: rust_decimal::Decimal,
        pub output_index: rust_decimal::Decimal,
        pub satoshis: rust_decimal::Decimal,
        pub script: String,
        pub txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Utxos {
        pub address: String,
        pub height: rust_decimal::Decimal,
        pub output_index: rust_decimal::Decimal,
        pub satoshis: rust_decimal::Decimal,
        pub script: String,
        pub txid: String,
    }
}
pub mod getbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetbalanceArguments {
        pub four_in_zat: Option<bool>,
        pub one_account: Option<String>,
        pub three_include_watchonly: Option<bool>,
        pub two_minconf: Option<rust_decimal::Decimal>,
    }
    pub type GetbalanceResponse = rust_decimal::Decimal;
}
pub mod getbestblockhash {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetbestblockhashArguments;
    pub type GetbestblockhashResponse = String;
}
pub mod getblock {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockArguments {
        pub two_verbosity: Option<rust_decimal::Decimal>,
        pub one_hash_or_height: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub enum GetblockResponse {
        Regular(String),
        Verbose {
            bits: String,
            confirmations: rust_decimal::Decimal,
            difficulty: rust_decimal::Decimal,
            finalsaplingroot: String,
            hash: String,
            height: rust_decimal::Decimal,
            merkleroot: String,
            nextblockhash: String,
            nonce: rust_decimal::Decimal,
            previousblockhash: String,
            size: rust_decimal::Decimal,
            time: rust_decimal::Decimal,
            tx: Vec<String>,
            version: rust_decimal::Decimal,
        },
        VeryVerbose {
            bits: String,
            confirmations: rust_decimal::Decimal,
            difficulty: rust_decimal::Decimal,
            finalsaplingroot: String,
            hash: String,
            height: rust_decimal::Decimal,
            merkleroot: String,
            nextblockhash: String,
            nonce: rust_decimal::Decimal,
            previousblockhash: String,
            size: rust_decimal::Decimal,
            time: rust_decimal::Decimal,
            tx: Vec<Tx>,
            version: rust_decimal::Decimal,
        },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptPubKey {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub req_sigs: rust_decimal::Decimal,
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
        pub expiryheight: Option<rust_decimal::Decimal>,
        pub blockhash: String,
        pub blocktime: rust_decimal::Decimal,
        pub confirmations: rust_decimal::Decimal,
        pub hex: String,
        pub in_active_chain: bool,
        pub locktime: rust_decimal::Decimal,
        pub size: rust_decimal::Decimal,
        pub time: rust_decimal::Decimal,
        pub txid: String,
        pub version: rust_decimal::Decimal,
        pub vin: Vec<Vin>,
        pub vjoinsplit: Vec<Vjoinsplit>,
        pub vout: Vec<Vout>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vin {
        pub script_sig: ScriptSig,
        pub sequence: rust_decimal::Decimal,
        pub txid: String,
        pub vout: rust_decimal::Decimal,
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
        pub vpub_new: rust_decimal::Decimal,
        pub vpub_old: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vout {
        pub n: rust_decimal::Decimal,
        pub script_pub_key: ScriptPubKey,
        pub value: rust_decimal::Decimal,
    }
}
pub mod getblockchaininfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockchaininfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Consensus {
        pub chaintip: String,
        pub nextblock: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Enforce {
        pub found: rust_decimal::Decimal,
        pub required: rust_decimal::Decimal,
        pub status: bool,
        pub window: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockchaininfoResponse {
        pub bestblockhash: String,
        pub blocks: rust_decimal::Decimal,
        pub chain: String,
        pub chainwork: String,
        pub commitments: rust_decimal::Decimal,
        pub consensus: Consensus,
        pub difficulty: rust_decimal::Decimal,
        pub estimatedheight: rust_decimal::Decimal,
        pub headers: rust_decimal::Decimal,
        pub initial_block_download_complete: bool,
        pub size_on_disk: rust_decimal::Decimal,
        pub softforks: Vec<Softforks>,
        pub upgrades: std::collections::HashMap<String, Upgrades>,
        pub verificationprogress: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Reject {
        pub found: rust_decimal::Decimal,
        pub required: rust_decimal::Decimal,
        pub status: bool,
        pub window: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Softforks {
        pub enforce: Enforce,
        pub id: String,
        pub reject: Reject,
        pub version: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Upgrades {
        pub activationheight: rust_decimal::Decimal,
        pub info: String,
        pub name: String,
        pub status: String,
    }
}
pub mod getblockcount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockcountArguments;
    pub type GetblockcountResponse = rust_decimal::Decimal;
}
pub mod getblockdeltas {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockdeltasArguments {
        pub one_hash: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Deltas {
        pub index: rust_decimal::Decimal,
        pub inputs: Vec<Inputs>,
        pub outputs: Vec<Outputs>,
        pub txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockdeltasResponse {
        pub bits: String,
        pub chainwork: String,
        pub confirmations: rust_decimal::Decimal,
        pub deltas: Vec<Deltas>,
        pub difficulty: rust_decimal::Decimal,
        pub hash: String,
        pub height: rust_decimal::Decimal,
        pub mediantime: rust_decimal::Decimal,
        pub merkleroot: String,
        pub nextblockhash: String,
        pub nonce: String,
        pub previousblockhash: String,
        pub size: rust_decimal::Decimal,
        pub time: rust_decimal::Decimal,
        pub version: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Inputs {
        pub address: String,
        pub index: rust_decimal::Decimal,
        pub prevout: rust_decimal::Decimal,
        pub prevtxid: String,
        pub satoshis: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Outputs {
        pub address: String,
        pub index: rust_decimal::Decimal,
        pub satoshis: rust_decimal::Decimal,
    }
}
pub mod getblockhash {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockhashArguments {
        pub one_index: rust_decimal::Decimal,
    }
    pub type GetblockhashResponse = String;
}
pub mod getblockhashes {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockhashesArguments {
        pub three_options: Option<String>,
        pub one_high: rust_decimal::Decimal,
        pub two_low: rust_decimal::Decimal,
    }
    pub type GetblockhashesResponse = Vec<String>;
}
pub mod getblockheader {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblockheaderArguments {
        pub two_verbose: Option<bool>,
        pub one_hash: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub enum GetblockheaderResponse {
        Regular(String),
        Verbose {
            bits: String,
            confirmations: rust_decimal::Decimal,
            difficulty: rust_decimal::Decimal,
            finalsaplingroot: String,
            hash: String,
            height: rust_decimal::Decimal,
            merkleroot: String,
            nextblockhash: String,
            nonce: rust_decimal::Decimal,
            previousblockhash: String,
            time: rust_decimal::Decimal,
            version: rust_decimal::Decimal,
        },
    }
}
pub mod getblocksubsidy {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblocksubsidyArguments {
        pub one_height: Option<rust_decimal::Decimal>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Fundingstreams {
        pub address: String,
        pub recipient: String,
        pub specification: String,
        pub value: rust_decimal::Decimal,
        pub value_zat: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetblocksubsidyResponse {
        pub founders: rust_decimal::Decimal,
        pub fundingstreams: Vec<Fundingstreams>,
        pub miner: rust_decimal::Decimal,
    }
}
pub mod getchaintips {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetchaintipsArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Getchaintips {
        pub branchlen: rust_decimal::Decimal,
        pub hash: String,
        pub height: rust_decimal::Decimal,
        pub status: String,
    }
    pub type GetchaintipsResponse = Vec<Getchaintips>;
}
pub mod getconnectioncount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetconnectioncountArguments;
    pub type GetconnectioncountResponse = rust_decimal::Decimal;
}
pub mod getdeprecationinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetdeprecationinfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetdeprecationinfoResponse {
        pub deprecationheight: rust_decimal::Decimal,
        pub subversion: String,
        pub version: rust_decimal::Decimal,
    }
}
pub mod getdifficulty {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetdifficultyArguments;
    pub type GetdifficultyResponse = rust_decimal::Decimal;
}
pub mod getexperimentalfeatures {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetexperimentalfeaturesArguments;
    pub type GetexperimentalfeaturesResponse = Vec<String>;
}
pub mod getgenerate {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetgenerateArguments;
    pub type GetgenerateResponse = bool;
}
pub mod getinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetinfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetinfoResponse {
        pub proxy: Option<String>,
        pub balance: rust_decimal::Decimal,
        pub blocks: rust_decimal::Decimal,
        pub connections: rust_decimal::Decimal,
        pub difficulty: rust_decimal::Decimal,
        pub errors: String,
        pub keypoololdest: rust_decimal::Decimal,
        pub keypoolsize: rust_decimal::Decimal,
        pub paytxfee: rust_decimal::Decimal,
        pub protocolversion: rust_decimal::Decimal,
        pub relayfee: rust_decimal::Decimal,
        pub testnet: bool,
        pub timeoffset: rust_decimal::Decimal,
        pub unlocked_until: rust_decimal::Decimal,
        pub version: rust_decimal::Decimal,
        pub walletversion: rust_decimal::Decimal,
    }
}
pub mod getlocalsolps {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetlocalsolpsArguments;
    pub type GetlocalsolpsResponse = rust_decimal::Decimal;
}
pub mod getmemoryinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmemoryinfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmemoryinfoResponse {
        pub locked: Locked,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Locked {
        pub chunks_free: rust_decimal::Decimal,
        pub chunks_used: rust_decimal::Decimal,
        pub free: rust_decimal::Decimal,
        pub locked: rust_decimal::Decimal,
        pub total: rust_decimal::Decimal,
        pub used: rust_decimal::Decimal,
    }
}
pub mod getmempoolinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmempoolinfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmempoolinfoResponse {
        pub bytes: rust_decimal::Decimal,
        pub size: rust_decimal::Decimal,
        pub usage: rust_decimal::Decimal,
    }
}
pub mod getmininginfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmininginfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetmininginfoResponse {
        pub blocks: rust_decimal::Decimal,
        pub chain: String,
        pub currentblocksize: rust_decimal::Decimal,
        pub currentblocktx: rust_decimal::Decimal,
        pub difficulty: rust_decimal::Decimal,
        pub errors: String,
        pub generate: bool,
        pub genproclimit: rust_decimal::Decimal,
        pub localsolps: rust_decimal::Decimal,
        pub networksolps: rust_decimal::Decimal,
        pub pooledtx: rust_decimal::Decimal,
        pub testnet: bool,
    }
}
pub mod getnettotals {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnettotalsArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnettotalsResponse {
        pub timemillis: rust_decimal::Decimal,
        pub totalbytesrecv: rust_decimal::Decimal,
        pub totalbytessent: rust_decimal::Decimal,
        pub uploadtarget: Uploadtarget,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Uploadtarget {
        pub bytes_left_in_cycle: rust_decimal::Decimal,
        pub serve_historical_blocks: bool,
        pub target: rust_decimal::Decimal,
        pub target_reached: bool,
        pub time_left_in_cycle: rust_decimal::Decimal,
        pub timeframe: rust_decimal::Decimal,
    }
}
pub mod getnetworkhashps {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnetworkhashpsArguments {
        pub one_blocks: Option<rust_decimal::Decimal>,
        pub two_height: Option<rust_decimal::Decimal>,
    }
    pub type GetnetworkhashpsResponse = rust_decimal::Decimal;
}
pub mod getnetworkinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnetworkinfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnetworkinfoResponse {
        pub connections: rust_decimal::Decimal,
        pub localaddresses: Vec<Localaddresses>,
        pub localservices: String,
        pub networks: Vec<Networks>,
        pub protocolversion: rust_decimal::Decimal,
        pub relayfee: rust_decimal::Decimal,
        pub subversion: String,
        pub timeoffset: rust_decimal::Decimal,
        pub version: rust_decimal::Decimal,
        pub warnings: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Localaddresses {
        pub address: String,
        pub port: rust_decimal::Decimal,
        pub score: rust_decimal::Decimal,
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
    pub struct GetnetworksolpsArguments {
        pub one_blocks: Option<rust_decimal::Decimal>,
        pub two_height: Option<rust_decimal::Decimal>,
    }
    pub type GetnetworksolpsResponse = rust_decimal::Decimal;
}
pub mod getnewaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetnewaddressArguments {
        pub one_account: Option<String>,
    }
    pub type GetnewaddressResponse = String;
}
pub mod getpeerinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetpeerinfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Getpeerinfo {
        pub addr: String,
        pub addrlocal: String,
        pub banscore: rust_decimal::Decimal,
        pub bytesrecv: rust_decimal::Decimal,
        pub bytessent: rust_decimal::Decimal,
        pub conntime: rust_decimal::Decimal,
        pub id: rust_decimal::Decimal,
        pub inbound: bool,
        pub inflight: Vec<rust_decimal::Decimal>,
        pub lastrecv: rust_decimal::Decimal,
        pub lastsend: rust_decimal::Decimal,
        pub pingtime: rust_decimal::Decimal,
        pub pingwait: rust_decimal::Decimal,
        pub services: String,
        pub startingheight: rust_decimal::Decimal,
        pub subver: String,
        pub synced_blocks: rust_decimal::Decimal,
        pub synced_headers: rust_decimal::Decimal,
        pub timeoffset: rust_decimal::Decimal,
        pub version: rust_decimal::Decimal,
    }
    pub type GetpeerinfoResponse = Vec<Getpeerinfo>;
}
pub mod getrawchangeaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetrawchangeaddressArguments;
    pub type GetrawchangeaddressResponse = String;
}
pub mod getrawmempool {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetrawmempoolArguments {
        pub one_verbose: Option<bool>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub enum GetrawmempoolResponse {
        Regular(Vec<String>),
        Verbose { transactionid: Transactionid },
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Transactionid {
        pub currentpriority: rust_decimal::Decimal,
        pub depends: Vec<String>,
        pub fee: rust_decimal::Decimal,
        pub height: rust_decimal::Decimal,
        pub size: rust_decimal::Decimal,
        pub startingpriority: rust_decimal::Decimal,
        pub time: rust_decimal::Decimal,
    }
}
pub mod getrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetrawtransactionArguments {
        pub three_blockhash: Option<String>,
        pub two_verbose: Option<rust_decimal::Decimal>,
        pub one_txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub enum GetrawtransactionResponse {
        Regular(String),
        Verbose {
            expiryheight: Option<rust_decimal::Decimal>,
            blockhash: String,
            blocktime: rust_decimal::Decimal,
            confirmations: rust_decimal::Decimal,
            hex: String,
            in_active_chain: bool,
            locktime: rust_decimal::Decimal,
            size: rust_decimal::Decimal,
            time: rust_decimal::Decimal,
            txid: String,
            version: rust_decimal::Decimal,
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
        pub req_sigs: rust_decimal::Decimal,
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
        pub sequence: rust_decimal::Decimal,
        pub txid: String,
        pub vout: rust_decimal::Decimal,
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
        pub vpub_new: rust_decimal::Decimal,
        pub vpub_old: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vout {
        pub n: rust_decimal::Decimal,
        pub script_pub_key: ScriptPubKey,
        pub value: rust_decimal::Decimal,
    }
}
pub mod getreceivedbyaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetreceivedbyaccountArguments {
        pub three_in_zat: Option<bool>,
        pub two_minconf: Option<rust_decimal::Decimal>,
        pub one_account: String,
    }
    pub type GetreceivedbyaccountResponse = rust_decimal::Decimal;
}
pub mod getreceivedbyaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetreceivedbyaddressArguments {
        pub three_in_zat: Option<bool>,
        pub two_minconf: Option<rust_decimal::Decimal>,
        pub one_zcashaddress: String,
    }
    pub type GetreceivedbyaddressResponse = rust_decimal::Decimal;
}
pub mod getspentinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetspentinfoArguments {}
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetspentinfoResponse {
        pub index: rust_decimal::Decimal,
        pub txid: String,
    }
}
pub mod gettransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettransactionArguments {
        pub two_include_watchonly: Option<bool>,
        pub one_txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Details {
        pub account: String,
        pub address: String,
        pub amount: rust_decimal::Decimal,
        pub amount_zat: rust_decimal::Decimal,
        pub category: String,
        pub vout: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettransactionResponse {
        pub amount: rust_decimal::Decimal,
        pub amount_zat: rust_decimal::Decimal,
        pub blockhash: String,
        pub blockindex: rust_decimal::Decimal,
        pub blocktime: rust_decimal::Decimal,
        pub confirmations: rust_decimal::Decimal,
        pub details: Vec<Details>,
        pub hex: String,
        pub status: String,
        pub time: rust_decimal::Decimal,
        pub timereceived: rust_decimal::Decimal,
        pub txid: String,
        pub vjoinsplit: Vec<Vjoinsplit>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Vjoinsplit {
        pub anchor: String,
        pub commitments: Vec<String>,
        pub macs: Vec<String>,
        pub nullifiers: Vec<String>,
        pub vpub_new: rust_decimal::Decimal,
        pub vpub_old: rust_decimal::Decimal,
    }
}
pub mod gettxout {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutArguments {
        pub three_includemempool: Option<bool>,
        pub one_txid: String,
        pub two_n: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutResponse {
        pub bestblock: String,
        pub coinbase: bool,
        pub confirmations: rust_decimal::Decimal,
        pub script_pub_key: ScriptPubKey,
        pub value: rust_decimal::Decimal,
        pub version: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ScriptPubKey {
        pub addresses: Vec<String>,
        pub asm: String,
        pub hex: String,
        pub req_sigs: rust_decimal::Decimal,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod gettxoutproof {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutproofArguments {
        pub two_block: Option<String>,
        pub one_txids: String,
    }
    pub type GettxoutproofResponse = String;
}
pub mod gettxoutsetinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutsetinfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GettxoutsetinfoResponse {
        pub bestblock: String,
        pub bytes_serialized: rust_decimal::Decimal,
        pub hash_serialized: String,
        pub height: rust_decimal::Decimal,
        pub total_amount: rust_decimal::Decimal,
        pub transactions: rust_decimal::Decimal,
        pub txouts: rust_decimal::Decimal,
    }
}
pub mod getunconfirmedbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetunconfirmedbalanceArguments;
    pub type GetunconfirmedbalanceResponse = rust_decimal::Decimal;
}
pub mod getwalletinfo {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetwalletinfoArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct GetwalletinfoResponse {
        pub balance: rust_decimal::Decimal,
        pub immature_balance: rust_decimal::Decimal,
        pub keypoololdest: rust_decimal::Decimal,
        pub keypoolsize: rust_decimal::Decimal,
        pub paytxfee: rust_decimal::Decimal,
        pub seedfp: String,
        pub shielded_balance: rust_decimal::Decimal,
        pub shielded_unconfirmed_balance: rust_decimal::Decimal,
        pub txcount: rust_decimal::Decimal,
        pub unconfirmed_balance: rust_decimal::Decimal,
        pub unlocked_until: rust_decimal::Decimal,
        pub walletversion: rust_decimal::Decimal,
    }
}
pub mod help {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct HelpArguments {
        pub one_command: Option<String>,
    }
    pub type HelpResponse = String;
}
pub mod importaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportaddressArguments {
        pub four_ptwosh: Option<bool>,
        pub three_rescan: Option<bool>,
        pub two_label: Option<String>,
        pub one_script: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportaddressResponse;
}
pub mod importprivkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportprivkeyArguments {
        pub three_rescan: Option<bool>,
        pub two_label: Option<String>,
        pub one_zcashprivkey: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportprivkeyResponse;
}
pub mod importpubkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportpubkeyArguments {
        pub three_rescan: Option<bool>,
        pub two_label: Option<String>,
        pub one_pubkey: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportpubkeyResponse;
}
pub mod importwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportwalletArguments {
        pub one_filename: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ImportwalletResponse;
}
pub mod keypoolrefill {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct KeypoolrefillArguments {
        pub one_newsize: Option<rust_decimal::Decimal>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct KeypoolrefillResponse;
}
pub mod listaccounts {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListaccountsArguments {
        pub one_minconf: Option<rust_decimal::Decimal>,
        pub two_include_watchonly: Option<bool>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListaccountsResponse {
        pub account: rust_decimal::Decimal,
    }
}
pub mod listaddressgroupings {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListaddressgroupingsArguments;
    pub type ListaddressgroupingsResponse = Vec<Vec<Vec<String>>>;
}
pub mod listbanned {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListbannedArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListbannedResponse;
}
pub mod listlockunspent {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListlockunspentArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Listlockunspent {
        pub txid: String,
        pub vout: rust_decimal::Decimal,
    }
    pub type ListlockunspentResponse = Vec<Listlockunspent>;
}
pub mod listreceivedbyaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListreceivedbyaccountArguments {
        pub one_minconf: Option<rust_decimal::Decimal>,
        pub three_include_watchonly: Option<bool>,
        pub two_includeempty: Option<bool>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Listreceivedbyaccount {
        pub account: String,
        pub amount: rust_decimal::Decimal,
        pub amount_zat: rust_decimal::Decimal,
        pub confirmations: rust_decimal::Decimal,
        pub involves_watchonly: bool,
    }
    pub type ListreceivedbyaccountResponse = Vec<Listreceivedbyaccount>;
}
pub mod listreceivedbyaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListreceivedbyaddressArguments {
        pub one_minconf: Option<rust_decimal::Decimal>,
        pub three_include_watchonly: Option<bool>,
        pub two_includeempty: Option<rust_decimal::Decimal>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Listreceivedbyaddress {
        pub account: String,
        pub address: String,
        pub amount: rust_decimal::Decimal,
        pub amount_zat: rust_decimal::Decimal,
        pub confirmations: rust_decimal::Decimal,
        pub involves_watchonly: bool,
    }
    pub type ListreceivedbyaddressResponse = Vec<Listreceivedbyaddress>;
}
pub mod listsinceblock {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListsinceblockArguments {
        pub one_blockhash: Option<String>,
        pub three_include_watchonly: Option<bool>,
        pub two_target_confirmations: Option<rust_decimal::Decimal>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListsinceblockResponse {
        pub lastblock: String,
        pub transactions: Vec<String>,
    }
}
pub mod listtransactions {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListtransactionsArguments {
        pub four_include_watchonly: Option<bool>,
        pub one_account: Option<String>,
        pub three_from: Option<rust_decimal::Decimal>,
        pub two_count: Option<rust_decimal::Decimal>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Listtransactions {
        pub account: String,
        pub address: String,
        pub amount: rust_decimal::Decimal,
        pub amount_zat: rust_decimal::Decimal,
        pub blockhash: String,
        pub blockindex: rust_decimal::Decimal,
        pub category: String,
        pub comment: String,
        pub confirmations: rust_decimal::Decimal,
        pub fee: rust_decimal::Decimal,
        pub otheraccount: String,
        pub size: rust_decimal::Decimal,
        pub status: String,
        pub time: rust_decimal::Decimal,
        pub timereceived: rust_decimal::Decimal,
        pub txid: String,
        pub vout: rust_decimal::Decimal,
    }
    pub type ListtransactionsResponse = Vec<Listtransactions>;
}
pub mod listunspent {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ListunspentArguments {
        pub one_minconf: Option<rust_decimal::Decimal>,
        pub two_maxconf: Option<rust_decimal::Decimal>,
        pub three_addresses: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Listunspent {
        pub account: String,
        pub address: String,
        pub amount: rust_decimal::Decimal,
        pub amount_zat: rust_decimal::Decimal,
        pub confirmations: rust_decimal::Decimal,
        pub generated: bool,
        pub redeem_script: String,
        pub script_pub_key: String,
        pub spendable: bool,
        pub txid: String,
        pub vout: rust_decimal::Decimal,
    }
    pub type ListunspentResponse = Vec<Listunspent>;
}
pub mod lockunspent {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct LockunspentArguments {
        pub one_unlock: bool,
        pub two_transactions: String,
    }
    pub type LockunspentResponse = bool;
}
pub mod move_mod {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct MoveArguments {
        pub five_comment: Option<String>,
        pub four_minconf: Option<rust_decimal::Decimal>,
        pub one_fromaccount: String,
        pub three_amount: rust_decimal::Decimal,
        pub two_toaccount: String,
    }
    pub type MoveResponse = bool;
}
pub mod ping {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PingArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PingResponse;
}
pub mod prioritisetransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct PrioritisetransactionArguments {
        pub one_txid: String,
        pub three_fee: rust_decimal::Decimal,
        pub two_priority: rust_decimal::Decimal,
    }
    pub type PrioritisetransactionResponse = bool;
}
pub mod sendfrom {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SendfromArguments {
        pub _comment_to: Option<String>,
        pub five_comment: Option<String>,
        pub four_minconf: Option<rust_decimal::Decimal>,
        pub one_fromaccount: String,
        pub three_amount: rust_decimal::Decimal,
        pub two_tozcashaddress: String,
    }
    pub type SendfromResponse = String;
}
pub mod sendmany {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SendmanyArguments {
        pub five_subtractfeefromamount: Option<String>,
        pub four_comment: Option<String>,
        pub three_minconf: Option<rust_decimal::Decimal>,
        pub one_fromaccount: String,
        pub two_amounts: String,
    }
    pub type SendmanyResponse = String;
}
pub mod sendrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SendrawtransactionArguments {
        pub two_allowhighfees: Option<bool>,
        pub one_hexstring: String,
    }
    pub type SendrawtransactionResponse = String;
}
pub mod sendtoaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SendtoaddressArguments {
        pub five_subtractfeefromamount: Option<bool>,
        pub four_comment_to: Option<String>,
        pub three_comment: Option<String>,
        pub one_zcashaddress: String,
        pub two_amount: rust_decimal::Decimal,
    }
    pub type SendtoaddressResponse = String;
}
pub mod setaccount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetaccountArguments {
        pub one_zcashaddress: String,
        pub two_account: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetaccountResponse;
}
pub mod setban {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetbanArguments {
        pub four_absolute: Option<bool>,
        pub three_bantime: Option<rust_decimal::Decimal>,
        pub one_ip_optionnetmask: String,
        pub two_command: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetbanResponse;
}
pub mod setgenerate {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetgenerateArguments {
        pub two_genproclimit: Option<rust_decimal::Decimal>,
        pub one_generate: bool,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetgenerateResponse;
}
pub mod setlogfilter {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetlogfilterArguments {
        pub one_new_filter_directives: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SetlogfilterResponse;
}
pub mod settxfee {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SettxfeeArguments {
        pub one_amount: rust_decimal::Decimal,
    }
    pub type SettxfeeResponse = bool;
}
pub mod signmessage {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SignmessageArguments {
        pub one_t_addr: String,
        pub two_message: String,
    }
    pub type SignmessageResponse = String;
}
pub mod signrawtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SignrawtransactionArguments {
        pub five_branchid: Option<String>,
        pub four_sighashtype: Option<String>,
        pub three_privatekeys: Option<String>,
        pub two_prevtxs: Option<String>,
        pub one_hexstring: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Errors {
        pub error: String,
        pub script_sig: String,
        pub sequence: rust_decimal::Decimal,
        pub txid: String,
        pub vout: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SignrawtransactionResponse {
        pub complete: bool,
        pub errors: Vec<Errors>,
        pub hex: String,
    }
}
pub mod stop {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct StopArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct StopResponse;
}
pub mod submitblock {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct SubmitblockArguments {
        pub one_hexdata: String,
        pub two__optionjsonparametersobject: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub enum SubmitblockResponse {
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
    pub struct ValidateaddressArguments {
        pub one_zcashaddress: String,
    }
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
    pub struct VerifychainArguments {
        pub one_checklevel: Option<rust_decimal::Decimal>,
        pub two_numblocks: Option<rust_decimal::Decimal>,
    }
    pub type VerifychainResponse = bool;
}
pub mod verifymessage {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct VerifymessageArguments {
        pub one_zcashaddress: String,
        pub three_message: String,
        pub two_signature: String,
    }
    pub type VerifymessageResponse = bool;
}
pub mod verifytxoutproof {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct VerifytxoutproofArguments {
        pub one_proof: String,
    }
    pub type VerifytxoutproofResponse = Vec<String>;
}
pub mod z_exportkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZExportkeyArguments {
        pub one_zaddr: String,
    }
    pub type ZExportkeyResponse = String;
}
pub mod z_exportviewingkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZExportviewingkeyArguments {
        pub one_zaddr: String,
    }
    pub type ZExportviewingkeyResponse = String;
}
pub mod z_exportwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZExportwalletArguments {
        pub one_filename: String,
    }
    pub type ZExportwalletResponse = String;
}
pub mod z_getbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetbalanceArguments {
        pub three_in_zat: Option<bool>,
        pub two_minconf: Option<rust_decimal::Decimal>,
        pub one_address: String,
    }
    pub type ZGetbalanceResponse = rust_decimal::Decimal;
}
pub mod z_getmigrationstatus {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetmigrationstatusArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetmigrationstatusResponse {
        pub time_started: Option<rust_decimal::Decimal>,
        pub destination_address: String,
        pub enabled: bool,
        pub finalized_migrated_amount: rust_decimal::Decimal,
        pub finalized_migration_transactions: rust_decimal::Decimal,
        pub migration_txids: Vec<String>,
        pub unfinalized_migrated_amount: rust_decimal::Decimal,
        pub unmigrated_amount: rust_decimal::Decimal,
    }
}
pub mod z_getnewaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetnewaddressArguments {
        pub one_type: Option<String>,
    }
    pub type ZGetnewaddressResponse = String;
}
pub mod z_getnotescount {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetnotescountArguments {
        pub one_minconf: Option<rust_decimal::Decimal>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetnotescountResponse {
        pub sapling: rust_decimal::Decimal,
        pub sprout: rust_decimal::Decimal,
    }
}
pub mod z_getpaymentdisclosure {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGetpaymentdisclosureArguments {
        pub four_message: Option<String>,
        pub one_txid: String,
        pub three_output_index: String,
        pub two_js_index: String,
    }
    pub type ZGetpaymentdisclosureResponse = String;
}
pub mod z_gettotalbalance {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGettotalbalanceArguments {
        pub one_minconf: Option<rust_decimal::Decimal>,
        pub two_include_watchonly: Option<bool>,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGettotalbalanceResponse {
        pub private: rust_decimal::Decimal,
        pub total: rust_decimal::Decimal,
        pub transparent: rust_decimal::Decimal,
    }
}
pub mod z_gettreestate {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZGettreestateArguments {
        pub one_hash_or_height: String,
    }
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
        pub height: rust_decimal::Decimal,
        pub sapling: Sapling,
        pub sprout: Sprout,
    }
}
pub mod z_importkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportkeyArguments {
        pub three_start_height: Option<rust_decimal::Decimal>,
        pub two_rescan: Option<String>,
        pub one_zkey: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportkeyResponse {
        pub address: String,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod z_importviewingkey {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportviewingkeyArguments {
        pub three_start_height: Option<rust_decimal::Decimal>,
        pub two_rescan: Option<String>,
        pub one_vkey: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportviewingkeyResponse {
        pub address: String,
        #[serde(rename = "type")]
        pub type_field: String,
    }
}
pub mod z_importwallet {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportwalletArguments {
        pub one_filename: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZImportwalletResponse;
}
pub mod z_listaddresses {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListaddressesArguments {
        pub one_include_watchonly: Option<bool>,
    }
    pub type ZListaddressesResponse = Vec<String>;
}
pub mod z_listoperationids {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListoperationidsArguments {
        pub one_status: Option<String>,
    }
    pub type ZListoperationidsResponse = Vec<String>;
}
pub mod z_listreceivedbyaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListreceivedbyaddressArguments {
        pub two_minconf: Option<rust_decimal::Decimal>,
        pub one_address: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListreceivedbyaddressResponse {
        pub amount: rust_decimal::Decimal,
        pub amount_zat: rust_decimal::Decimal,
        pub blockheight: rust_decimal::Decimal,
        pub blockindex: rust_decimal::Decimal,
        pub blocktime: rust_decimal::Decimal,
        pub change: bool,
        pub confirmations: rust_decimal::Decimal,
        pub jsindex: rust_decimal::Decimal,
        pub jsoutindex: rust_decimal::Decimal,
        pub memo: String,
        pub outindex: rust_decimal::Decimal,
        pub txid: String,
    }
}
pub mod z_listunspent {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListunspentArguments {
        pub one_minconf: Option<rust_decimal::Decimal>,
        pub three_include_watchonly: Option<bool>,
        pub two_maxconf: Option<rust_decimal::Decimal>,
        pub four_addresses: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZListunspent {
        pub address: String,
        pub amount: rust_decimal::Decimal,
        pub change: bool,
        pub confirmations: rust_decimal::Decimal,
        pub jsindex: rust_decimal::Decimal,
        pub jsoutindex: rust_decimal::Decimal,
        pub memo: String,
        pub outindex: rust_decimal::Decimal,
        pub spendable: bool,
        pub txid: String,
    }
    pub type ZListunspentResponse = Vec<ZListunspent>;
}
pub mod z_mergetoaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZMergetoaddressArguments {
        pub _memo: Option<String>,
        pub five_shielded_limit: Option<rust_decimal::Decimal>,
        pub four_transparent_limit: Option<rust_decimal::Decimal>,
        pub three_fee: Option<rust_decimal::Decimal>,
        pub one_fromaddresses: String,
        pub two_toaddress: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZMergetoaddressResponse {
        pub merging_notes: rust_decimal::Decimal,
        pub merging_shielded_value: rust_decimal::Decimal,
        pub merging_transparent_value: rust_decimal::Decimal,
        pub merging_u_t_x_os: rust_decimal::Decimal,
        pub opid: String,
        pub remaining_notes: rust_decimal::Decimal,
        pub remaining_shielded_value: rust_decimal::Decimal,
        pub remaining_transparent_value: rust_decimal::Decimal,
        pub remaining_u_t_x_os: rust_decimal::Decimal,
    }
}
pub mod z_sendmany {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZSendmanyArguments {
        pub four_fee: Option<rust_decimal::Decimal>,
        pub three_minconf: Option<rust_decimal::Decimal>,
        pub one_fromaddress: String,
        pub two_amounts: String,
    }
    pub type ZSendmanyResponse = String;
}
pub mod z_setmigration {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZSetmigrationArguments {
        pub one_enabled: bool,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZSetmigrationResponse;
}
pub mod z_shieldcoinbase {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZShieldcoinbaseArguments {
        pub four_limit: Option<rust_decimal::Decimal>,
        pub three_fee: Option<rust_decimal::Decimal>,
        pub one_fromaddress: String,
        pub two_toaddress: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZShieldcoinbaseResponse {
        pub opid: String,
        pub remaining_u_t_x_os: rust_decimal::Decimal,
        pub remaining_value: rust_decimal::Decimal,
        pub shielding_u_t_x_os: rust_decimal::Decimal,
        pub shielding_value: rust_decimal::Decimal,
    }
}
pub mod z_validateaddress {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZValidateaddressArguments {
        pub one_zaddr: String,
    }
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
    pub struct ZValidatepaymentdisclosureArguments {
        pub one_paymentdisclosure: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZValidatepaymentdisclosureResponse;
}
pub mod z_viewtransaction {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZViewtransactionArguments {
        pub one_txid: String,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Outputs {
        pub address: String,
        pub js: rust_decimal::Decimal,
        pub js_output: rust_decimal::Decimal,
        pub memo: String,
        pub memo_str: String,
        pub outgoing: bool,
        pub output: rust_decimal::Decimal,
        #[serde(rename = "type")]
        pub type_field: String,
        pub value: rust_decimal::Decimal,
        pub value_zat: rust_decimal::Decimal,
    }
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Spends {
        pub address: String,
        pub js: rust_decimal::Decimal,
        pub js_output_prev: rust_decimal::Decimal,
        pub js_prev: rust_decimal::Decimal,
        pub js_spend: rust_decimal::Decimal,
        pub output_prev: rust_decimal::Decimal,
        pub spend: rust_decimal::Decimal,
        pub txid_prev: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub value: rust_decimal::Decimal,
        pub value_zat: rust_decimal::Decimal,
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
    pub struct ZcbenchmarkArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct Zcbenchmark {
        pub runningtime: rust_decimal::Decimal,
    }
    pub type ZcbenchmarkResponse = Vec<Zcbenchmark>;
}
pub mod zcrawjoinsplit {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawjoinsplitArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawjoinsplitResponse {
        pub encryptednote1: String,
        pub encryptednote2: String,
        pub rawtxn: String,
    }
}
pub mod zcrawkeygen {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawkeygenArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawkeygenResponse {
        pub zcaddress: String,
        pub zcsecretkey: String,
        pub zcviewingkey: String,
    }
}
pub mod zcrawreceive {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawreceiveArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcrawreceiveResponse {
        pub amount: rust_decimal::Decimal,
        pub exists: bool,
        pub note: String,
    }
}
pub mod zcsamplejoinsplit {
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcsamplejoinsplitArguments;
    #[derive(Debug, serde :: Deserialize, serde :: Serialize)]
    pub struct ZcsamplejoinsplitResponse;
}
