// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 855ee2b6f1b64eea7b421ca083525da4e6371c5a

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


// Definitions for network config parameters

/// ServerAddress is an IP (or hostname) and port address, separated by a
/// colon.
pub type ServerAddress = StringM;

/// UPID is a globally unique process identifier (function of start time, place
/// and randomness)
pub type UPID = [u8; 8 as usize];

/// ProtocolCommit describes a specific protocol repo version.
/// Used to ensure communicating daemons in a validator deployment are compatible.
///
/// Printable ASCII: a short prefix followed by the leading characters of the
/// protocol repo commit id. Peers only ever compare it byte for byte, and it
/// travels as plain text outside XDR too -- runtime manifests, log lines, and
/// indexer outbox filenames -- so it stays text rather than a raw hash.
///
/// Fixed width, so a peer cannot declare a length: decoding reads exactly
/// these bytes into an inline array, with no length prefix and no allocation.
/// 20 is a whole number of XDR words, so nothing is spent on padding.
pub type ProtocolCommit = [u8; 20 as usize];


//
// XDR internals below this point
//


