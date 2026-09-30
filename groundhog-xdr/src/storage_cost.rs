// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: dcb5d7af8f39be558b0bfe56c0d0bb9163cce9f2

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
#[cfg(feature = "xdrs")]
use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct StorageCostModelXdr {
    pub ext: ExtensionPoint,
    pub base_data_item_cost: u64,
    pub bytestring_cost_per_byte: u32,
    pub base_contract_deployment_cost: u64,
    pub contract_deployment_cost_per_byte: u64,
    pub auth_per_key_cost: u64,
    pub rcq_cost_per_entry: u64,
    pub replay_cache_cost_per_entry: u64,
    pub balance_cost: u64,
}


//
// XDR internals below this point
//


