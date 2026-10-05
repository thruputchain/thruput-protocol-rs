// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: dcb5d7af8f39be558b0bfe56c0d0bb9163cce9f2

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



// ShardCard uniquely describes a database shard instance and its initial
// state.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ShardCard {
    pub protocol: ProtocolCommit,
    pub upid: UPID,
    pub block_height: BlockHeight,
    // This is the db address that the executor should connect to.
    pub address_for_exec: ServerAddress,
    // This is the db address that the scheduler should connect to.
    pub address_for_sched: ServerAddress,
    // This is the db address that the auth server should connect to.
    pub address_for_auth: ServerAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Delta {
    pub address: ContractAddress,
    pub hog_delta: int128_xdr,
}

// One slice of the native-token balance deltas a committed block finalized.
//
// hog_db publishes a block's deltas as a run of chunk_count chunks, all for
// the same block, after the block is committed. hog_db chooses the chunk
// size; the wire bounds a chunk only by the link's record cap. The chunks of
// a run may be sent concurrently and over several connections, so they can
// arrive in any order; hog_db starts the next block's run only after every
// chunk of this one is acknowledged, so the mempool never has more than one
// block's run in flight. An address appears in at most one chunk of a block's
// run (a shard holds one balance per address) and every delta is nonzero. The
// mempool treats the block's transfers as complete only once every chunk of
// the run has arrived; nothing that gates on a block's transfers may open
// before that.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HogsTransferredChunk {
    // The height of the block in which these transfers occurred. The scheduler
    // uses this height to update average hog ages.
    pub block: BlockHeight,
    // This chunk's position in the block's run, from 0, and the run's length.
    // Every chunk of a run carries the same chunk_count, and each index in
    // [0, chunk_count) is sent exactly once. A block that finalized no nonzero
    // delta is a run of one empty chunk, so every committed block still sends
    // a run.
    pub chunk_index: u32,
    pub chunk_count: u32,
    pub deltas: VecM<Delta>,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SchedDb_SchedDb1: Send + Sync {        // [scheddb_intro_shard] is called by a db shard to introduce itself to the
    // system.

    async fn scheddb_intro_shard(&self, arg0 : ShardCard) -> Result<(), ::rpc_api::ProgramError>;
        // Retired numbers, never to be reused, so that a peer built against one
    // fails to dispatch rather than misparse a newer payload:
    //   2: scheddb_hogs_transferred(HogsTransferred), a block's deltas in one
    //      unbounded record, which a block with enough distinct recipients
    //      grows past any single-record cap (hogd#2332).
    //   3: scheddb_changed_addresses(BlockHeight, ContractAddressBatch),
    //      removed in e413dc9. Its payload also opens with a block height and
    //      a vector length, so a chunk sent on 3 would decode there as
    //      addresses.

    // Deliver one chunk of a committed block's balance deltas. With several db
    // shards the mempool completes the block once every registered shard has
    // sent its run's last chunk.

    async fn scheddb_hogs_transferred_chunk(&self, arg0 : HogsTransferredChunk) -> Result<(), ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct SchedDb_SchedDb1_Dispatcher {
  inner: Box<dyn SchedDb_SchedDb1>,
}

impl SchedDb_SchedDb1_Dispatcher {
  pub fn new(inner: Box<dyn SchedDb_SchedDb1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for SchedDb_SchedDb1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x7ee79998, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // scheddb_intro_shard
            1 => {
                let args: (ShardCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scheddb_intro_shard(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scheddb_hogs_transferred_chunk
            4 => {
                let args: (HogsTransferredChunk, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scheddb_hogs_transferred_chunk(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SchedDb_SchedDb1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl SchedDb_SchedDb1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x7ee79998, 1)
    }
    pub async fn scheddb_intro_shard(&self, arg0 : &ShardCard) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scheddb_hogs_transferred_chunk(&self, arg0 : &HogsTransferredChunk) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 4, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for SchedDb_SchedDb1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


