// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: ca84ca7f43039d74b7bb3e1dfd7a32766d798d77

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "std")]
use std::io::{Read, Write};
use xdrs::*;



#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct StorageCostModelXdr {
    pub ext: ExtensionPoint,
    pub base_data_item_cost: int128_xdr,
    pub bytestring_cost_per_byte: i32,
    pub base_contract_deployment_cost: int128_xdr,
    pub contract_deployment_cost_per_byte: int128_xdr,
    pub auth_per_key_cost: int128_xdr,
    pub rcq_cost_per_entry: int128_xdr,
    pub replay_cache_cost_per_entry: int128_xdr,
    pub balance_cost: int128_xdr,
}


//
// XDR internals below this point
//


