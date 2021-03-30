//procedurally generated response types, note that zcashrpc-typegen
//is in early alpha, and output is subject to change at any time.
pub mod z_gettreestate {
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
