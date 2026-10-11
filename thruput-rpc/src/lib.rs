#![allow(non_snake_case)]
#![allow(unused_imports)]

#[cfg(not(feature = "xdrs"))]
use xdr_common::*;

#[rustfmt::skip]
pub mod test_rpc;
pub use test_rpc::*;
