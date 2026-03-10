// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: c95abcc628bcb7818be237c2f2081a6b45f156b4

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

pub type BlockHash = SHA256Hash;

pub type DbHash = [u8; 2048 as usize];

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DbTip {
    pub height: BlockHeight,
    pub block_hash: BlockHash,
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
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum RPCError {
    kRPCNone = 0 as i32,

    // Lookup on an item in the database that does not exist.
    // Used by any database lookup RPC, not just for one type.
    kRPCNotFound = 1 as i32,

    // BlockID does not exist, or is not in correct state
    kRPCBadBlockID = 3 as i32,

    // bad storage modification
    kRPCStorageWriteFailed = 4 as i32,

    // tx validation or proposal failed
    kRPCTransactionFailed = 5 as i32,

    // Unimplemented RPC
    kRPCUnimplemented = 6 as i32,

    // Unrecoverable error in the server (i.e. OOM, DB corruption)
    kRPCUnrecoverable = 7 as i32,
}
impl TryFrom<i32> for RPCError {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kRPCNone),
            1 => Ok(Self::kRPCNotFound),
            3 => Ok(Self::kRPCBadBlockID),
            4 => Ok(Self::kRPCStorageWriteFailed),
            5 => Ok(Self::kRPCTransactionFailed),
            6 => Ok(Self::kRPCUnimplemented),
            7 => Ok(Self::kRPCUnrecoverable),
            _ => Err(()),
        }
    }
}


// The mode in which a block executes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum BlockExecutionMode {
    kBlockExecutionModeValidation = 0 as i32,
    kBlockExecutionModeProposal = 1 as i32,
}
impl TryFrom<i32> for BlockExecutionMode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kBlockExecutionModeValidation),
            1 => Ok(Self::kBlockExecutionModeProposal),
            _ => Err(()),
        }
    }
}


pub type StorageCost = int128_xdr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExpectedStorageCost {
    // case kRPCNone:
    //   StorageCost storage_cost;
    kRPCNone { storage_cost: StorageCost } = RPCError::kRPCNone as i32,
    // default:
    //   void;
    Default { error: i32 },
}

// Is this the XDR pattern we want?
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExpectedContract {
    // case kRPCNone:
    //   Contract contract;
    kRPCNone { contract: Contract } = RPCError::kRPCNone as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExpectedDeployment {
    // case kRPCNone:
    //   DeployedContractHash deployed_hash;
    kRPCNone { deployed_hash: DeployedContractHash } = RPCError::kRPCNone as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExpectedProposalData {
    // case kRPCNone:
    //   NondeterministicProposalData proposal_data;
    kRPCNone { proposal_data: NondeterministicProposalData } = RPCError::kRPCNone as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExpectedStorageItem {
    // case kRPCNone:
    //   WireStorageItem item;
    kRPCNone { item: WireStorageItem } = RPCError::kRPCNone as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExpectedRetrocausalQueueBlock {
    // case kRPCNone:
    //   RetrocausalQueueBlock queue_block;
    kRPCNone { queue_block: RetrocausalQueueBlock } = RPCError::kRPCNone as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExpectedQueueEntry {
    // case kRPCNone:
    //   QueueEntry queue_entry;
    kRPCNone { queue_entry: QueueEntry } = RPCError::kRPCNone as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExpectedDBTip {
    // case kRPCNone:
    //   DbTip db_tip;
    kRPCNone { db_tip: DbTip } = RPCError::kRPCNone as i32,
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
    pub fn error(&self) -> i32 {
        match self {
            ExpectedStorageCost::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExpectedContract {
    pub fn error(&self) -> i32 {
        match self {
            ExpectedContract::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExpectedDeployment {
    pub fn error(&self) -> i32 {
        match self {
            ExpectedDeployment::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExpectedProposalData {
    pub fn error(&self) -> i32 {
        match self {
            ExpectedProposalData::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExpectedStorageItem {
    pub fn error(&self) -> i32 {
        match self {
            ExpectedStorageItem::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExpectedRetrocausalQueueBlock {
    pub fn error(&self) -> i32 {
        match self {
            ExpectedRetrocausalQueueBlock::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExpectedQueueEntry {
    pub fn error(&self) -> i32 {
        match self {
            ExpectedQueueEntry::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExpectedDBTip {
    pub fn error(&self) -> i32 {
        match self {
            ExpectedDBTip::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


