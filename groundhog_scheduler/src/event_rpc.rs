// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 658b4a40c8451ea28d2bddb17f52ee1d5c485c65

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




#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait EventExecutor_EventExecutor1: Send + Sync {    
    async fn eventexecutor_null(&self) -> Result<(), ::rpc_api::ProgramError>;
        // Called when executor opens a block - increments executor handle count

    async fn eventexecutor_open_block(&self, arg0 : BlockID) -> Result<RPCError, ::rpc_api::ProgramError>;
        // Called when executor closes its handle to a block - decrements count

    async fn eventexecutor_close_block(&self, arg0 : BlockID) -> Result<RPCError, ::rpc_api::ProgramError>;
        // Best-effort report that a transaction failed nondeterministically
    // during 2-phase commit and was excluded from the proposal. The event
    // server places this in a bounded best-effort mailbox for the indexer to
    // pull; it touches no per-block state (the tx will never commit, so it has
    // no place in the commit pipeline). The executor sends fire-and-forget and
    // ignores the result.

    async fn eventexecutor_report_tx_failure(&self, arg0 : TransactionHash, arg1 : TxFailureDiagnostic) -> Result<RPCError, ::rpc_api::ProgramError>;
        // A transaction should not call this more than once
    // We don't technically need the transaction ID here;
    // however, this will help later with tracing RPC calls through the system,
    // and helps in testing.
    //
    // The ProposedTransaction carries the full transaction body (signed_tx +
    // nondeterministic data) so the event server can forward bodies to the
    // indexer alongside the event log.  In proposal mode the executor assembles
    // it from the SignedTransaction it executed plus the nondeterministic data
    // it produced; in validation mode it is the ProposedTransaction it was given.

    async fn eventexecutor_emit_event(&self, arg0 : BlockID, arg1 : TransactionID, arg2 : TxEventLog, arg3 : ProposedTransaction) -> Result<RPCError, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct EventExecutor_EventExecutor1_Dispatcher {
  inner: Box<dyn EventExecutor_EventExecutor1>,
}

impl EventExecutor_EventExecutor1_Dispatcher {
  pub fn new(inner: Box<dyn EventExecutor_EventExecutor1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for EventExecutor_EventExecutor1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa81224, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // eventexecutor_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventexecutor_null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // eventexecutor_open_block
            31 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventexecutor_open_block(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // eventexecutor_close_block
            32 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventexecutor_close_block(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // eventexecutor_report_tx_failure
            33 => {
                let args: (TransactionHash, TxFailureDiagnostic, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventexecutor_report_tx_failure(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // eventexecutor_emit_event
            30 => {
                let args: (BlockID, TransactionID, TxEventLog, ProposedTransaction, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventexecutor_emit_event(args.0, args.1, args.2, args.3).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct EventExecutor_EventExecutor1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl EventExecutor_EventExecutor1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa81224, 1)
    }
    pub async fn eventexecutor_null(&self) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn eventexecutor_open_block(&self, arg0 : &BlockID) -> Result<RPCError, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 31, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn eventexecutor_close_block(&self, arg0 : &BlockID) -> Result<RPCError, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 32, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn eventexecutor_report_tx_failure(&self, arg0 : &TransactionHash, arg1 : &TxFailureDiagnostic) -> Result<RPCError, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 33, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn eventexecutor_emit_event(&self, arg0 : &BlockID, arg1 : &TransactionID, arg2 : &TxEventLog, arg3 : &ProposedTransaction) -> Result<RPCError, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0, &arg1, &arg2, &arg3)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 30, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for EventExecutor_EventExecutor1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum EventBlockResult {
    // case kRPCNone:
    //   SHA256Hash root_hash;
    kRPCNone { root_hash: SHA256Hash } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { error: i32 },
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait EventSched_EventSched1: Send + Sync {    
    async fn eventsched_null(&self) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn eventsched_open_block(&self, arg0 : BlockID) -> Result<RPCError, ::rpc_api::ProgramError>;
    
    async fn eventsched_close_block(&self, arg0 : BlockID, arg1 : BlockHeight) -> Result<EventBlockResult, ::rpc_api::ProgramError>;
        // Called after finalization. Durably publishes the event/indexer record;
    // the record becomes pullable as soon as this call succeeds, even if the
    // local HogDB commit running alongside it has not completed yet.

    async fn eventsched_commit_block(&self, arg0 : BlockID, arg1 : ConsensusValue) -> Result<RPCError, ::rpc_api::ProgramError>;
    
    async fn eventsched_abandon_block(&self, arg0 : BlockID) -> Result<RPCError, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct EventSched_EventSched1_Dispatcher {
  inner: Box<dyn EventSched_EventSched1>,
}

impl EventSched_EventSched1_Dispatcher {
  pub fn new(inner: Box<dyn EventSched_EventSched1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for EventSched_EventSched1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa81226, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // eventsched_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventsched_null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // eventsched_open_block
            10 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventsched_open_block(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // eventsched_close_block
            30 => {
                let args: (BlockID, BlockHeight, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventsched_close_block(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // eventsched_commit_block
            31 => {
                let args: (BlockID, ConsensusValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventsched_commit_block(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // eventsched_abandon_block
            40 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.eventsched_abandon_block(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct EventSched_EventSched1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl EventSched_EventSched1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x8aa81226, 1)
    }
    pub async fn eventsched_null(&self) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn eventsched_open_block(&self, arg0 : &BlockID) -> Result<RPCError, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn eventsched_close_block(&self, arg0 : &BlockID, arg1 : &BlockHeight) -> Result<EventBlockResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 30, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn eventsched_commit_block(&self, arg0 : &BlockID, arg1 : &ConsensusValue) -> Result<RPCError, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0, &arg1)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 31, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn eventsched_abandon_block(&self, arg0 : &BlockID) -> Result<RPCError, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 40, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for EventSched_EventSched1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl EventBlockResult {
    pub fn error(&self) -> RPCError {
        match self {
            EventBlockResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


