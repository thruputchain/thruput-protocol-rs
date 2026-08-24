// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 7cc13390a06c0d4590a76fcc35ad15f6b499f050

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
    async fn discovery_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn discovery_update_quorums(&self, arg0 : KnownQuorumSets) -> Result<KnownQuorumSets, ::rpc_driver::ProgramError>;
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
impl ::rpc_driver::handler::RpcHandler for Discovery_Discovery1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x3eef0505, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // discovery_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // discovery_update_quorums
            10 => {
                let args: (KnownQuorumSets, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_update_quorums(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct Discovery_Discovery1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl Discovery_Discovery1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x3eef0505, 1)
    }
    pub async fn discovery_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn discovery_update_quorums(&self, arg0 : &KnownQuorumSets) -> Result<KnownQuorumSets, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_driver::RpcSender>> for Discovery_Discovery1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


