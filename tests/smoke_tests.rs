macro_rules! run_smoketest {
    ($x:ident ($($args:expr)?)) => {
        #[tokio::test]
        async fn $x() {
            #[allow(unused_imports)]
            use serde_json::{json, from_value};
            let _response = zcashrpc::client::utils::make_client(true)
                .$x($(from_value($args).unwrap())?)
                .await
                .unwrap();
        }
    };
}

run_smoketest!(getblockchaininfo());
run_smoketest!(z_getnewaddress(json!(None::<String>)));
