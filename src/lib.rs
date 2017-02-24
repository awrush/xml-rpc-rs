//! An XML-RPC implementation in Rust.
//!
//! The `xmlrpc` crate provides a minimal implementation of the [XML-RPC spec][spec].
//!
//! [Repo link][repo].
//!
//! [spec]: http://xmlrpc.scripting.com/spec.html
//! [repo]: https://github.com/jonas-schievink/xml-rpc-rs

extern crate base64;
extern crate hyper;
extern crate iso8601;
extern crate xml;

mod error;
pub mod parser;
mod request;
mod value;
mod utils;

pub use error::{RequestError, Fault};
pub use request::{Request, RequestResult};
pub use value::Value;

/// A response from the server.
///
/// XML-RPC specifies that a call should either return a single `Value`, or a `<fault>`.
pub type Response = Result<Value, Fault>;
