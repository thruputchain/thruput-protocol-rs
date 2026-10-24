// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: c385cdc8b9f636baa910ab0814dbbb7f0ff55057

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



// clang-format on

// Uniquely describes an executor instance.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ExecCard {
    pub protocol: ProtocolCommit,
    pub upid: UPID,
    pub address: ServerAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IntroExecResult {
    // A unique 16-bit identifier assigned to the executor by the system.
    pub exec_id: u64,
    // The database shards.
    pub shards: VecM<ShardCard>,
    // The event server
    pub event: EventCard,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SchedExec_SchedExec1: Send + Sync {        // [schedexec_intro_exec] is called by an executor to introduce itself to
    // the system.

    async fn schedexec_intro_exec(&self, arg0 : ExecCard) -> Result<IntroExecResult, ::thruput_rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct SchedExec_SchedExec1_Dispatcher {
  inner: Box<dyn SchedExec_SchedExec1>,
}

impl SchedExec_SchedExec1_Dispatcher {
  pub fn new(inner: Box<dyn SchedExec_SchedExec1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::thruput_rpc_api::RpcHandler for SchedExec_SchedExec1_Dispatcher {
    fn prog_vers(&self) -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x7ee79997, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::thruput_rpc_api::RpcError> {
        match proc {
            // schedexec_intro_exec
            1 => {
                let args: (ExecCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedexec_intro_exec(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            _ => Err(::thruput_rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SchedExec_SchedExec1_Client {
    handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>,
}

impl SchedExec_SchedExec1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x7ee79997, 1)
    }
    pub async fn schedexec_intro_exec(&self, arg0 : &(impl xdrs::EncodesAs<ExecCard> + ?Sized)) -> Result<IntroExecResult, ::thruput_rpc_api::RpcError> {
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
}

impl From<::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>> for SchedExec_SchedExec1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


