// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 98514a367a11d3d9fa0455293a7d2f9bc369ac15

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




// number of topics is 3:
// 1. event signature
// 2. from_address
// 3. to_address
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TransferEvent {
    pub from_address: ContractAddress,
    pub to_address: ContractAddress,
    pub amount: int128_xdr,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ApprovalEvent {
    pub owner_address: ContractAddress,
    pub spender_address: ContractAddress,
    pub amount: int128_xdr,
}


//
// XDR internals below this point
//


