// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: bab641a16d398d48b16086772f51d186ac0522cb

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TxInfo {
    // The height of the block the tx was executed in.
    pub height: BlockHeight,
    pub events: TxEventLog,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct TxInfoOutcome(pub i32);

impl TxInfoOutcome {
    pub const TxInfoFound: Self = Self(0);
    pub const TxInfoNotFound: Self = Self(1);
}
impl From<i32> for TxInfoOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for TxInfoOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::TxInfoFound.0 => {
                write!(f, "TxInfoOutcome::TxInfoFound")
            },
            x if x == Self::TxInfoNotFound.0 => {
                write!(f, "TxInfoOutcome::TxInfoNotFound")
            },
            _ => {
                write!(f, "TxInfoOutcome({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
#[non_exhaustive]
pub enum TxInfoResult {
    // case TxInfoFound:
    //   TxInfo info;
    TxInfoFound { info: TxInfo } = TxInfoOutcome::TxInfoFound.0 as i32,
    // default:
    //   void;
    Default { outcome: i32 },
}

pub type IdentifiedEventBatch = VecM::<IdentifiedEvent>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IndexerBlockExplorerIntroOutcome(pub i32);

impl IndexerBlockExplorerIntroOutcome {
    pub const IntroSuccess: Self = Self(0);
    pub const IntroMismatchedProtocol: Self = Self(1);
}
impl From<i32> for IndexerBlockExplorerIntroOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for IndexerBlockExplorerIntroOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::IntroSuccess.0 => {
                write!(f, "IndexerBlockExplorerIntroOutcome::IntroSuccess")
            },
            x if x == Self::IntroMismatchedProtocol.0 => {
                write!(f, "IndexerBlockExplorerIntroOutcome::IntroMismatchedProtocol")
            },
            _ => {
                write!(f, "IndexerBlockExplorerIntroOutcome({})", self.0)
            }
        }
    }
}



#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait IndexerBlockExplorer_IndexerBlockExplorer1: Send + Sync {    
    async fn indexer_block_explorer_intro(&self, arg0 : Protocol) -> Result<IndexerBlockExplorerIntroOutcome, ::rpc_driver::ProgramError>;
    
    async fn get_transaction_info(&self, arg0 : SignedTransactionHash) -> Result<TxInfoResult, ::rpc_driver::ProgramError>;
    
    async fn get_contract_events(&self, arg0 : ContractAddress, arg1 : BlockHeight, arg2 : BlockHeight) -> Result<IdentifiedEventBatch, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct IndexerBlockExplorer_IndexerBlockExplorer1_Dispatcher {
  inner: Box<dyn IndexerBlockExplorer_IndexerBlockExplorer1>,
}

impl IndexerBlockExplorer_IndexerBlockExplorer1_Dispatcher {
  pub fn new(inner: Box<dyn IndexerBlockExplorer_IndexerBlockExplorer1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for IndexerBlockExplorer_IndexerBlockExplorer1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaaa, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // indexer_block_explorer_intro
            0 => {
                let args: (Protocol, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.indexer_block_explorer_intro(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // get_transaction_info
            1 => {
                let args: (SignedTransactionHash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.get_transaction_info(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // get_contract_events
            2 => {
                let args: (ContractAddress, BlockHeight, BlockHeight, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.get_contract_events(args.0, args.1, args.2).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct IndexerBlockExplorer_IndexerBlockExplorer1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl IndexerBlockExplorer_IndexerBlockExplorer1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaaa, 1)
    }
    pub async fn indexer_block_explorer_intro(&self, arg0 : &Protocol) -> Result<IndexerBlockExplorerIntroOutcome, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn get_transaction_info(&self, arg0 : &SignedTransactionHash) -> Result<TxInfoResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn get_contract_events(&self, arg0 : &ContractAddress, arg1 : &BlockHeight, arg2 : &BlockHeight) -> Result<IdentifiedEventBatch, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1, &arg2)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}


pub type TxEventLogBatch = VecM::<TxEventLog>;


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait IndexerEvent_IndexerEvent1: Send + Sync {    
    async fn indexer_event_process_events(&self, arg0 : BlockHeight, arg1 : TxEventLogBatch) -> Result<(), ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct IndexerEvent_IndexerEvent1_Dispatcher {
  inner: Box<dyn IndexerEvent_IndexerEvent1>,
}

impl IndexerEvent_IndexerEvent1_Dispatcher {
  pub fn new(inner: Box<dyn IndexerEvent_IndexerEvent1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for IndexerEvent_IndexerEvent1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaab, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // indexer_event_process_events
            0 => {
                let args: (BlockHeight, TxEventLogBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.indexer_event_process_events(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct IndexerEvent_IndexerEvent1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl IndexerEvent_IndexerEvent1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaab, 1)
    }
    pub async fn indexer_event_process_events(&self, arg0 : &BlockHeight, arg1 : &TxEventLogBatch) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexerCard {
    pub protocol: Protocol,
    pub address_for_event: ServerAddress,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait EventIndexer_EventIndexer1: Send + Sync {    
    async fn event_indexer_intro(&self, arg0 : IndexerCard) -> Result<(), ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct EventIndexer_EventIndexer1_Dispatcher {
  inner: Box<dyn EventIndexer_EventIndexer1>,
}

impl EventIndexer_EventIndexer1_Dispatcher {
  pub fn new(inner: Box<dyn EventIndexer_EventIndexer1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for EventIndexer_EventIndexer1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaac, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // event_indexer_intro
            0 => {
                let args: (IndexerCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.event_indexer_intro(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct EventIndexer_EventIndexer1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl EventIndexer_EventIndexer1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x4321aaac, 1)
    }
    pub async fn event_indexer_intro(&self, arg0 : &IndexerCard) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//

impl TxInfoResult {
    pub fn outcome(&self) -> TxInfoOutcome {
        match self {
            TxInfoResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


