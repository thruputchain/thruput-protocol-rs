#![allow(non_snake_case)]
#![allow(unused_imports)]

pub use groundhog_xdr::types::*;

#[cfg(not(feature = "xdrs"))]
use xdr_common::*;

#[rustfmt::skip]
pub mod scp;
pub use scp::*;

#[rustfmt::skip]
pub mod scp_config;
pub use scp_config::*;

#[rustfmt::skip]
pub mod scp_control;
pub use scp_control::*;

#[rustfmt::skip]
pub mod scp_handle;
pub use scp_handle::*;

#[rustfmt::skip]
pub mod network;
pub use network::*;
