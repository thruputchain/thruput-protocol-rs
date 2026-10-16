// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0c2c59fb8ebeb0d6d37a1dad9cf88c4f22187ce6

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
pub struct AuthErrorCode(pub i32);

impl AuthErrorCode {
    pub const kAuthSuccess: Self = Self(0);
    pub const kAuthFailed: Self = Self(1);

    // The query is for a block height that is not the most recent committed block height.
    // This can happen if a query is made for an old block height,
    // or if (concurrent with the query) a block commits and updates to the auth data begin to stream in.
    // The verification can be retried in either case.
    // TODO(geoff): We could switch this around, to retry at the higher block height.
    pub const kAuthStaleBlockHeight: Self = Self(2);

    // Either an internal auth error or forwarded from an internal DB error
    pub const kAuthInternal: Self = Self(3);
}
impl From<i32> for AuthErrorCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for AuthErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kAuthSuccess.0 => {
                write!(f, "AuthErrorCode::kAuthSuccess")
            },
            x if x == Self::kAuthFailed.0 => {
                write!(f, "AuthErrorCode::kAuthFailed")
            },
            x if x == Self::kAuthStaleBlockHeight.0 => {
                write!(f, "AuthErrorCode::kAuthStaleBlockHeight")
            },
            x if x == Self::kAuthInternal.0 => {
                write!(f, "AuthErrorCode::kAuthInternal")
            },
            _ => {
                write!(f, "AuthErrorCode({})", self.0)
            }
        }
    }
}


