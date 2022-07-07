//! JSON-RPCv2 data types
//!
//! This library provides generic type definitions to serialize/deserialize JSON-RPC request/responses.
//! It doesn't contain implementation itself as it is intended to be used as building block of actual implementation.
#![warn(missing_docs)]

#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

pub use str_buf;

mod version;
pub use version::Version;
mod id;
pub use id::Id;
mod error;
pub use error::{ErrorCode, Error};
mod request;
pub use request::Request;
mod response;
pub use response::Response;
mod utils;
