//! Includes both `Client` and all of the RPC response types.
#[macro_use]
mod callrpc;
pub mod subcomponents;
pub mod utils;

use self::subcomponents::{
    GenerateResponse, GetBlockChainInfoResponse, GetInfoResponse,
    ZGetNewAddressResponse,
};
use crate::ResponseResult;
use reqwest;
use serde::de::DeserializeOwned;
use std::future::Future;
use std::ops::RangeFrom;

/// A `Client` is used to make multiple requests to a specific zcashd RPC server. Requests are invoked by async methods that correspond to `zcashd` RPC API method names with request-specific parameters. Each such method has an associated response type.
pub struct Client {
    url: String,
    auth: String,
    reqcli: reqwest::Client,
    idit: RangeFrom<u64>,
}

impl Client {
    /// Construct a new `Client` with connection & authentication info.
    /// - `hostport` is a host/ip with an optional `:PORT` appended.
    /// - `authcookie` is the contents of `~/.zcash/.cookie`.
    pub fn new(hostport: String, authcookie: String) -> Client {
        Client {
            url: format!("http://{}/", hostport),
            auth: format!("Basic {}", base64::encode(authcookie)),
            reqcli: reqwest::Client::new(),
            idit: (0..),
        }
    }

    zcashrpc_macros::declare_rpc_client_methods! {
        GetInfo,
        GetBlockChainInfo,
        ZGetNewAddress,
        Generate (how_many: u32),
    }
}

impl Client {
    fn make_request<R>(
        &mut self,
        method: &'static str,
        args: Vec<serde_json::Value>,
    ) -> impl Future<Output = ResponseResult<R>>
    where
        R: DeserializeOwned,
    {
        use crate::{
            envelope::{RequestEnvelope, ResponseEnvelope},
            json,
        };

        let id = self.idit.next().unwrap();
        let sendfut = self
            .reqcli
            .post(&self.url)
            .header("Authorization", &self.auth)
            .body(&RequestEnvelope::wrap(id, method, args))
            .send();
        async move {
            let reqresp = sendfut.await?;
            let text = reqresp.text().await?;
            let respenv: ResponseEnvelope =
                json::parse_value(json::parse_string(text)?)?;
            let resp = respenv.unwrap(id)?;
            Ok(resp)
        }
    }
}
