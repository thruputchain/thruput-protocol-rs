// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: d186232e27cd16b0bc090bb23cd5c474b02b30bf

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
use ::xdrs::{ReadXDR, WriteXDR};


use xdr_common::*;



// clang-format on

// TXS encoding of the call arguments (as a tuple)
pub type Calldata = BytesM;

pub type ReturnBuffer = BytesM;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct Invocation {
    pub invoked_address: ContractAddress,
    pub entrypoint: ContractEntrypointID,
    pub gas_limit: GasAmount,
    pub calldata: Calldata,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum InvokeFlag {
    kInvokeFlagNone = 0 as i32,
    kInvokeFlagTryInvoke = 0x00000001 as i32,
    kInvokeFlagReadOnly = 0x00000002 as i32,
    kInvokeFlagDelegateCall = 0x00000004 as i32,
}
pub use InvokeFlag::*;

impl TryFrom<i32> for InvokeFlag {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kInvokeFlagNone),
            0x00000001 => Ok(Self::kInvokeFlagTryInvoke),
            0x00000002 => Ok(Self::kInvokeFlagReadOnly),
            0x00000004 => Ok(Self::kInvokeFlagDelegateCall),
            _ => Err(()),
        }
    }
}


pub type InvokeFlagSet = i32;

pub type OriginID = [u8; 16];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct Transaction {
    pub origin_id: OriginID,
    pub created: Timestamp,
    pub invocation: Invocation,
    pub contracts_to_deploy: VecM<Contract>,
}

pub type SignedTransactionHash = SHA256Hash;

// Deliberately leaving this SignedTransaction object
// (and all the other metadata)
// for a later PR.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct SignedTransaction {
    pub signature: Signature,
    pub tx: Transaction,
}

// Nondeterministic data attached to a transaction by the proposer.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct ProposedTransaction {
    pub signed_tx: SignedTransaction,
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}

// Nominations, included in SCP messages, include only the ND data.
// The transactions themselves are disseminated through the overlay.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct NominatedTransaction {
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum TransactionStatistic {
    kTxStatGasRemaining = 0 as i32,
}
pub use TransactionStatistic::*;

impl TryFrom<i32> for TransactionStatistic {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kTxStatGasRemaining),
            _ => Err(()),
        }
    }
}



//
// XDR internals below this point
//


