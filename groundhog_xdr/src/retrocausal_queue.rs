// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};




// clang-format on

pub const kQueueEntryMaxLen : u32 = 256;

pub type QueueEntry = VecM<u8, kQueueEntryMaxLen>;

// A block in a retrocausal queue, containing a list of entries and a block
// number The entries are stored in lexicographically sorted order within the
// block Block numbers are monotonically increasing
#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct RetrocausalQueueBlock {
    // Variable-length array of queue entries
    pub entries: VecM<QueueEntry, 0xffffffff>,
    // Unique identifier and ordering for this block
    pub block_number: u64,
}

// This delta consumes all of the retrocausal queue entries
// in blocks less than or equal to max_fully_consumed_block_number,
// and removes the first partial_consumed_number entries from the
// block number max_fully_consumed_block_number + 1.
#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct RetrocausalQueuePastBlockConsumeDelta {
    pub max_fully_consumed_block_number: u64,
    pub partial_consumed_number: u64,
}

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct RetrocausalQueueConsumeDelta {
    pub past_block_consume: RetrocausalQueuePastBlockConsumeDelta,
    pub current_block_consumed: VecM<QueueEntry, 0xffffffff>,
}

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct RetrocausalQueueInsertDelta {
    pub added_entries: VecM<QueueEntry, 0xffffffff>,
}

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
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


