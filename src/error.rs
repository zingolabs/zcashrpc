//! The `error` mod includes types representing specific errors which are all bundled into the top-level `Error` enum.

use serde::{Deserialize, Serialize};

pub type DispatchResult<R> = Result<R, DispatchError>;

/// A `ResponseResult<R>` is a convenience type-alias for `Result<R, ServerError>`.
pub type ResponseResult<R> = Result<R, ServerError>;

#[derive(Debug, derive_more::From)]
pub enum DispatchError {
    Server(ServerError),
    Input(UserInputError),
}

/// An `ServerError` represents errors encountered in making an RPC request, and encompasses application-level error responses, protocol errors, and transient failures.
#[derive(Debug, derive_more::From)]
pub enum ServerError {
    /// A `Response` represents an application-level error sent back from `zcashd`.
    Response(ResponseError),

    /// An `UnexpectedResponse` occurs when the server sends a successful response which doesn't match this crate's expected structure or types.
    UnexpectedResponse(UnexpectedResponse),

    /// A `JsonRpcViolation` indicates the `zcashd` server violates this library's expectation about JSONRPC protocol. These should not occur if this crate has thorough integration tests against the specific version of `zcashd` on the server-side.
    JsonRpcViolation(JsonRpcViolation),

    /// The `Http` variant indicates some HTTP-layer error and passes errors directly from the `reqwest` HTTP client dependency.
    Http(reqwest::Error),
}

/// The `ResponseError` represents any application-level error sent from `zcashd`.
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseError {
    pub code: i64,
    pub message: String,
}

/// An `UnexpectedResponse` occurs when `zcashd` responds with valid JSON that doesn't match the expected types of this crate.
#[derive(Debug)]
pub struct UnexpectedResponse {
    pub structure: serde_json::Value,
    pub reason: serde_json::Error,
}

/// A `JsonRpcViolation` occurs when `zcashd` responds with malformed JSON or with a response envelope that violates this crate's assumed JSONRPC protocol invariants.
#[derive(Debug)]
pub enum JsonRpcViolation {
    MalformedJson {
        input_text: String,
        reason: serde_json::Error,
    },
    UnexpectedServerId {
        client: u64,
        server: u64,
    },
    NoResultOrError,
    ResultAndError {
        result: serde_json::Value,
        error: ResponseError,
    },
}

pub type UserInputResult<R> = Result<R, UserInputError>;
#[derive(Debug, derive_more::From)]
pub enum UserInputError {
    InvalidJson(serde_json::Error),
    //Stub variant, this will hold more information in the future
    InvalidArguments,
}
