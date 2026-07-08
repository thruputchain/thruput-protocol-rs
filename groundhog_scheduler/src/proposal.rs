// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 33cc27325a7a3cb150f12f75fabfabeb95b17b44

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
use xdrs::*;



#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

/// A candidate block: header plus the transaction metadata needed to
/// re-execute it.
///
/// Does not travel on network: nodes reconstruct it from a `ConsensusValue`
/// and its `ProposalTxs`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Proposal {
    pub header: ProposalHeader,
    pub transactions: ProposalTxs,
}

/// Transaction-side payload for a proposal.
///
/// This is the payload a `ConsensusValue` commits to through
/// `ProposalTxsHash`. The signed transaction bodies travel separately, since
/// nodes usually already hold them from transaction dissemination.
///
/// Travels on network.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ProposalTxs {
    pub ext: ExtensionPoint,
    /// Hashes of the block's signed transactions, in execution order.
    pub hash: ParallelSignedTransactionHashBatch,
    /// Nondeterministic choices captured by the proposer, replayed by
    /// validators so re-execution reproduces the header outputs exactly.
    pub nondeterministic_data: ParallelNondeterministicProposalDataBatch,
}

/// SHA-256 of the XDR-encoded `ProposalTxs`.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ProposalTxsHash {
    inner: SHA256Hash,
}
impl From<SHA256Hash> for ProposalTxsHash {
    fn from(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}
impl ProposalTxsHash {
    pub fn into_inner(self) -> SHA256Hash {
        self.inner
    }
    pub const fn new(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ProposalTxsHash {
    type Target = SHA256Hash;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ProposalTxsHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub type OptProposal = Option::<Proposal>;

/// Summary of the previous finalized proposal, used by SCP leader selection.
///
/// Intentionally carries only the `ConsensusValueHash`: leader-election
/// pseudorandomness must not depend on transaction-payload availability.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct PrevProposalSummary {
    pub value_hash: ConsensusValueHash,
}


//
// XDR internals below this point
//


