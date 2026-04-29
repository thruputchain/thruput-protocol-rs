// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: fa6e8c614a824e73c3e9dd1fe0e5a478096c3fce

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DbSchedResult(pub i32);

impl DbSchedResult {
    pub const DbSchedResultOK: Self = Self(0);
    pub const DbSchedResultError: Self = Self(1);
    pub const DbSchedResultInvalidParentId: Self = Self(2);
}
impl From<i32> for DbSchedResult {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for DbSchedResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::DbSchedResultOK.0 => {
                write!(f, "DbSchedResult::DbSchedResultOK")
            },
            x if x == Self::DbSchedResultError.0 => {
                write!(f, "DbSchedResult::DbSchedResultError")
            },
            x if x == Self::DbSchedResultInvalidParentId.0 => {
                write!(f, "DbSchedResult::DbSchedResultInvalidParentId")
            },
            _ => {
                write!(f, "DbSchedResult({})", self.0)
            }
        }
    }
}


pub const kHogAgeGroupToSchedule : u32 = 0;

pub const kHogAgeGroupAlreadyScheduled : u32 = 1;

pub const kHogAgeGroupReceived : u32 = 2;

pub const kHogAgeNumGroups : u32 = 3;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HogAge {
    pub address: ContractAddress,
    pub num_hogs: [uint128_xdr; kHogAgeNumGroups as usize],
    pub average_ages: [BlockHeight; kHogAgeNumGroups as usize],
}

pub type HogAgeBatch = VecM::<HogAge>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
#[non_exhaustive]
pub enum HogAgeResult {
    // case DbSchedResultOK:
    //   HogAge hog_age;
    DbSchedResultOK { hog_age: HogAge } = DbSchedResult::DbSchedResultOK.0 as i32,
    // default:
    //   void;
    Default { result: i32 },
}

