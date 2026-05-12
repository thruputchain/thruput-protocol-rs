// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: bca4204390034cf31ecbfd4ca04619eab347032d

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

// An internal unique ID describing a block.
pub type BlockID = u64;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DbTip {
    pub height: BlockHeight,
    pub block_hash: BlockHash,
    pub events_hash: SHA256Hash,
    pub db_hash: DbHash,
    pub id: BlockID,
}

// Internal unique ID describing a transaction.
// Can be reused across blocks but cannot be reused within one block.
pub type TransactionID = u64;

// This is missing error codes, and is here purely as a placeholder.
// These are some of the error codes I suspect we will need, but these
// are not organized in any particular manner.
//
// TODO(geoff): This probably should get moved to a different file?
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct RPCError(pub i32);

impl RPCError {
    pub const kRPCNone: Self = Self(0);

    // Lookup on an item in the database that does not exist.
    // Used by any database lookup RPC, not just for one type.
    pub const kRPCNotFound: Self = Self(1);

    // BlockID does not exist, or is not in correct state
    pub const kRPCBadBlockID: Self = Self(3);

    // bad storage modification
    pub const kRPCStorageWriteFailed: Self = Self(4);

    // tx validation or proposal failed
    pub const kRPCTransactionFailed: Self = Self(5);

    // Unimplemented RPC
    pub const kRPCUnimplemented: Self = Self(6);

    // Unrecoverable error in the server (i.e. OOM, DB corruption)
    pub const kRPCUnrecoverable: Self = Self(7);
}
impl From<i32> for RPCError {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for RPCError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kRPCNone.0 => {
                write!(f, "RPCError::kRPCNone")
            },
            x if x == Self::kRPCNotFound.0 => {
                write!(f, "RPCError::kRPCNotFound")
            },
            x if x == Self::kRPCBadBlockID.0 => {
                write!(f, "RPCError::kRPCBadBlockID")
            },
            x if x == Self::kRPCStorageWriteFailed.0 => {
                write!(f, "RPCError::kRPCStorageWriteFailed")
            },
            x if x == Self::kRPCTransactionFailed.0 => {
                write!(f, "RPCError::kRPCTransactionFailed")
            },
            x if x == Self::kRPCUnimplemented.0 => {
                write!(f, "RPCError::kRPCUnimplemented")
            },
            x if x == Self::kRPCUnrecoverable.0 => {
                write!(f, "RPCError::kRPCUnrecoverable")
            },
            _ => {
                write!(f, "RPCError({})", self.0)
            }
        }
    }
}


// The mode in which a block executes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct BlockExecutionMode(pub i32);

impl BlockExecutionMode {
    pub const kBlockExecutionModeValidation: Self = Self(0);
    pub const kBlockExecutionModeProposal: Self = Self(1);
}
impl From<i32> for BlockExecutionMode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for BlockExecutionMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kBlockExecutionModeValidation.0 => {
                write!(f, "BlockExecutionMode::kBlockExecutionModeValidation")
            },
            x if x == Self::kBlockExecutionModeProposal.0 => {
                write!(f, "BlockExecutionMode::kBlockExecutionModeProposal")
            },
            _ => {
                write!(f, "BlockExecutionMode({})", self.0)
            }
        }
    }
}


pub type StorageCost = int128_xdr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExpectedStorageCost {
    // case kRPCNone:
    //   StorageCost storage_cost;
    kRPCNone { storage_cost: StorageCost } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    Default { error: i32 },
}

// Is this the XDR pattern we want?
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExpectedContract {
    // case kRPCNone:
    //   Contract contract;
    kRPCNone { contract: Contract } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExpectedDeployment {
    // case kRPCNone:
    //   DeployedContractHash deployed_hash;
    kRPCNone { deployed_hash: DeployedContractHash } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExpectedProposalData {
    // case kRPCNone:
    //   NondeterministicProposalData proposal_data;
    kRPCNone { proposal_data: NondeterministicProposalData } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExpectedRetrocausalQueueBlock {
    // case kRPCNone:
    //   RetrocausalQueueBlock queue_block;
    kRPCNone { queue_block: RetrocausalQueueBlock } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExpectedQueueEntry {
    // case kRPCNone:
    //   QueueEntry queue_entry;
    kRPCNone { queue_entry: QueueEntry } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExpectedDBTip {
    // case kRPCNone:
    //   DbTip db_tip;
    kRPCNone { db_tip: DbTip } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    Default { error: i32 },
}

/// Typedef for a list of contract addresses.
pub type ContractAddressBatch = VecM::<ContractAddress>;


//
// XDR internals below this point
//

impl ExpectedStorageCost {
    pub fn error(&self) -> RPCError {
        match self {
            ExpectedStorageCost::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl ExpectedContract {
    pub fn error(&self) -> RPCError {
        match self {
            ExpectedContract::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl ExpectedDeployment {
    pub fn error(&self) -> RPCError {
        match self {
            ExpectedDeployment::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl ExpectedProposalData {
    pub fn error(&self) -> RPCError {
        match self {
            ExpectedProposalData::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl ExpectedRetrocausalQueueBlock {
    pub fn error(&self) -> RPCError {
        match self {
            ExpectedRetrocausalQueueBlock::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl ExpectedQueueEntry {
    pub fn error(&self) -> RPCError {
        match self {
            ExpectedQueueEntry::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl ExpectedDBTip {
    pub fn error(&self) -> RPCError {
        match self {
            ExpectedDBTip::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


