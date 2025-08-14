// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 

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

// TXS encoding of the call arguments (as a tuple)
pub type Calldata = BytesM;

pub type ReturnBuffer = BytesM;

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Invocation {
    pub invoked_address: ContractAddress,
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: GasAmount,
    pub calldata: Calldata,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[repr(i32)]
pub enum InvokeFlag {
    kInvokeFlagNone = 0 as i32,
    kInvokeFlagTryInvoke = 0x00000001 as i32,
    kInvokeFlagReadOnly = 0x00000002 as i32,
    kInvokeFlagDelegateCall = 0x00000004 as i32,
}pub use InvokeFlag::*;


pub type InvokeFlagSet = i32;

pub type OriginID = [u8; 16];

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Transaction {
    pub id: OriginID,
    pub created: Timestamp,
    pub invocation: Invocation,
    pub contracts_to_deploy: VecM<Contract>,
}

pub type SignedTransactionHash = SHA256Hash;

// Deliberately leaving this SignedTransaction object
// (and all the other metadata)
// for a later PR.
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SignedTransaction {
    pub signature: Signature,
    pub tx: Transaction,
}

// Nondeterministic data attached to a transaction by the proposer.
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NondeterministicProposalData {
    // string of nondeterministic bits generated during the course
    // of the transaction.
    pub nondeterministic_bits: BytesM,
    // the length of the nondeterministic_bits string, in bits
    // (nondeterminstic_bits.size() returns the rounded-up length in bytes)
    pub nondeterministic_bit_count: u32,
    // sequence of queue entries observed by the proposer
    pub observed_queue_entry_sequence: VecM<QueueEntry>,
}

// A signed transaction, with the nondeterministic data from the proposer
// attached to it.
// TODO(petar): Who is the signer? The originator of the transaction or the
// validator proposing it?
// TODO(petar): Should the signature capture the non-deterministic data?
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ProposedTransaction {
    pub signed_tx: SignedTransaction,
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}

// Nominations, included in SCP messages, include only the ND data.
// The transactions themselves are disseminated through the overlay.
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct NominatedTransaction {
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[repr(i32)]
pub enum TransactionStatistic {
    kTxStatGasRemaining = 0 as i32,
}pub use TransactionStatistic::*;



//
// XDR internals below this point
//


