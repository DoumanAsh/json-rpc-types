//! JSON-RPCv2 data types
//!
//! This library provides generic type definitions to serialize/deserialize JSON-RPC request/responses.
//! It doesn't contain implementation itself as it is intended to be used as building block of actual implementation.
//!
//! ## Features
//!
//! - `id-str-only` - Forces ID deserialization to assume string only.
//! - `id-number-only` - Forces ID deserialization to assume number only.
#![warn(missing_docs)]

#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style, clippy::derivable_impls))]
#![cfg_attr(feature = "cargo-fmt", rustfmt::skip)]

pub use str_buf;

#[rustfmt::skip]
mod version;
pub use version::Version;
#[rustfmt::skip]
mod id;
pub use id::Id;
#[rustfmt::skip]
mod error;
pub use error::{ErrorCode, Error};
#[rustfmt::skip]
mod request;
pub use request::Request;
#[rustfmt::skip]
mod response;
pub use response::Response;
#[rustfmt::skip]
mod utils;
