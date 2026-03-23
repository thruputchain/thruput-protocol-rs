// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 738ffcebf4d4aa33f5880797b1f889b551bb5a62

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



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CatchupBlock {
    pub consensus_value: ConsensusValue,
    pub signed_transactions: ParallelSignedTransactionBatch,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CatchupSpan {
    pub from_block_height: BlockHeight,
    // not including
    pub to_block_height: BlockHeight,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CatchupBlocksRequest {
    pub timestamp: TimePoint,
    pub spans: VecM<CatchupSpan>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedCatchupBlocksRequest {
    pub signer: ValidatorPublicInfo,
    pub signature: Signature,
    pub request: CatchupBlocksRequest,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CatchupBlocksResult {
    pub timestamp: TimePoint,
    pub blocks: VecM<CatchupBlock>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedCatchupBlocksResult {
    pub signer: ValidatorPublicInfo,
    pub signature: Signature,
    pub result: CatchupBlocksResult,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait Catchup_Catchup1: Send + Sync {    
    async fn catchup_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn catchup_solicit(&self, arg0 : SignedCatchupBlocksRequest) -> Result<SignedCatchupBlocksResult, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct Catchup_Catchup1_Dispatcher {
  inner: Box<dyn Catchup_Catchup1>,
}

impl Catchup_Catchup1_Dispatcher {
  pub fn new(inner: Box<dyn Catchup_Catchup1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for Catchup_Catchup1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321abcd, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // catchup_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.catchup_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // catchup_solicit
            10 => {
                let args: (SignedCatchupBlocksRequest, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.catchup_solicit(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct Catchup_Catchup1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl Catchup_Catchup1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321abcd, 1)
    }
    pub async fn catchup_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn catchup_solicit(&self, arg0 : &SignedCatchupBlocksRequest) -> Result<SignedCatchupBlocksResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//


