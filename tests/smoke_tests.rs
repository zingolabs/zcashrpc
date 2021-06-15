macro_rules! run_smoketest {
    ($tn:ident, $x:ident ($($args:expr)?)) => {
        #[tokio::test]
        async fn $tn() {
            use zcashrpc::client::ProcedureCall;
            #[allow(unused_imports)]
            use serde_json::{json, from_value};
            let _response = zcashrpc::client::utils::make_client(true)
                .$x($(from_value($args).unwrap())?)
                .await
                .unwrap();
        }
    };
}

run_smoketest!(happy_gbi, getblockchaininfo());
run_smoketest!(noarg_zgna, z_getnewaddress(json!(None::<String>)));
run_smoketest!(onearg_zgna, z_getnewaddress(json!("sapling")));
//run_smoketest!(onearg_zmergetoaddress, z_mergetoaddress(json!("sapling")));

#[tokio::test]
async fn dispatch_named_command_z_getnewaddress() {
    let _response = zcashrpc::client::Client::dispatch_named_command(
        "z_getnewaddress".to_string(),
        vec![serde_json::json!("sapling")],
    )
    .await
    .unwrap();
}
