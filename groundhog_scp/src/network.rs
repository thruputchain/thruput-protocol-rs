// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 8b242a5dc6ea5d82760cff0d3e5bc41a8c6aec6c

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


#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;


// Definitions for network config parameters

/// ServerAddress is an IP (or hostname) and port address, separated by a
/// colon.
pub type ServerAddress = StringM;

/// UPID is a globally unique process identifier (function of start time, place
/// and randomness)
pub type UPID = [u8; 8 as usize];

/// Protocol describes a specific protocol and version.
pub type Protocol = StringM;


//
// XDR internals below this point
//