pub type HogAgeResults = VecM::<HogAgeResult>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
#[non_exhaustive]
pub enum DbAddressAuthDataBatchResult {
    // case DbSchedResultOK:
    //   VecM<AddressAuthData> address_auth_data_batch;
    DbSchedResultOK { address_auth_data_batch: VecM<AddressAuthData> } = DbSchedResult::DbSchedResultOK.0 as i32,
    // default:
    //   void;
    Default { result: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
#[non_exhaustive]
pub enum DbBlockResult {
    // case DbSchedResultOK:
    //   DbHash db_hash;
    DbSchedResultOK { db_hash: DbHash } = DbSchedResult::DbSchedResultOK.0 as i32,
    // default:
    //   void;
    Default { result: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DbShardResult(pub i32);

impl DbShardResult {
    pub const ShardOK: Self = Self(0);
}
impl From<i32> for DbShardResult {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for DbShardResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::ShardOK.0 => {
                write!(f, "DbShardResult::ShardOK")
            },
            _ => {
                write!(f, "DbShardResult({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
#[non_exhaustive]
pub enum DbTipResult {
    // case DbSchedResultOK:
    //   DbTip db_tip;
    DbSchedResultOK { db_tip: DbTip } = DbSchedResult::DbSchedResultOK.0 as i32,
    // default:
    //   void;
    Default { result: i32 },
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait DbSched_DbSched1: Send + Sync {    
    async fn dbsched_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn dbsched_get_address_auth_data(&self, arg0 : BlockID, arg1 : ContractAddressBatch) -> Result<DbAddressAuthDataBatchResult, ::rpc_driver::ProgramError>;
    
    async fn dbsched_get_hog_ages(&self, arg0 : ContractAddressBatch) -> Result<HogAgeResults, ::rpc_driver::ProgramError>;
    
    async fn dbsched_write_hog_ages(&self, arg0 : HogAgeBatch) -> Result<DbSchedResult, ::rpc_driver::ProgramError>;
        // Examples of valid call sequences to the db:
    //
    // dbsched_create_snapshot | dbsched_open_snapshot
    //   ... calls to executor
    // dbsched_commit_snapshot
    //
    // dbsched_create_snapshot | dbsched_open_snapshot
    //   ... calls to executor
    // dbsched_close_snapshot
    //
    // dbsched_create_snapshot | dbsched_open_snapshot
    //   ... calls to executor
    // dbsched_abandon_snapshot

    // Called when creating a new block.
    // The first argument is the ID of the parent block.
    // The database will surface requests from the snapshot
    // that would be created if the parent block commits.
    // Parent block must be closed.

    async fn dbsched_create_snapshot(&self, arg0 : BlockID, arg1 : BlockID, arg2 : BlockExecutionMode) -> Result<DbSchedResult, ::rpc_driver::ProgramError>;
        // Commits the block with the specified block ID to disk. Should only be
    // called for e.g. finalized blocks. At a minimum, the block must be closed
    // before being committed. Any BlockID that is not a descendant of this
    // block will return an error if accessed. This BlockID must be the
    // immediate descendant of the most recently committed block.

    async fn dbsched_commit_snapshot(&self, arg0 : BlockID, arg1 : BlockHash, arg2 : SHA256Hash) -> Result<DbBlockResult, ::rpc_driver::ProgramError>;
        // Call to close a block. This indicates that no more transactions
    // will be added to this block. The block must have been previously opened
    // with open_database_snapshot().
    // A block cannot have a child opened before it is closed.
    // Error if the block has been pruned or contains an item in an invalid
    // state (due to e.g. a block validation failure).

    async fn dbsched_close_snapshot(&self, arg0 : BlockID) -> Result<DbBlockResult, ::rpc_driver::ProgramError>;
        // Call to abandon a block.

    async fn dbsched_abandon_snapshot(&self, arg0 : BlockID) -> Result<DbSchedResult, ::rpc_driver::ProgramError>;
        // Load genesis configuration data into storage. The second argument is the
    // network ID. The third argument is the txset hash and the third argument
    // is the events hash. Since the genesis block does not contain any
    // transactions, both of these hashes will generally be the root hash of an
    // empty Merkle trie.
    //
    // This RPC calculates storage keys and stores allocation amounts as
    // bytestrings.

    async fn dbsched_load_genesis_configuration(&self, arg0 : GenesisConfiguration, arg1 : u64, arg2 : SHA256Hash, arg3 : SHA256Hash) -> Result<DbSchedResult, ::rpc_driver::ProgramError>;
    
    async fn dbsched_get_tip(&self) -> Result<DbTipResult, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct DbSched_DbSched1_Dispatcher {
  inner: Box<dyn DbSched_DbSched1>,
}

impl DbSched_DbSched1_Dispatcher {
  pub fn new(inner: Box<dyn DbSched_DbSched1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for DbSched_DbSched1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x88887ee7, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // dbsched_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_get_address_auth_data
            1 => {
                let args: (BlockID, ContractAddressBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_get_address_auth_data(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_get_hog_ages
            2 => {
                let args: (ContractAddressBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_get_hog_ages(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_write_hog_ages
            3 => {
                let args: (HogAgeBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_write_hog_ages(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_create_snapshot
            5 => {
                let args: (BlockID, BlockID, BlockExecutionMode, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_create_snapshot(args.0, args.1, args.2).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_commit_snapshot
            6 => {
                let args: (BlockID, BlockHash, SHA256Hash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_commit_snapshot(args.0, args.1, args.2).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_close_snapshot
            7 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_close_snapshot(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_abandon_snapshot
            8 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_abandon_snapshot(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_load_genesis_configuration
            9 => {
                let args: (GenesisConfiguration, u64, SHA256Hash, SHA256Hash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_load_genesis_configuration(args.0, args.1, args.2, args.3).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dbsched_get_tip
            10 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_get_tip().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct DbSched_DbSched1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl DbSched_DbSched1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x88887ee7, 1)
    }
    pub async fn dbsched_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_get_address_auth_data(&self, arg0 : &BlockID, arg1 : &ContractAddressBatch) -> Result<DbAddressAuthDataBatchResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_get_hog_ages(&self, arg0 : &ContractAddressBatch) -> Result<HogAgeResults, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_write_hog_ages(&self, arg0 : &HogAgeBatch) -> Result<DbSchedResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 3, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_create_snapshot(&self, arg0 : &BlockID, arg1 : &BlockID, arg2 : &BlockExecutionMode) -> Result<DbSchedResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1, &arg2)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 5, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_commit_snapshot(&self, arg0 : &BlockID, arg1 : &BlockHash, arg2 : &SHA256Hash) -> Result<DbBlockResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1, &arg2)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 6, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_close_snapshot(&self, arg0 : &BlockID) -> Result<DbBlockResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 7, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_abandon_snapshot(&self, arg0 : &BlockID) -> Result<DbSchedResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 8, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_load_genesis_configuration(&self, arg0 : &GenesisConfiguration, arg1 : u64, arg2 : &SHA256Hash, arg3 : &SHA256Hash) -> Result<DbSchedResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, arg1, &arg2, &arg3)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 9, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dbsched_get_tip(&self) -> Result<DbTipResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//

impl HogAgeResult {
    pub fn result(&self) -> DbSchedResult {
        match self {
            HogAgeResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl DbAddressAuthDataBatchResult {
    pub fn result(&self) -> DbSchedResult {
        match self {
            DbAddressAuthDataBatchResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl DbBlockResult {
    pub fn result(&self) -> DbSchedResult {
        match self {
            DbBlockResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl DbTipResult {
    pub fn result(&self) -> DbSchedResult {
        match self {
            DbTipResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


