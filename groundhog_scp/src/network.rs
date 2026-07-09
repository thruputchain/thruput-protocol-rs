// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 33cc27325a7a3cb150f12f75fabfabeb95b17b44

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


// Definitions for network config parameters

/// ServerAddress is an IP (or hostname) and port address, separated by a
/// colon.
pub type ServerAddress = StringM;

/// UPID is a globally unique process identifier (function of start time, place
/// and randomness)
pub type UPID = [u8; 8 as usize];

/// ProtocolCommit describes a specific protocol repo version.
/// Used to ensure communicating daemons in a validator deployment are compatible.
pub type ProtocolCommit = StringM;


//
// XDR internals below this point
//


