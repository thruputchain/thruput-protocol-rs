// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0c2c59fb8ebeb0d6d37a1dad9cf88c4f22187ce6

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



// clang-format on

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
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


