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



// clang-format on

// XXX(petar): Nomination should be renamed to Proposal
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Nomination {
    pub block_height: BlockHeight,
    //XXX: rename to merkle_hash
    pub block_hash: BlockHash,
    //XXX: rename to prev_merkle_hash
    pub prev_block_hash: BlockHash,
    pub transactions_hash: ParallelSignedTransactionHashBatch,
    pub transactions_nondeterministic_data: ParallelNondeterministicProposalDataBatch,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct PrevNominationSummary {
    pub block_hash: BlockHash,
}


//
// XDR internals below this point
//


