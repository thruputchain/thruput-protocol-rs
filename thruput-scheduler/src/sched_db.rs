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
    pub native_token_delta: int128_xdr,
}

// One slice of the native-token balance deltas a committed block finalized.
//
// The db publishes a block's deltas as a run of chunk_count chunks, all for
// the same block, after the block is committed. The db chooses the chunk
// size; the wire bounds a chunk only by the link's record cap. The chunks of
// a run may be sent concurrently and over several connections, so they can
// arrive in any order; the db starts the next block's run only after every
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
pub struct NativeTokensTransferredChunk {
    // The height of the block in which these transfers occurred. The scheduler
    // uses this height to update average native-token ages.
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


/// Each handler replies with anything that encodes like the reply type.
/// Implementations can be plain `async fn`s returning a concrete reply type
/// under `#[allow(refining_impl_trait)]`.
#[allow(non_camel_case_types)]
pub trait SchedDb_SchedDb1: Send + Sync {        // [scheddb_intro_shard] is called by a db shard to introduce itself to the
    // system.

    fn scheddb_intro_shard(&self, arg0 : ShardCard) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
        // Retired numbers, never to be reused, so that a peer built against one
    // fails to dispatch rather than misparse a newer payload:
    //   2: the unchunked form of procedure 4, a block's deltas in one
    //      unbounded record, which a block with enough distinct recipients
    //      grows past any single-record cap.
    //   3: scheddb_changed_addresses(BlockHeight, ContractAddressBatch),
    //      removed in e413dc9. Its payload also opens with a block height and
    //      a vector length, so a chunk sent on 3 would decode there as
    //      addresses.

    // Deliver one chunk of a committed block's balance deltas. With several db
    // shards the mempool completes the block once every registered shard has
    // sent its run's last chunk.

    fn scheddb_native_tokens_transferred_chunk(&self, arg0 : NativeTokensTransferredChunk) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct SchedDb_SchedDb1_Dispatcher<__Handler> {
  inner: Box<__Handler>,
}

impl<__Handler: SchedDb_SchedDb1> SchedDb_SchedDb1_Dispatcher<__Handler> {
  pub fn new(inner: Box<__Handler>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<__Handler: SchedDb_SchedDb1> ::thruput_rpc_api::RpcHandler for SchedDb_SchedDb1_Dispatcher<__Handler> {
    fn prog_vers(&self) -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x7ee79998, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::thruput_rpc_api::RpcError> {
        match proc {
            // scheddb_intro_shard
            1 => {
                let args: (ShardCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scheddb_intro_shard(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scheddb_native_tokens_transferred_chunk
            4 => {
                let args: (NativeTokensTransferredChunk, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scheddb_native_tokens_transferred_chunk(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            _ => Err(::thruput_rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SchedDb_SchedDb1_Client {
    handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>,
}

impl SchedDb_SchedDb1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x7ee79998, 1)
    }
    pub async fn scheddb_intro_shard(&self, arg0 : &(impl xdrs::EncodesAs<ShardCard> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 1, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scheddb_native_tokens_transferred_chunk(&self, arg0 : &(impl xdrs::EncodesAs<NativeTokensTransferredChunk> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 4, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>> for SchedDb_SchedDb1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


