// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 6d7793493815cb08a683232d5b391d635721c369

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
#[cfg(feature = "xdrs")]
use xdrs::*;
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

/// Inputs fixed before executing a candidate block.
///
/// Supplied by the coordinator when it asks the scheduler to build a proposal;
/// the parent block is identified by the surrounding RPC. Both fields are
/// copied into the resulting `ProposalHeader`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ProposalContext {
    // Network protocol version.
    pub protocol: Protocol,
    /// Block proposer. Nodes missing block data fetch it from the proposer.
    pub proposer: ValidatorID,
    /// The block's timepoint, fixed by the coordinator before execution begins.
    pub block_timepoint: TimePoint,
}

/// Header of a proposed block.
///
/// The consensus-facing summary of block execution: the fixed inputs of the
/// proposal together with the deterministic outputs of executing its
/// transactions. `ConsensusValue` embeds this header directly, so every field
/// is part of the block identity.
///
/// Travels on network.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ProposalHeader {
    pub ext: ExtensionPoint,
    /// The hash of the network name, separating this network's blocks from
    /// those of other networks.
    pub network_hash: NetworkHash,
    /// Block proposer. Nodes missing block data fetch it from the proposer.
    pub proposer: ValidatorID,
    /// The block's timepoint, fixed by the coordinator before execution begins.
    pub block_timepoint: TimePoint,
    /// The height of this block.
    pub block_height: BlockHeight,
    /// The Merkle root hash of the transactions.
    pub txs_hash: TxsHash,
    /// The hash of the storage contents after this block is applied.
    pub storage_hash: StorageShortHash,
    /// The Merkle root hash of the events.
    pub events_hash: EventsHash,
}


//
// XDR internals below this point
//


