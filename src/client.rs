//! Includes the `Client`
pub mod rpc_types;
pub mod utils;

use crate::ResponseResult;
use serde::de::DeserializeOwned;
use std::future::Future;

/// A `Client` is used to make multiple requests to a specific zcashd RPC server. Requests are invoked by async methods that correspond to `zcashd` RPC API method names with request-specific parameters. Each such method has an associated response type.
pub struct Client {
    inner: utils::ReqwClientWrapper,
}

impl Client {
    /// Construct a new `Client` with connection & authentication info.
    /// - `hostport` is a host/ip with an optional `:PORT` appended.
    /// - `authcookie` is the contents of `~/.zcash/.cookie`.
    pub fn new(hostport: String, authcookie: String) -> Client {
        Client {
            inner: utils::ReqwClientWrapper::new(hostport, authcookie),
        }
    }
}

zcashrpc_macros::implement_rpc_call_methods! {}
impl ProcedureCall for Client {
    fn make_request<R>(
        &mut self,
        method: &'static str,
        args: Vec<serde_json::Value>,
    ) -> std::pin::Pin<Box<dyn Future<Output = ResponseResult<R>>>>
    where
        R: DeserializeOwned,
    {
        dbg!(&args);
        use crate::{envelope::ResponseEnvelope, json};

        let (id, sendfut) = self.inner.procedure_call(method, args);
        Box::pin(async move {
            let reqresp = sendfut.await?;
            let text = reqresp.text().await?;
            let respenv: ResponseEnvelope =
                json::parse_value(json::parse_string(text)?)?;
            let resp = respenv.unseal(id)?;
            Ok(resp)
        })
    }
}

#[cfg(test)]
mod client {
    use super::*;
    #[test]
    fn serialize_into_output_format_getinfo_happy_path() {
        let input_args = vec![serde_json::json!([])];
        let expected = vec![serde_json::Value::Array(vec![])];
        let observed = Client::serialize_into_output_format(input_args);
        assert_eq!(expected, observed);
    }
    #[test]
    fn serialize_into_output_format_z_getnewaddress_no_arg() {
        //let input_args = vec![serde_json::json!([])];
        //let expected = vec![serde_json::Value::Array(vec![])];
        //let observed = Client::serialize_into_output_format(input_args);
        //assert_eq!(expected, observed);
    }
    #[test]
    fn serialize_into_output_format_z_getnewaddress_sprout() {
        //let input_args = vec![serde_json::json!([])];
        //let expected = vec![serde_json::Value::Array(vec![])];
        //let observed = Client::serialize_into_output_format(input_args);
        //assert_eq!(expected, observed);
    }
}
