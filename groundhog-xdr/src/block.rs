// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: e3dda0763ff3fb21779d16b6fa2e130c633911a0

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
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

// This struct is serialized and hashed to form the block hash.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BlockHeader {
    // A unique identifier for this blockchain. If there are multiple instances of
    // the blockchain running, each one will have a different [network_id].
    pub network_id: u64,
    // The height of this block.
    pub block_height: BlockHeight,
    // The Merkle root hash of the transactions.
    pub txset_hash: SHA256Hash,
    // The hash of the database contents after this block is applied to the
    // database.
    pub db_hash: DbHash,
    // The Merkle root hash of the events.
    pub events_hash: SHA256Hash,
    // The hash of the previous block.
    pub prev_block_hash: BlockHash,
}


//
// XDR internals below this point
//


