// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 7c72a5c64359279353bd0bdc336ae3445bb71e5e

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ParallelBatch {
    pub segment_tokens: VecM<u32>,
    pub segment_bytes: VecM<u32>,
    pub xdr: BytesM,
}

pub type ParallelSignedTransactionBatch = ParallelBatch;

pub type ParallelSignedTransactionHashBatch = ParallelBatch;

pub type ParallelNondeterministicProposalDataBatch = ParallelBatch;


//
// XDR internals below this point
//


