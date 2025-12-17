// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 98514a367a11d3d9fa0455293a7d2f9bc369ac15

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

#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;



// The event format puts the first N * 32 bytes of the data
// as the topics.

pub const kMaxEventLength : u32 = 1024;

pub const kMaxTopics : u32 = 3;

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
pub struct Topic {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays"))]
    inner: [u8; 32],
}
impl From<[u8; 32]> for Topic {
    fn from(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}
impl Topic {
    pub fn into_inner(self) -> [u8; 32] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for Topic {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Topic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for Topic {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}
impl<'a> From<&'a [u8; 32]> for &'a Topic {
    fn from(arr: &[u8; 32]) -> Self {
        let ptr = arr as *const [u8; 32] as *const Topic;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]
impl<'r> ReadXDR<'r> for &'r Topic {
    fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
        let arr: &[u8; 32] = xdrs::read_xdr(r)?;
        let val = Self::from(arr);
        Ok(val)
    }
}


#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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


