use crate::{error::ResponseError, ResponseResult};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseEnvelope {
    id: u64,
    result: Option<serde_json::Value>,
    error: Option<ResponseError>,
}

impl ResponseEnvelope {
    pub fn unseal<R>(self, clientid: u64) -> ResponseResult<R>
    where
        R: DeserializeOwned,
    {
        use crate::json;

        let jv = self.unseal_internal(clientid)?;
        json::parse_value(jv)
    }

    fn unseal_internal(
        self,
        clientid: u64,
    ) -> ResponseResult<serde_json::Value> {
        use crate::{
            error::JsonRpcViolation::*,
            Error::{JsonRpcViolation, Response},
        };

        if self.id != clientid {
            Err(JsonRpcViolation(UnexpectedServerId {
                client: clientid,
                server: self.id,
            }))
        } else {
            match (self.result, self.error) {
                (None, None) => Err(JsonRpcViolation(NoResultOrError)),
                (Some(r), Some(e)) => Err(JsonRpcViolation(ResultAndError {
                    result: r,
                    error: e,
                })),
                (Some(r), None) => Ok(r),
                (None, Some(e)) => Err(Response(e)),
            }
        }
    }
}

/// Contains RPC name and args and id
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RequestEnvelope {
    pub(crate) id: u64,
    pub(crate) method: &'static str,
    pub(crate) params: Vec<serde_json::Value>,
}

impl<'a> From<&'a RequestEnvelope> for reqwest::Body {
    fn from(re: &'a RequestEnvelope) -> reqwest::Body {
        use serde_json::to_string_pretty;

        reqwest::Body::from(to_string_pretty(re).unwrap())
    }
}

impl RequestEnvelope {
    pub fn seal(
        id: u64,
        method: &'static str,
        params: Vec<serde_json::Value>,
    ) -> RequestEnvelope {
        RequestEnvelope {
            id: id,
            method: method,
            params: params,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn unseal_internal_right_id() {
        dbg!("HELLO HAZEL!");
        use super::*;
        let test_respenvelope = ResponseEnvelope {
            id: 0 as u64,
            result: Some(serde_json::Value::Bool(true)),
            error: None,
        };
        test_respenvelope.unseal::<bool>(5).expect_err(
            "This should be an error. Client id and server id are different.",
        );
    }
}