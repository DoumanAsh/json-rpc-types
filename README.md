# json-rpc-types

![Rust](https://github.com/DoumanAsh/json-rpc-types/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/json-rpc-types.svg)](https://crates.io/crates/json-rpc-types)
[![Documentation](https://docs.rs/json-rpc-types/badge.svg)](https://docs.rs/crate/json-rpc-types/)

This library provides generic type definitions to serialize/deserialize JSON-RPC request/responses.
It doesn't contain implementation itself as it is intended to be used as building block of actual implementation.

## Features

- `id-fixed-int`:
  Disabled by default. Fix the `id` field deserialize into a `Id::Int()` only. This allow to use [`serde-json-core`] instead of [`serde_json`] for `no_std` programs.

[`serde-json-core`]: https://crates.io/crates/serde-json-core
[`serde_json`]: https://crates.io/crates/serde_json
