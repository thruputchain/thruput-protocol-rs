// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 58301365769c00d43c91f44a53e2148f54fdfa6a

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

// missing transactions (v2)

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct MissingTxs {
    pub tx_hashes: VecM<TransactionHash>,
    pub consensus_value: ConsensusValue,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedMissingTxs {
    pub signer: ValidatorID,
    pub signature: Signature,
    pub missing_txs: MissingTxs,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RepairedTxs {
    pub txs: VecM<SignedTransaction>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedRepairedTxs {
    pub signature: Signature,
    pub repaired_txs: RepairedTxs,
}

// missing proposal structures (v2)

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct MissingProposal {
    pub consensus_value: ConsensusValue,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedMissingProposal {
    pub signer: ValidatorID,
    pub signature: Signature,
    pub missing_proposal: MissingProposal,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RepairedProposal {
    pub proposal: Option<Proposal>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedRepairedProposal {
    pub signature: Signature,
    pub repaired_proposal: RepairedProposal,
}

// missing slot structures (v2)

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct MissingSlot {
    pub slot: u64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedMissingSlot {
    pub signer: ValidatorID,
    pub signature: Signature,
    pub missing_slot: MissingSlot,
}

pub type SlotMessage = BytesM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RepairedSlot {
    pub slot_messages: VecM<SlotMessage>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedRepairedSlot {
    pub signature: Signature,
    pub repaired_slot: RepairedSlot,
}

// missing block structures (v2)

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct MissingBlocks {
    pub spans: VecM<CatchupSpan>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedMissingBlocks {
    pub signer: ValidatorID,
    pub signature: Signature,
    pub missing_blocks: MissingBlocks,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct RepairedBlocks {
    pub blocks: VecM<CatchupBlock>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedRepairedBlocks {
    pub signature: Signature,
    pub repaired_blocks: RepairedBlocks,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait Repair_Repair2: Send + Sync {    
    async fn repair_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn repair_missing_txs(&self, arg0 : SignedMissingTxs) -> Result<RepairedTxs, ::rpc_driver::ProgramError>;
    
    async fn repair_missing_proposal(&self, arg0 : SignedMissingProposal) -> Result<RepairedProposal, ::rpc_driver::ProgramError>;
    
    async fn repair_missing_slot(&self, arg0 : SignedMissingSlot) -> Result<RepairedSlot, ::rpc_driver::ProgramError>;
    
    async fn repair_missing_blocks(&self, arg0 : SignedMissingBlocks) -> Result<RepairedBlocks, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct Repair_Repair2_Dispatcher {
  inner: Box<dyn Repair_Repair2>,
}

impl Repair_Repair2_Dispatcher {
  pub fn new(inner: Box<dyn Repair_Repair2>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for Repair_Repair2_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x2233cccc, 2)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // repair_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.repair_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // repair_missing_txs
            10 => {
                let args: (SignedMissingTxs, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.repair_missing_txs(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // repair_missing_proposal
            20 => {
                let args: (SignedMissingProposal, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.repair_missing_proposal(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // repair_missing_slot
            30 => {
                let args: (SignedMissingSlot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.repair_missing_slot(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // repair_missing_blocks
            40 => {
                let args: (SignedMissingBlocks, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.repair_missing_blocks(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct Repair_Repair2_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl Repair_Repair2_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x2233cccc, 2)
    }
    pub async fn repair_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn repair_missing_txs(&self, arg0 : &SignedMissingTxs) -> Result<RepairedTxs, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn repair_missing_proposal(&self, arg0 : &SignedMissingProposal) -> Result<RepairedProposal, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 20, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn repair_missing_slot(&self, arg0 : &SignedMissingSlot) -> Result<RepairedSlot, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 30, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn repair_missing_blocks(&self, arg0 : &SignedMissingBlocks) -> Result<RepairedBlocks, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 40, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_driver::RpcSender>> for Repair_Repair2_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


