// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 21ee49e9b2a76b0fa98de8a0837954f4f9d61f19

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



// clang-format on

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HogConfig {
    pub private_info: ValidatorPrivateInfo,
    // can use HOGX_VALIDATOR_ADDRESS
    pub quorum_set: SCPQuorumSet,
}


//
// XDR internals below this point
//


