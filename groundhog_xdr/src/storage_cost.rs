// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 9018ed1dba53614fec304b3ae7dab58e4f4f16ff

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};

#[cfg(feature="xdrs")]
use xdrs::*;
#[cfg(feature="spec")]
use xdrs::xdr_repr::*;




#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct StorageCostModelXdr {
    pub ext: ExtensionPoint,
    pub base_data_item_cost: int128_xdr,
    pub bytestring_cost_per_byte: i32,
    pub base_contract_deployment_cost: int128_xdr,
    pub contract_deployment_cost_per_byte: int128_xdr,
    pub auth_per_key_cost: int128_xdr,
    pub replay_cache_cost_per_entry: int128_xdr,
}


//
// XDR internals below this point
//


