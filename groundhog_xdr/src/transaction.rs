// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: af7ecf4a4dc1cf1c12cfd67f41044f04349e57af

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

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Uniquifier {
    pub block_expiry_height: BlockHeight,
    pub nonce: u64,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Transaction {
    pub invocation: Invocation,
    pub uniquifier: Uniquifier,
    // New contracts to deploy.
    pub contracts_to_deploy: VecM<Contract>,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AuthSignaturePreimage {
    pub signature_weight: Weight,
    pub message: BytesM,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct DecoratedSignature {
    // uint32_t
    pub public_key_id: u32,
    pub signature: Signature,
}

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AuxiliaryDecoratedSignatures {
    pub address: ContractAddress,
    pub signatures: VecM<DecoratedSignature>,
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
    pub tx: Transaction,
    pub signature_weight: Weight,
    pub signatures: VecM<DecoratedSignature>,
    pub aux_signatures: VecM<AuxiliaryDecoratedSignatures>,
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


