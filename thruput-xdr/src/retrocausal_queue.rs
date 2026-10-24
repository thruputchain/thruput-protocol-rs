// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: e0e14dde6a5cd5db1bcaab5a5c5c8006735576c1

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



// clang-format on

pub const kQueueEntryMaxLen : u32 = 256;

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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

/// Per-transaction cap on the number of concurrent RetrocausalQueue entries a
/// transaction may observe (peek/consume) and record in its
/// NondeterministicProposalData. Each observation is proposer-controlled and
/// must be buffered and validated against an insert, so an unbounded count is
/// an unbounded memory/compute vector — this is a tx resource limit in the same
/// family as gas, not an RPC artifact. Because it is a tx-observable count
/// checked against this protocol constant, the limit is consensus-uniform
/// (every node agrees on "<= cap reads") and independent of any RPC buffer size.
/// (Full rationale — incl. why an RPC-message-size bound would be unsafe — in
/// the PR.)
pub const kMaxConcurrentRcqReadsPerTx : u32 = 16384;

/// Maximum number of entries in a single RetrocausalQueue block.
/// Each entry is at most 256 bytes. With XDR serialization overhead (4 bytes
/// per entry length prefix, 12 bytes framing), ramdb key overhead (73 bytes),
/// and ItemHeader (24 bytes), 250 entries of 256 bytes fits within ramdb's
/// 64 KiB item limit (kBigItemThreshold). See issue #1437.
pub const kRetrocausalQueueBlockEntriesMaxLen : u32 = 250;

// A block in a retrocausal queue, containing a list of entries and a block
// number The entries are stored in lexicographically sorted order within the
// block Block numbers are monotonically increasing
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RetrocausalQueuePastBlockConsumeDelta {
    pub max_fully_consumed_block_number: u64,
    pub partial_consumed_number: u64,
}

// Root metadata for a Retrocausal Queue, stored at the item key.
// Tracks both head (consume state) and tail (production state).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RetrocausalQueueConsumeDelta {
    pub past_block_consume: RetrocausalQueuePastBlockConsumeDelta,
    pub current_block_consumed: VecM<QueueEntry, kRetrocausalQueueBlockEntriesMaxLen>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RetrocausalQueueInsertDelta {
    pub added_entries: VecM<QueueEntry, kRetrocausalQueueBlockEntriesMaxLen>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RetrocausalQueueDelta {
    // set to null if no entries are consumed.
    // This version of the retrocausal queue is single-consumer multi-producer.
    pub consume: Option<RetrocausalQueueConsumeDelta>,
    pub insert: RetrocausalQueueInsertDelta,
    // Concurrent entries observed (e.g. via peek) but not consumed.
    // HogDb validates that each entry was actually inserted by a concurrent
    // transaction. Unlike current_block_consumed, these do not decrement the
    // insert balance — they only require that a matching insert exists.
    //
    // This is bounded by the per-transaction observation cap
    // (kMaxConcurrentRcqReadsPerTx), not the much smaller per-block entry cap
    // (kRetrocausalQueueBlockEntriesMaxLen), for a non-obvious reason: when a
    // validator replays a proposed block, the executor rebuilds this per-queue
    // array from the proposer-supplied observation list, and a dishonest proposer
    // can pack a whole transaction's observations into a single queue. The bogus
    // entries are caught afterward (HogDb matches each observed entry to a real
    // insert), but the array must be built before that check runs -- so it has to
    // hold a full transaction's worth. Capping it at the per-block size would make
    // that rebuild overflow and crash the validator instead of cleanly rejecting.
    pub observed_concurrent_entries: VecM<QueueEntry, kMaxConcurrentRcqReadsPerTx>,
}


//
// XDR internals below this point
//


