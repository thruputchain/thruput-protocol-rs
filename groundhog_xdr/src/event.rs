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



#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct EventData {
    pub topics: VecM<SHA256Hash, 3>,
    pub event: BytesM,
}

// The structure of an event that is placed into our merkle-patricia trie
// For contracts that structure their data appropriately (specify the topic types, no more than 3 topics), 
// this can be converted into a stellar::ContractEvent.
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Event {
    // sha256(ProposedTransaction)
    pub proposed_tx_hash: SHA256Hash,
    // uniquifies the event
    pub sequence_number: u32,
    pub contract_address: ContractAddress,
    pub data: EventData,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TxEmittedEvent {
    pub contract_address: ContractAddress,
    // serialized EventData
    pub serialized_event: BytesM,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TxEventLog {
    // sha256(ProposedTransaction)
    pub proposed_tx_hash: SHA256Hash,
    pub events: VecM<TxEmittedEvent>,
}


//
// XDR internals below this point
//