pub type AuthErrorCodeBatch = VecM::<AuthErrorCode>;


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait AuthMempool_AuthMempool1: Send + Sync {        // Verify a list of transactions.  This version returns a result for each transaction.
    // The BlockHeight is the height of the most recently committed block,
    // i.e. the block height of the block that produced the snapshot auth data against which
    // the transactions are to be verified.

    async fn authmempool_verify_transactions(&self, arg0 : BlockHeight, arg1 : SignedTransactionBatch) -> Result<AuthErrorCodeBatch, ::rpc_api::ProgramError>;
        // Verify a batch of transactions.  This version returns a single result for the batch,
    // i.e. are all correct or is there at least one invalid transaction?
    // This version should be called when validating a proposal, in which if even one
    // transaction is invalid then the whole block can be rejected.
    // This version can use batch verification to speed up the verification.

    async fn authmempool_verify_batch(&self, arg0 : BlockHeight, arg1 : SignedTransactionBatch) -> Result<AuthErrorCode, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct AuthMempool_AuthMempool1_Dispatcher {
  inner: Box<dyn AuthMempool_AuthMempool1>,
}

impl AuthMempool_AuthMempool1_Dispatcher {
  pub fn new(inner: Box<dyn AuthMempool_AuthMempool1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for AuthMempool_AuthMempool1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa87aa7, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // authmempool_verify_transactions
            2 => {
                let args: (BlockHeight, SignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.authmempool_verify_transactions(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // authmempool_verify_batch
            3 => {
                let args: (BlockHeight, SignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.authmempool_verify_batch(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct AuthMempool_AuthMempool1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl AuthMempool_AuthMempool1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa87aa7, 1)
    }
    pub async fn authmempool_verify_transactions(&self, arg0 : &(impl xdrs::EncodesAs<BlockHeight> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SignedTransactionBatch> + ?Sized)) -> Result<AuthErrorCodeBatch, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn authmempool_verify_batch(&self, arg0 : &(impl xdrs::EncodesAs<BlockHeight> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SignedTransactionBatch> + ?Sized)) -> Result<AuthErrorCode, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 3, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for AuthMempool_AuthMempool1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct KeyChange {
    pub address: ContractAddress,
    pub auth_data: Option<AddressAuthData>,
}

pub type KeyChangeBatch = VecM::<KeyChange>;

// A committed block's auth-data changes reach the auth server as a run of
// chunks, each a KeyChangeChunk, because the one-record form grew with the
// number of accounts whose auth data a block changed (hogd#3146). hog_db
// chooses the chunk size; the wire bounds a chunk only by the link's record
// cap. The chunks of a run may be sent concurrently and over several
// connections, so they can arrive in any order, and hog_db may have the runs
// of several heights in flight at once, so the chunks of different heights
// can interleave. The auth server treats a height's changes as complete only
// once every chunk of its run has arrived, and applies heights in order.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct KeyChangeChunk {
    // The height of the block that made these changes.
    pub height: BlockHeight,
    // This chunk's position in the height's run, from 0, and the run's length.
    // Every chunk of a run carries the same chunk_count, and each index in
    // [0, chunk_count) is sent exactly once. A block that changed no auth data
    // is a run of one empty chunk, so every committed block still sends a run
    // and the auth server's height advances with every block.
    pub chunk_index: u32,
    pub chunk_count: u32,
    pub changes: VecM<KeyChange>,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait AuthDB_AuthDB1: Send + Sync {        // Retired number, never to be reused, so that a peer built against it
    // fails to dispatch rather than misparse a newer payload:
    //   1: authdb_forward_key_changes(BlockHeight, KeyChangeBatch), a block's
    //      auth-data changes in one unbounded record, which a block that
    //      changes enough accounts' auth data grows past any single-record
    //      cap (hogd#3146).

    // Deliver one chunk of a committed block's auth-data changes to the auth
    // server.

    async fn authdb_forward_key_changes_chunk(&self, arg0 : KeyChangeChunk) -> Result<(), ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct AuthDB_AuthDB1_Dispatcher {
  inner: Box<dyn AuthDB_AuthDB1>,
}

impl AuthDB_AuthDB1_Dispatcher {
  pub fn new(inner: Box<dyn AuthDB_AuthDB1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for AuthDB_AuthDB1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa87aa8, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // authdb_forward_key_changes_chunk
            2 => {
                let args: (KeyChangeChunk, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.authdb_forward_key_changes_chunk(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct AuthDB_AuthDB1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl AuthDB_AuthDB1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa87aa8, 1)
    }
    pub async fn authdb_forward_key_changes_chunk(&self, arg0 : &(impl xdrs::EncodesAs<KeyChangeChunk> + ?Sized)) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for AuthDB_AuthDB1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DBAuthError(pub i32);

impl DBAuthError {
    pub const kDBAuthSuccess: Self = Self(0);
    pub const kDBAuthNotFound: Self = Self(1);
    pub const kDBAuthStaleBlockHeight: Self = Self(2);
    pub const kDBAuthInternal: Self = Self(3);
}
impl From<i32> for DBAuthError {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for DBAuthError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kDBAuthSuccess.0 => {
                write!(f, "DBAuthError::kDBAuthSuccess")
            },
            x if x == Self::kDBAuthNotFound.0 => {
                write!(f, "DBAuthError::kDBAuthNotFound")
            },
            x if x == Self::kDBAuthStaleBlockHeight.0 => {
                write!(f, "DBAuthError::kDBAuthStaleBlockHeight")
            },
            x if x == Self::kDBAuthInternal.0 => {
                write!(f, "DBAuthError::kDBAuthInternal")
            },
            _ => {
                write!(f, "DBAuthError({})", self.0)
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
pub enum DBAuthResult {
    // case kDBAuthSuccess:
    //   AddressAuthData auth_data;
    kDBAuthSuccess { auth_data: AddressAuthData } = DBAuthError::kDBAuthSuccess.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { error: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DBAuthIntroOutcome(pub i32);

impl DBAuthIntroOutcome {
    pub const kIntroSuccess: Self = Self(0);
    pub const kIntroFailure: Self = Self(1);
}
impl From<i32> for DBAuthIntroOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for DBAuthIntroOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kIntroSuccess.0 => {
                write!(f, "DBAuthIntroOutcome::kIntroSuccess")
            },
            x if x == Self::kIntroFailure.0 => {
                write!(f, "DBAuthIntroOutcome::kIntroFailure")
            },
            _ => {
                write!(f, "DBAuthIntroOutcome({})", self.0)
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
pub enum DBAuthIntroResult {
    // case kIntroSuccess:
    //   BlockHeight current_height;
    kIntroSuccess { current_height: BlockHeight } = DBAuthIntroOutcome::kIntroSuccess.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}

pub type DBAuthResultBatch = VecM::<DBAuthResult>;


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait DBAuth_DBAuth1: Send + Sync {        // Get the auth data for a single address.
    // If some block concurrently commits which modifies the auth data,
    // then this call can return kDBAuthStaleBlockHeight.

    async fn authdb_get_auth_data(&self, arg0 : BlockHeight, arg1 : ContractAddress) -> Result<DBAuthResult, ::rpc_api::ProgramError>;
        // Get the auth data for a batch of addresses.

    async fn authdb_get_auth_data_batch(&self, arg0 : BlockHeight, arg1 : ContractAddressBatch) -> Result<DBAuthResultBatch, ::rpc_api::ProgramError>;
        // Introduce an auth server to a db shard,
    // and subscribe to updates to auth data

    async fn dbauth_intro(&self, arg0 : AuthCard) -> Result<DBAuthIntroResult, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct DBAuth_DBAuth1_Dispatcher {
  inner: Box<dyn DBAuth_DBAuth1>,
}

impl DBAuth_DBAuth1_Dispatcher {
  pub fn new(inner: Box<dyn DBAuth_DBAuth1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for DBAuth_DBAuth1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa87aa9, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // authdb_get_auth_data
            0 => {
                let args: (BlockHeight, ContractAddress, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.authdb_get_auth_data(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // authdb_get_auth_data_batch
            1 => {
                let args: (BlockHeight, ContractAddressBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.authdb_get_auth_data_batch(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // dbauth_intro
            2 => {
                let args: (AuthCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbauth_intro(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct DBAuth_DBAuth1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl DBAuth_DBAuth1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa87aa9, 1)
    }
    pub async fn authdb_get_auth_data(&self, arg0 : &(impl xdrs::EncodesAs<BlockHeight> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ContractAddress> + ?Sized)) -> Result<DBAuthResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn authdb_get_auth_data_batch(&self, arg0 : &(impl xdrs::EncodesAs<BlockHeight> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ContractAddressBatch> + ?Sized)) -> Result<DBAuthResultBatch, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn dbauth_intro(&self, arg0 : &(impl xdrs::EncodesAs<AuthCard> + ?Sized)) -> Result<DBAuthIntroResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for DBAuth_DBAuth1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IntroAuthOutcome(pub i32);

impl IntroAuthOutcome {
    pub const kSuccess: Self = Self(0);
    pub const kFailure: Self = Self(1);
}
impl From<i32> for IntroAuthOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for IntroAuthOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kSuccess.0 => {
                write!(f, "IntroAuthOutcome::kSuccess")
            },
            x if x == Self::kFailure.0 => {
                write!(f, "IntroAuthOutcome::kFailure")
            },
            _ => {
                write!(f, "IntroAuthOutcome({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct MempoolAuthIntroSuccess {
    pub shards: VecM<ShardCard>,
    pub network_hash: NetworkHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum IntroAuthResult {
    // case kSuccess:
    //   MempoolAuthIntroSuccess success;
    kSuccess { success: MempoolAuthIntroSuccess } = IntroAuthOutcome::kSuccess.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait MempoolAuth_MempoolAuth1: Send + Sync {    
    async fn mempoolauth_intro_auth(&self, arg0 : AuthCard) -> Result<IntroAuthResult, ::rpc_api::ProgramError>;
        // Forward updated key changes from the auth server to the mempool.

    async fn mempoolauth_auth_data_changed(&self, arg0 : BlockHeight, arg1 : KeyChangeBatch) -> Result<(), ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct MempoolAuth_MempoolAuth1_Dispatcher {
  inner: Box<dyn MempoolAuth_MempoolAuth1>,
}

impl MempoolAuth_MempoolAuth1_Dispatcher {
  pub fn new(inner: Box<dyn MempoolAuth_MempoolAuth1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for MempoolAuth_MempoolAuth1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa87aaa, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // mempoolauth_intro_auth
            0 => {
                let args: (AuthCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.mempoolauth_intro_auth(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // mempoolauth_auth_data_changed
            1 => {
                let args: (BlockHeight, KeyChangeBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.mempoolauth_auth_data_changed(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct MempoolAuth_MempoolAuth1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl MempoolAuth_MempoolAuth1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa87aaa, 1)
    }
    pub async fn mempoolauth_intro_auth(&self, arg0 : &(impl xdrs::EncodesAs<AuthCard> + ?Sized)) -> Result<IntroAuthResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn mempoolauth_auth_data_changed(&self, arg0 : &(impl xdrs::EncodesAs<BlockHeight> + ?Sized), arg1 : &(impl xdrs::EncodesAs<KeyChangeBatch> + ?Sized)) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for MempoolAuth_MempoolAuth1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl DBAuthResult {
    pub fn error(&self) -> DBAuthError {
        match self {
            DBAuthResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl DBAuthIntroResult {
    pub fn outcome(&self) -> DBAuthIntroOutcome {
        match self {
            DBAuthIntroResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl IntroAuthResult {
    pub fn outcome(&self) -> IntroAuthOutcome {
        match self {
            IntroAuthResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


