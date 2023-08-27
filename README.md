# json-rpc-types

![Rust](https://github.com/DoumanAsh/json-rpc-types/workflows/Rust/badge.svg?branch=master)
[![Crates.io](https://img.shields.io/crates/v/json-rpc-types.svg)](https://crates.io/crates/json-rpc-types)
[![Documentation](https://docs.rs/json-rpc-types/badge.svg)](https://docs.rs/crate/json-rpc-types/)

This library provides generic type definitions to serialize/deserialize JSON-RPC request/responses.
It doesn't contain implementation itself as it is intended to be used as building block of actual implementation.

## Features

- `id-str-only` - Forces ID deserialization to assume string only.
- `id-number-only` - Forces ID deserialization to assume number only.
