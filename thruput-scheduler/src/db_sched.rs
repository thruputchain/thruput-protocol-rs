// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 6ab835b07c234abb4b0b50b61d687904727a4f32

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



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
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


pub const kNativeTokenAgeGroupToSchedule : u32 = 0;

pub const kNativeTokenAgeGroupAlreadyScheduled : u32 = 1;

pub const kNativeTokenAgeGroupReceived : u32 = 2;

pub const kNativeTokenAgeNumGroups : u32 = 3;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NativeTokenAge {
    pub address: ContractAddress,
    pub num_native_tokens: [uint128_xdr; kNativeTokenAgeNumGroups as usize],
    pub average_ages: [BlockHeight; kNativeTokenAgeNumGroups as usize],
}

pub type NativeTokenAgeBatch = VecM::<NativeTokenAge>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum NativeTokenAgeResult {
    // case DbSchedResultOK:
    //   NativeTokenAge native_token_age;
    DbSchedResultOK { native_token_age: NativeTokenAge } = DbSchedResult::DbSchedResultOK.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { result: i32 },
}

pub type NativeTokenAgeResults = VecM::<NativeTokenAgeResult>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum DbAddressAuthDataBatchResult {
    // case DbSchedResultOK:
    //   VecM<AddressAuthData> address_auth_data_batch;
    DbSchedResultOK { address_auth_data_batch: VecM<AddressAuthData> } = DbSchedResult::DbSchedResultOK.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { result: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DbBlockOk {
    pub storage_hash: StorageShortHash,
    pub storage_long_hash: StorageLongHash,
    pub config_hash_preimage: ConfigHashPreimage,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum DbBlockResult {
    // case DbSchedResultOK:
    //   DbBlockOk block;
    DbSchedResultOK { block: DbBlockOk } = DbSchedResult::DbSchedResultOK.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { result: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum DbTipResult {
    // case DbSchedResultOK:
    //   DbTip db_tip;
    DbSchedResultOK { db_tip: DbTip } = DbSchedResult::DbSchedResultOK.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { result: i32 },
}


/// Each handler replies with anything that encodes like the reply type.
/// Implementations can be plain `async fn`s returning a concrete reply type
/// under `#[allow(refining_impl_trait)]`.
#[allow(non_camel_case_types)]
pub trait DbSched_DbSched1: Send + Sync {    
    fn dbsched_null(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn dbsched_get_address_auth_data(&self, arg0 : BlockID, arg1 : ContractAddressBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbAddressAuthDataBatchResult>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn dbsched_get_native_token_ages(&self, arg0 : ContractAddressBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<NativeTokenAgeResults>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn dbsched_write_native_token_ages(&self, arg0 : NativeTokenAgeBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbSchedResult>, ::thruput_rpc_api::ProgramError>> + Send;
        // Examples of valid call sequences to the db:
    //
    // dbsched_create_snapshot
    //   ... calls to executor
    // dbsched_close_snapshot
    // dbsched_commit_snapshot
    //
    // dbsched_create_snapshot
    //   ... calls to executor
    // dbsched_abandon_snapshot

    // Called when creating a new block.
    // The first argument is the ID of the parent block.
    // The database will surface requests from the snapshot
    // that would be created if the parent block commits.
    // Parent block must be closed.

    fn dbsched_create_snapshot(&self, arg0 : BlockID, arg1 : BlockID, arg2 : BlockExecutionMode) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbSchedResult>, ::thruput_rpc_api::ProgramError>> + Send;
        // Commits the block with the specified block ID to disk. Should only be
    // called for e.g. finalized blocks. At a minimum, the block must be closed
    // before being committed. Any BlockID that is not a descendant of this
    // block will return an error if accessed. This BlockID must be the
    // immediate descendant of the most recently committed block.

    fn dbsched_commit_snapshot(&self, arg0 : BlockID, arg1 : ConsensusValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbBlockResult>, ::thruput_rpc_api::ProgramError>> + Send;
        // Call to close a block. This indicates that no more transactions
    // will be added to this block. The block must have been previously opened
    // with dbsched_create_snapshot().
    // A block cannot have a child opened before it is closed.
    // Error if the block has been pruned or contains an item in an invalid
    // state (due to e.g. a block validation failure).

    fn dbsched_close_snapshot(&self, arg0 : BlockID) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbBlockResult>, ::thruput_rpc_api::ProgramError>> + Send;
        // Call to abandon a block.

    fn dbsched_abandon_snapshot(&self, arg0 : BlockID) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbSchedResult>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn dbsched_load_genesis_configuration(&self, arg0 : GenesisConfiguration, arg1 : ConsensusValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbSchedResult>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn dbsched_get_tip(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbTipResult>, ::thruput_rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct DbSched_DbSched1_Dispatcher<__Handler> {
  inner: Box<__Handler>,
}

impl<__Handler: DbSched_DbSched1> DbSched_DbSched1_Dispatcher<__Handler> {
  pub fn new(inner: Box<__Handler>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<__Handler: DbSched_DbSched1> ::thruput_rpc_api::RpcHandler for DbSched_DbSched1_Dispatcher<__Handler> {
    fn prog_vers(&self) -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x88887ee7, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::thruput_rpc_api::RpcError> {
        match proc {
            // dbsched_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_null().await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_get_address_auth_data
            1 => {
                let args: (BlockID, ContractAddressBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_get_address_auth_data(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_get_native_token_ages
            2 => {
                let args: (ContractAddressBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_get_native_token_ages(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_write_native_token_ages
            3 => {
                let args: (NativeTokenAgeBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_write_native_token_ages(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_create_snapshot
            5 => {
                let args: (BlockID, BlockID, BlockExecutionMode, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_create_snapshot(args.0, args.1, args.2).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_commit_snapshot
            6 => {
                let args: (BlockID, ConsensusValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_commit_snapshot(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_close_snapshot
            7 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_close_snapshot(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_abandon_snapshot
            8 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_abandon_snapshot(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_load_genesis_configuration
            9 => {
                let args: (GenesisConfiguration, ConsensusValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_load_genesis_configuration(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // dbsched_get_tip
            10 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbsched_get_tip().await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            _ => Err(::thruput_rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct DbSched_DbSched1_Client {
    handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>,
}

impl DbSched_DbSched1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x88887ee7, 1)
    }
    pub async fn dbsched_null(&self) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 0, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_get_address_auth_data(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ContractAddressBatch> + ?Sized)) -> Result<DbAddressAuthDataBatchResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 1, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_get_native_token_ages(&self, arg0 : &(impl xdrs::EncodesAs<ContractAddressBatch> + ?Sized)) -> Result<NativeTokenAgeResults, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 2, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_write_native_token_ages(&self, arg0 : &(impl xdrs::EncodesAs<NativeTokenAgeBatch> + ?Sized)) -> Result<DbSchedResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 3, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_create_snapshot(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg2 : &(impl xdrs::EncodesAs<BlockExecutionMode> + ?Sized)) -> Result<DbSchedResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1, arg2), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 5, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_commit_snapshot(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ConsensusValue> + ?Sized)) -> Result<DbBlockResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 6, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_close_snapshot(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized)) -> Result<DbBlockResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 7, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_abandon_snapshot(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized)) -> Result<DbSchedResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 8, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_load_genesis_configuration(&self, arg0 : &(impl xdrs::EncodesAs<GenesisConfiguration> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ConsensusValue> + ?Sized)) -> Result<DbSchedResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 9, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn dbsched_get_tip(&self) -> Result<DbTipResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 10, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>> for DbSched_DbSched1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl NativeTokenAgeResult {
    #[allow(clippy::match_single_binding)]
    pub fn result(&self) -> DbSchedResult {
        match self {
            NativeTokenAgeResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl DbAddressAuthDataBatchResult {
    #[allow(clippy::match_single_binding)]
    pub fn result(&self) -> DbSchedResult {
        match self {
            DbAddressAuthDataBatchResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl DbBlockResult {
    #[allow(clippy::match_single_binding)]
    pub fn result(&self) -> DbSchedResult {
        match self {
            DbBlockResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl DbTipResult {
    #[allow(clippy::match_single_binding)]
    pub fn result(&self) -> DbSchedResult {
        match self {
            DbTipResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


