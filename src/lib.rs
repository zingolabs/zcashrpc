//! An asynchronous zcashd RPC client.

pub mod client;
mod envelope;
pub mod error;
mod json;

#[doc(inline)]
pub use client::Client;

#[doc(inline)]
pub use error::{
    DispatchResult, ResponseResult, ServerError, UserInputError,
    UserInputResult,
};

/// The `ZecAmount` type alias is used to document where ZEC-denominated fields are used. Note that this does not represent Zatoshi-denominated units.
pub type ZecAmount = rust_decimal::Decimal;
