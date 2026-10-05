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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HogsTransferred {
    // The height of the block in which these transfers occurred. The scheduler
    // uses this height to update average hog ages.
    pub block: BlockHeight,
    pub deltas: VecM<Delta>,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SchedDb_SchedDb1: Send + Sync {        // [scheddb_intro_shard] is called by a db shard to introduce itself to the
    // system.

    async fn scheddb_intro_shard(&self, arg0 : ShardCard) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scheddb_hogs_transferred(&self, arg0 : HogsTransferred) -> Result<(), ::rpc_api::ProgramError>;
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
            // scheddb_hogs_transferred
            2 => {
                let args: (HogsTransferred, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scheddb_hogs_transferred(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
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
    pub async fn scheddb_hogs_transferred(&self, arg0 : &HogsTransferred) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, call).await?;
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


