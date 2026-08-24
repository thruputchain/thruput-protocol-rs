// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 7cc13390a06c0d4590a76fcc35ad15f6b499f050

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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ParallelTransactionHashBatch {
    inner: ParallelBatch,
}
impl From<ParallelBatch> for ParallelTransactionHashBatch {
    fn from(_inner: ParallelBatch) -> Self {
        Self { inner: _inner }
    }
}
impl ParallelTransactionHashBatch {
    pub fn into_inner(self) -> ParallelBatch {
        self.inner
    }
    pub const fn new(_inner: ParallelBatch) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ParallelTransactionHashBatch {
    type Target = ParallelBatch;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ParallelTransactionHashBatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

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


