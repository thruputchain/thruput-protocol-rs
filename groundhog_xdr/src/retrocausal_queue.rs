// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: e8b2a5c6376874d6fdb8de6ffada6edd68211553

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


#[cfg(not(feature="xdrs"))]
use xdr_common::*;



// clang-format on

pub const kQueueEntryMaxLen : u32 = 256;

// 
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct QueueEntry {
    inner: BytesM<kQueueEntryMaxLen>,
}
impl From<BytesM<kQueueEntryMaxLen>> for QueueEntry {
    fn from(_inner: BytesM<kQueueEntryMaxLen>) -> Self {
        Self { inner: _inner }
    }
}
impl QueueEntry {
    pub fn into_inner(self) -> BytesM<kQueueEntryMaxLen> {
        self.inner
    }
    pub const fn new(_inner: BytesM<kQueueEntryMaxLen>) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for QueueEntry {
    type Target = BytesM<kQueueEntryMaxLen>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for QueueEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// A block in a retrocausal queue, containing a list of entries and a block
// number The entries are stored in lexicographically sorted order within the
// block Block numbers are monotonically increasing
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RetrocausalQueueBlock {
    // Variable-length array of queue entries
    pub entries: VecM<QueueEntry>,
    // Unique identifier and ordering for this block
    pub block_number: u64,
}

// This delta consumes all of the retrocausal queue entries
// in blocks less than or equal to max_fully_consumed_block_number,
// and removes the first partial_consumed_number entries from the
// block number max_fully_consumed_block_number + 1.
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RetrocausalQueuePastBlockConsumeDelta {
    pub max_fully_consumed_block_number: u64,
    pub partial_consumed_number: u64,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RetrocausalQueueConsumeDelta {
    pub past_block_consume: RetrocausalQueuePastBlockConsumeDelta,
    pub current_block_consumed: VecM<QueueEntry>,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RetrocausalQueueInsertDelta {
    pub added_entries: VecM<QueueEntry>,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RetrocausalQueueDelta {
    // set to null if no entries are consumed.
    // This version of the retrocausal queue is single-consumer multi-producer.
    pub consume: Option<RetrocausalQueueConsumeDelta>,
    pub insert: RetrocausalQueueInsertDelta,
}


//
// XDR internals below this point
//


