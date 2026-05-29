// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: d77f1087b6802e206902605689e2d4f92618a753

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
pub struct ParallelBatch {
    pub segment_tokens: VecM<u32>,
    pub segment_bytes: VecM<u32>,
    pub xdr: BytesM,
}

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ParallelSignedTransactionBatch {
    inner: ParallelBatch,
}
impl From<ParallelBatch> for ParallelSignedTransactionBatch {
    fn from(_inner: ParallelBatch) -> Self {
        Self { inner: _inner }
    }
}
impl ParallelSignedTransactionBatch {
    pub fn into_inner(self) -> ParallelBatch {
        self.inner
    }
    pub const fn new(_inner: ParallelBatch) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ParallelSignedTransactionBatch {
    type Target = ParallelBatch;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ParallelSignedTransactionBatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ParallelSignedTransactionHashBatch {
    inner: ParallelBatch,
}
impl From<ParallelBatch> for ParallelSignedTransactionHashBatch {
    fn from(_inner: ParallelBatch) -> Self {
        Self { inner: _inner }
    }
}
impl ParallelSignedTransactionHashBatch {
    pub fn into_inner(self) -> ParallelBatch {
        self.inner
    }
    pub const fn new(_inner: ParallelBatch) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ParallelSignedTransactionHashBatch {
    type Target = ParallelBatch;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ParallelSignedTransactionHashBatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ParallelNondeterministicProposalDataBatch {
    inner: ParallelBatch,
}
impl From<ParallelBatch> for ParallelNondeterministicProposalDataBatch {
    fn from(_inner: ParallelBatch) -> Self {
        Self { inner: _inner }
    }
}
impl ParallelNondeterministicProposalDataBatch {
    pub fn into_inner(self) -> ParallelBatch {
        self.inner
    }
    pub const fn new(_inner: ParallelBatch) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ParallelNondeterministicProposalDataBatch {
    type Target = ParallelBatch;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ParallelNondeterministicProposalDataBatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}


//
// XDR internals below this point
//


