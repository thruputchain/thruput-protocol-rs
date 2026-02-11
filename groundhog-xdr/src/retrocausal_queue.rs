// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 84fabf8ff739fc6a8d0917c073b267be05fe3db9

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

pub const kQueueEntryMaxLen : u32 = 256;

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RetrocausalQueuePastBlockConsumeDelta {
    pub max_fully_consumed_block_number: u64,
    pub partial_consumed_number: u64,
}

// Root metadata for a Retrocausal Queue, stored at the item key.
// Tracks both head (consume state) and tail (production state).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RetrocausalQueueMetadata {
    // Current consume state - where consumption left off.
    pub consume_state: RetrocausalQueuePastBlockConsumeDelta,
    // The highest block number currently live (written and not fully consumed).
    // New blocks will be numbered max_live_block_num + 1.
    // 0 if no blocks have been produced yet.
    pub max_live_block_num: u64,
    // Total number of entries currently live in the queue (for cost calculation).
    pub live_entries: u64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RetrocausalQueueConsumeDelta {
    pub past_block_consume: RetrocausalQueuePastBlockConsumeDelta,
    pub current_block_consumed: VecM<QueueEntry>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RetrocausalQueueInsertDelta {
    pub added_entries: VecM<QueueEntry>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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


