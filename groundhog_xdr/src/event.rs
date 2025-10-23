// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: cf97ef93e476696a208681298310e15f4b728136

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



// The event format puts the first N * 32 bytes of the data
// as the topics.

pub const kMaxEventLength : u32 = 1024;

pub const kMaxTopics : u32 = 3;

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TxEmittedEvent {
    pub contract_address: ContractAddress,
    // <= kMaxTopics
    pub num_topics: u32,
    pub serialized_event: BytesM<kMaxEventLength>,
}

// This is the struct that goes in the merkle-patricia trie.
// The key is the proposed_tx_hash field, and the value is the events field.
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TxEventLog {
    // sha256(ProposedTransaction)
    pub proposed_tx_hash: SHA256Hash,
    pub events: VecM<TxEmittedEvent>,
}

// Information that suffices to uniquely identify an event.
// This information can be reconstructed from the TxEmittedEvent.
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct IdentifiedEvent {
    // sha256(ProposedTransaction)
    pub proposed_tx_hash: SHA256Hash,
    // uniquifies the event
    pub sequence_number: u32,
    pub emitted_event: TxEmittedEvent,
}


//
// XDR internals below this point
//


