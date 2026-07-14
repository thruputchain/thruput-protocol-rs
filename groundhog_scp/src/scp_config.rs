// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: c0e2b46d8abfb48cd8fb90d8f06af8d2a99acf27

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
pub struct SCPConfig {
    pub max_timeout_ms: u64,
    pub initial_ballot_timeout_ms: u64,
    pub increment_ballot_timeout_ms: u64,
    pub initial_nomination_timeout_ms: u64,
    pub increment_nomination_timeout_ms: u64,
    pub quorum_set: SCPQuorumSet,
}


//
// XDR internals below this point
//


