// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 4191665d9425e58c121d3cbb5a413a3ae736c24f

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

/// The block commitment externalized by consensus.
///
/// `ConsensusValue` is the small value that systems use to identify
/// a finalized Groundhog block without the full transaction payload.
/// Its XDR hash is the block identity (`ConsensusValueHash`), so every
/// field is consensus-critical.
///
/// Travels on network.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConsensusValue {
    // Network protocol version.
    pub protocol: Protocol,
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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
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


