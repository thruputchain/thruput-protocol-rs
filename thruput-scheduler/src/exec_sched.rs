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



// clang-format on

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExecProposeResult {
    // case ExecResultOK:
    // case ExecResultTxDeterministicError:
    // case ExecResultTxRootFrameReverted:
    //   NondeterministicProposalData proposal_data;
    ExecResultOK { proposal_data: NondeterministicProposalData } = ExecResult::ExecResultOK.0 as i32,
    ExecResultTxDeterministicError { proposal_data: NondeterministicProposalData } = ExecResult::ExecResultTxDeterministicError.0 as i32,
    ExecResultTxRootFrameReverted { proposal_data: NondeterministicProposalData } = ExecResult::ExecResultTxRootFrameReverted.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { result: i32 },
}

pub type ExecProposeBatchResult = VecM::<ExecProposeResult>;

pub type ExecValidateBatchResult = VecM::<ExecResult>;


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait ExecutorSched_ExecutorSched1: Send + Sync {    
    async fn executorsched_null(&self) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn executorsched_ready(&self) -> Result<ExecResult, ::rpc_api::ProgramError>;
    
    async fn executorsched_propose_transaction(&self, arg0 : BlockID, arg1 : SignedTransaction) -> Result<ExecProposeResult, ::rpc_api::ProgramError>;
    
    async fn executorsched_validate_transaction(&self, arg0 : BlockID, arg1 : ProposedTransaction) -> Result<ExecResult, ::rpc_api::ProgramError>;
    
    async fn executorsched_propose_batch(&self, arg0 : BlockID, arg1 : SignedTransactionBatch) -> Result<ExecProposeBatchResult, ::rpc_api::ProgramError>;
    
    async fn executorsched_validate_batch(&self, arg0 : BlockID, arg1 : ProposedTransactionBatch) -> Result<ExecValidateBatchResult, ::rpc_api::ProgramError>;
        // Executor can be in one of three states: None, Proposing, Validating
    // open_xyz_block() methods set the executor state, or return an error
    // if executor has already opened a block.
    // finish_block() blocks until all running executor threads are finished,
    // and stops the executor from accepting new transactions.
    //
    // Both open_xyz_block() methods take the same BlockExecutionContext, so
    // the executor sees an identical view of the block whether it is
    // proposing or validating, and everything a contract can observe about
    // the current block (its height, and the GetBlockTimepoint syscall)
    // comes from one place instead of being inferred from the parent tip.
    //
    // The scheduler builds the context from whichever side it is on:
    //   - proposing:  from the coordinator's ProposalContext, passed to
    //                 schedulercoord_propose_block.
    //   - validating: from the ProposalHeader of the proposal under
    //                 validation.
    //
    // A validator MUST take these values from the proposal it is validating
    // and never regenerate them locally: the proposer's values are the ones
    // its execution committed to, so any divergence here diverges execution
    // and splits consensus.

    async fn executorsched_open_proposal_block(&self, arg0 : BlockID, arg1 : BlockExecutionContext) -> Result<ExecResult, ::rpc_api::ProgramError>;
    
    async fn executorsched_open_validation_block(&self, arg0 : BlockID, arg1 : BlockExecutionContext) -> Result<ExecResult, ::rpc_api::ProgramError>;
    
    async fn executorsched_finish_block(&self, arg0 : BlockID) -> Result<ExecResult, ::rpc_api::ProgramError>;
        // Scheduler should call this to shutdown the executor.
    // Shutdown will not occur until all pending transactions complete.
    // No new transactions or blocks can be opened after shutdown.

    async fn executorsched_shutdown(&self) -> Result<(), ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct ExecutorSched_ExecutorSched1_Dispatcher {
  inner: Box<dyn ExecutorSched_ExecutorSched1>,
}

impl ExecutorSched_ExecutorSched1_Dispatcher {
  pub fn new(inner: Box<dyn ExecutorSched_ExecutorSched1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for ExecutorSched_ExecutorSched1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x7ee79999, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // executorsched_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_ready
            1 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_ready().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_propose_transaction
            10 => {
                let args: (BlockID, SignedTransaction, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_propose_transaction(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_validate_transaction
            20 => {
                let args: (BlockID, ProposedTransaction, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_validate_transaction(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_propose_batch
            11 => {
                let args: (BlockID, SignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_propose_batch(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_validate_batch
            21 => {
                let args: (BlockID, ProposedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_validate_batch(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_open_proposal_block
            30 => {
                let args: (BlockID, BlockExecutionContext, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_open_proposal_block(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_open_validation_block
            40 => {
                let args: (BlockID, BlockExecutionContext, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_open_validation_block(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_finish_block
            50 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_finish_block(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // executorsched_shutdown
            60 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_shutdown().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct ExecutorSched_ExecutorSched1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl ExecutorSched_ExecutorSched1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x7ee79999, 1)
    }
    pub async fn executorsched_null(&self) -> Result<(), ::rpc_api::RpcError> {
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
    pub async fn executorsched_ready(&self) -> Result<ExecResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn executorsched_propose_transaction(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SignedTransaction> + ?Sized)) -> Result<ExecProposeResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn executorsched_validate_transaction(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ProposedTransaction> + ?Sized)) -> Result<ExecResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 20, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn executorsched_propose_batch(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SignedTransactionBatch> + ?Sized)) -> Result<ExecProposeBatchResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 11, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn executorsched_validate_batch(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ProposedTransactionBatch> + ?Sized)) -> Result<ExecValidateBatchResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 21, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn executorsched_open_proposal_block(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<BlockExecutionContext> + ?Sized)) -> Result<ExecResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 30, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn executorsched_open_validation_block(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<BlockExecutionContext> + ?Sized)) -> Result<ExecResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 40, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn executorsched_finish_block(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized)) -> Result<ExecResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 50, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn executorsched_shutdown(&self) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 60, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for ExecutorSched_ExecutorSched1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl ExecProposeResult {
    pub fn result(&self) -> ExecResult {
        match self {
            ExecProposeResult::Default{ result } => (*result).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


