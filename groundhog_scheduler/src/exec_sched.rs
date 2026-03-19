// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0b35b4bc56dc32f40c6de155f35b76f18a1da5f2

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



// clang-format on

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
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
    Default { result: i32 },
}

pub type ProposedTransactionBatch = VecM::<ProposedTransaction>;

pub type ExecProposeBatchResult = VecM::<ExecProposeResult>;

pub type ExecValidateBatchResult = VecM::<ExecResult>;


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait ExecutorSched_ExecutorSched1: Send + Sync {    
    async fn executorsched_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn executorsched_ready(&self) -> Result<ExecResult, ::rpc_driver::ProgramError>;
    
    async fn executorsched_propose_transaction(&self, arg0 : BlockID, arg1 : Transaction) -> Result<ExecProposeResult, ::rpc_driver::ProgramError>;
    
    async fn executorsched_validate_transaction(&self, arg0 : BlockID, arg1 : ProposedTransaction) -> Result<ExecResult, ::rpc_driver::ProgramError>;
    
    async fn executorsched_propose_batch(&self, arg0 : BlockID, arg1 : TransactionBatch) -> Result<ExecProposeBatchResult, ::rpc_driver::ProgramError>;
    
    async fn executorsched_validate_batch(&self, arg0 : BlockID, arg1 : ProposedTransactionBatch) -> Result<ExecValidateBatchResult, ::rpc_driver::ProgramError>;
        // Executor can be in one of three states: None, Proposing, Validating
    // open_xyz_block() methods set the executor state, or return an error
    // if executor has already opened a block.
    // finish_block() blocks until all running executor threads are finished,
    // and stops the executor from accepting new transactions.

    async fn executorsched_open_proposal_block(&self, arg0 : BlockID) -> Result<ExecResult, ::rpc_driver::ProgramError>;
    
    async fn executorsched_open_validation_block(&self, arg0 : BlockID) -> Result<ExecResult, ::rpc_driver::ProgramError>;
    
    async fn executorsched_finish_block(&self, arg0 : BlockID) -> Result<ExecResult, ::rpc_driver::ProgramError>;
        // Scheduler should call this to shutdown the executor.
    // Shutdown will not occur until all pending transactions complete.
    // No new transactions or blocks can be opened after shutdown.

    async fn executorsched_shutdown(&self) -> Result<(), ::rpc_driver::ProgramError>;
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
impl ::rpc_driver::handler::RpcHandler for ExecutorSched_ExecutorSched1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x7ee79999, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // executorsched_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_ready
            1 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_ready().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_propose_transaction
            10 => {
                let args: (BlockID, Transaction, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_propose_transaction(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_validate_transaction
            20 => {
                let args: (BlockID, ProposedTransaction, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_validate_transaction(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_propose_batch
            11 => {
                let args: (BlockID, TransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_propose_batch(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_validate_batch
            21 => {
                let args: (BlockID, ProposedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_validate_batch(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_open_proposal_block
            30 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_open_proposal_block(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_open_validation_block
            40 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_open_validation_block(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_finish_block
            50 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_finish_block(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // executorsched_shutdown
            60 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.executorsched_shutdown().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct ExecutorSched_ExecutorSched1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl ExecutorSched_ExecutorSched1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x7ee79999, 1)
    }
    pub async fn executorsched_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_ready(&self) -> Result<ExecResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_propose_transaction(&self, arg0 : &BlockID, arg1 : &Transaction) -> Result<ExecProposeResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_validate_transaction(&self, arg0 : &BlockID, arg1 : &ProposedTransaction) -> Result<ExecResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 20, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_propose_batch(&self, arg0 : &BlockID, arg1 : &TransactionBatch) -> Result<ExecProposeBatchResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 11, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_validate_batch(&self, arg0 : &BlockID, arg1 : &ProposedTransactionBatch) -> Result<ExecValidateBatchResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 21, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_open_proposal_block(&self, arg0 : &BlockID) -> Result<ExecResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 30, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_open_validation_block(&self, arg0 : &BlockID) -> Result<ExecResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 40, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_finish_block(&self, arg0 : &BlockID) -> Result<ExecResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 50, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn executorsched_shutdown(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 60, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//

impl ExecProposeResult {
    pub fn result(&self) -> i32 {
        match self {
            ExecProposeResult::Default{ result } => *result ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


