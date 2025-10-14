// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 762022f1f3117f2d6d0e41af026a4a7a1553fe27

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

// A claim about sponsored gas units for a transaction.
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct GasClaim {
    // The contract being sponsored.
    pub sponsored_contract: ContractAddress,
    // The number of gas units being sponsored.
    pub amount: GasAmount,
}

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
    pub auth_claims: AuthClaimData,
    pub gas_claims: VecM<GasClaim>,
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

pub type SignedTransactionHash = SHA256Hash;

pub type DecoratedSignatureList = VecM::<DecoratedSignature>;

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct SignedTransaction {
    pub tx: Transaction,
    // These signatures get matched against the auth claims in the transaction.
    // These signatures are over corresponding AuthClaimPreimage objects.
    pub signatures: VecM<DecoratedSignatureList>,
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


