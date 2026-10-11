// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 4eba524354bda9d7db1efa2845da43ce40dd1ab9

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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct QuorumSetSnapshot {
    pub timestamp: TimePoint,
    pub quorum_set: SCPQuorumSet,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedQuorumSetSnapshot {
    pub signer: ValidatorPublicInfo,
    pub signature: Signature,
    pub quorum_set_snapshot: QuorumSetSnapshot,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorInfoSnapshot {
    pub timestamp: TimePoint,
    pub validator: ValidatorPublicInfo,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedValidatorInfoSnapshot {
    pub signer: ValidatorPublicInfo,
    pub signature: Signature,
    pub validator_snapshot: ValidatorInfoSnapshot,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct KnownQuorumSets {
    pub quorum_set_snapshots: VecM<SignedQuorumSetSnapshot>,
    pub validator_snapshots: VecM<SignedValidatorInfoSnapshot>,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait Discovery_Discovery1: Send + Sync {    
    async fn discovery_null(&self) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn discovery_update_quorums(&self, arg0 : KnownQuorumSets) -> Result<KnownQuorumSets, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct Discovery_Discovery1_Dispatcher {
  inner: Box<dyn Discovery_Discovery1>,
}

impl Discovery_Discovery1_Dispatcher {
  pub fn new(inner: Box<dyn Discovery_Discovery1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for Discovery_Discovery1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x3eef0505, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // discovery_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // discovery_update_quorums
            10 => {
                let args: (KnownQuorumSets, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_update_quorums(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct Discovery_Discovery1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl Discovery_Discovery1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x3eef0505, 1)
    }
    pub async fn discovery_null(&self) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn discovery_update_quorums(&self, arg0 : &(impl xdrs::EncodesAs<KnownQuorumSets> + ?Sized)) -> Result<KnownQuorumSets, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for Discovery_Discovery1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


