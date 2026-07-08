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

/// A serialized protocol upgrade, carried opaquely in `ConsensusValue`.
pub type Upgrade = BytesM::<128>;

/// SHA-256 of the XDR-encoded `ConsensusValue`; the canonical block identity.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ConsensusValueHash {
    inner: SHA256Hash,
}
impl From<SHA256Hash> for ConsensusValueHash {
    fn from(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}
impl ConsensusValueHash {
    pub fn into_inner(self) -> SHA256Hash {
        self.inner
    }
    pub const fn new(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ConsensusValueHash {
    type Target = SHA256Hash;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ConsensusValueHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// The block commitment externalized by consensus.
///
/// `ConsensusValue` is the small value that systems use to identify
/// a finalized Groundhog block without the full transaction payload.
/// Its XDR hash is the block identity (`ConsensusValueHash`), so every
/// field is consensus-critical.
///
/// Travels on network.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConsensusValue {
    pub ext: ExtensionPoint,
    /// Hash of the parent block's `ConsensusValue`, linking the chain.
    pub prev_hash: ConsensusValueHash,
    /// The executed header of the proposed block.
    pub proposal_header: ProposalHeader,
    /// Commits to the block's transaction payload, which is kept out of the
    /// value for compactness and travels separately in `ConsensusBasis`.
    pub proposal_txs_hash: ProposalTxsHash,
    /// Protocol upgrades selected by consensus to take effect at this block.
    pub upgrades: VecM<Upgrade, 6>,
}

/// The validation payload for a consensus value.
///
/// `ConsensusBasis` pairs a `ConsensusValue` with the transaction payload it
/// commits to but does not contain, letting a node reconstruct the `Proposal`
/// as `{ header: value.proposal_header, transactions: proposal_txs }` and
/// validate it. Gossiped and used during catchup.
///
/// Consensus agrees only on `value`; the basis has no consensus identity of its own.
///
/// Travels on network.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConsensusBasis {
    pub ext: ExtensionPoint,
    pub value: ConsensusValue,
    pub proposal_txs: ProposalTxs,
}


//
// XDR internals below this point
//


