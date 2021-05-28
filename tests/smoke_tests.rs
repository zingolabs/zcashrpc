macro_rules! run_smoketest {
    ($x:ident) => {
        #[tokio::test]
        async fn $x() {
            let _response = zcashrpc::client::utils::make_client(true)
                .$x()
                .await
                .unwrap();
        }
    };
}

run_smoketest!(getblockchaininfo);
run_smoketest!(z_getnewaddress);
