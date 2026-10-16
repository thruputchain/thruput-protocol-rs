// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: ec35b7e948f24afe96a948ea1637c12f260be1f5

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
pub struct BristleconeQuorumSetSnapshot {
    pub timestamp: TimePoint,
    pub quorum_set: BristleconeQuorumSet,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeSignedQuorumSetSnapshot {
    pub signer: ValidatorPublicInfo,
    pub signature: Signature,
    pub quorum_set_snapshot: BristleconeQuorumSetSnapshot,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeValidatorInfoSnapshot {
    pub timestamp: TimePoint,
    pub validator: ValidatorPublicInfo,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeSignedValidatorInfoSnapshot {
    pub signer: ValidatorPublicInfo,
    pub signature: Signature,
    pub validator_snapshot: BristleconeValidatorInfoSnapshot,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeKnownQuorumSets {
    pub quorum_set_snapshots: VecM<BristleconeSignedQuorumSetSnapshot>,
    pub validator_snapshots: VecM<BristleconeSignedValidatorInfoSnapshot>,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait Discovery_Discovery2: Send + Sync {    
    async fn discovery_null(&self) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn discovery_update_quorums(&self, arg0 : BristleconeKnownQuorumSets) -> Result<BristleconeKnownQuorumSets, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct Discovery_Discovery2_Dispatcher {
  inner: Box<dyn Discovery_Discovery2>,
}

impl Discovery_Discovery2_Dispatcher {
  pub fn new(inner: Box<dyn Discovery_Discovery2>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for Discovery_Discovery2_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x3eef0707, 2)
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
                let args: (BristleconeKnownQuorumSets, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_update_quorums(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct Discovery_Discovery2_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl Discovery_Discovery2_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x3eef0707, 2)
    }
    pub async fn discovery_null(&self) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn discovery_update_quorums(&self, arg0 : &(impl xdrs::EncodesAs<BristleconeKnownQuorumSets> + ?Sized)) -> Result<BristleconeKnownQuorumSets, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for Discovery_Discovery2_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


