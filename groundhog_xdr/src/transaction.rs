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
use ::serde::{Serialize, Deserialize};
use ::serde_arrays;

pub use crate::byte_array::*;

#[cfg(feature="std")]
use std::io::{Read, Write};




// clang-format on
pub type Calldata = VecM<u8, 0xffffffff>;

pub type ReturnBuffer = VecM<u8, 0xffffffff>;

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct Invocation {
    pub invoked_address: ContractAddress,
    pub method: u32,
    pub gas_limit: GasAmount,
    pub calldata: Calldata,
}


#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature="std", derive(ReadXdrEnum,WriteXdrEnum))]
#[repr(i32)]
pub enum InvokeFlag {
    kInvokeFlagNone = 0,
    kInvokeFlagTryInvoke = 0x00000001,
}

pub type InvokeFlagSet = i32;

pub type OriginID = ByteArray<16>;

#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct Transaction {
    pub id: OriginID,
    pub created: Timestamp,
    pub invocation: Invocation,
    pub contracts_to_deploy: VecM<Contract, 0xffffffff>,
}

pub type SignedTransactionHash = SHA256Hash;

// Deliberately leaving this SignedTransaction object
// (and all the other metadata)
// for a later PR.
#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct SignedTransaction {
    // XXX Manually editing this out for now.
    // We don't have a Default implementation for Signature,
    // so it doesn't build with that in place.
    // TODO: figure out if we want a Default impl on generated unions,
    // or remove default more broadly (or change the type of Signature).
    //pub signature: Signature,
    pub tx: Transaction,
}

// Nondeterministic data attached to a transaction by the proposer.
#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct NondeterministicProposalData {
    // string of nondeterministic bits generated during the course
    // of the transaction.
    pub nondeterministic_bits: VecM<u8, 0xffffffff>,
    // the length of the nondeterministic_bits string, in bits
    // (nondeterminstic_bits.size() returns the rounded-up length in bytes)
    pub nondeterministic_bit_count: u32,
    // sequence of queue entries observed by the proposer
    pub observed_queue_entry_sequence: VecM<QueueEntry, 0xffffffff>,
}

// A signed transaction, with the nondeterministic data from the proposer
// attached to it.
// TODO(petar): Who is the signer? The originator of the transaction or the
// validator proposing it?
// TODO(petar): Should the signature capture the non-deterministic data?
#[derive(PartialEq, Debug, Default, Clone)]
#[cfg_attr(feature="std", derive(ReadXdr,WriteXdr))]
#[repr(C)]
pub struct ProposedTransaction {
    pub signed_tx: SignedTransaction,
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}


//
// XDR internals below this point
//


