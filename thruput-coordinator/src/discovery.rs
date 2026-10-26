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


/// Each handler replies with anything that encodes like the reply type.
/// Implementations can be plain `async fn`s returning a concrete reply type
/// under `#[allow(refining_impl_trait)]`.
#[allow(non_camel_case_types)]
pub trait Discovery_Discovery1: Send + Sync {    
    fn discovery_null(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn discovery_update_quorums(&self, arg0 : KnownQuorumSets) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<KnownQuorumSets>, ::thruput_rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct Discovery_Discovery1_Dispatcher<__Handler> {
  inner: Box<__Handler>,
}

impl<__Handler: Discovery_Discovery1> Discovery_Discovery1_Dispatcher<__Handler> {
  pub fn new(inner: Box<__Handler>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<__Handler: Discovery_Discovery1> ::thruput_rpc_api::RpcHandler for Discovery_Discovery1_Dispatcher<__Handler> {
    fn prog_vers(&self) -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x3eef0505, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::thruput_rpc_api::RpcError> {
        match proc {
            // discovery_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_null().await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // discovery_update_quorums
            10 => {
                let args: (KnownQuorumSets, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_update_quorums(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            _ => Err(::thruput_rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct Discovery_Discovery1_Client {
    handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>,
}

impl Discovery_Discovery1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x3eef0505, 1)
    }
    pub async fn discovery_null(&self) -> Result<(), ::thruput_rpc_api::RpcError> {
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
    pub async fn discovery_update_quorums(&self, arg0 : &(impl xdrs::EncodesAs<KnownQuorumSets> + ?Sized)) -> Result<KnownQuorumSets, ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 10, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>> for Discovery_Discovery1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


