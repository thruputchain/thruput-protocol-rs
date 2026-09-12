// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: ecaf232903ce409121e6a42607860a6a0db767e8

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

/// Input: a batch of items to read, identified by (address, key).
pub type AddressAndKeyBatch = VecM::<AddressAndKey>;

/// Per-item result for DbGateway1::dbgateway_get_items.
///
/// The success case carries the WireStorageItem; kRPCNotFound is the
/// legitimate outcome for an item that does not exist in the committed
/// snapshot (e.g., a balance item for an address that has never received
/// hogs); other errors indicate server-side failures.  Defined locally
/// rather than reusing `ExpectedStorageItem` from executor_to_db_rpc so
/// this program has no cross-dependency on the executor-side RPC.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum DbGatewayItemResult {
    // case kRPCNone:
    //   WireStorageItem item;
    kRPCNone { item: WireStorageItem } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { error: i32 },
}

pub type DbGatewayItemBatch = VecM::<DbGatewayItemResult>;

/// Batch-read payload: one BlockHeight applies to every result in the
/// batch because all reads come from a single committed snapshot.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DbGatewayPayload {
    pub block_height: BlockHeight,
    pub items: DbGatewayItemBatch,
}

/// Outer result: the RPC itself succeeded or failed as a whole.
/// Per-item misses live inside the `items` batch as kRPCNotFound.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum DbGatewayResult {
    // case kRPCNone:
    //   DbGatewayPayload payload;
    kRPCNone { payload: DbGatewayPayload } = RPCError::kRPCNone.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { error: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DbGatewayIntroOutcome(pub i32);

impl DbGatewayIntroOutcome {
    pub const DbGatewayIntroSuccess: Self = Self(0);
    pub const DbGatewayIntroMismatchedProtocol: Self = Self(1);
}
impl From<i32> for DbGatewayIntroOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for DbGatewayIntroOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::DbGatewayIntroSuccess.0 => {
                write!(f, "DbGatewayIntroOutcome::DbGatewayIntroSuccess")
            },
            x if x == Self::DbGatewayIntroMismatchedProtocol.0 => {
                write!(f, "DbGatewayIntroOutcome::DbGatewayIntroMismatchedProtocol")
            },
            _ => {
                write!(f, "DbGatewayIntroOutcome({})", self.0)
            }
        }
    }
}



#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait DbGateway_DbGateway1: Send + Sync {    
    async fn dbgateway_intro(&self, arg0 : ProtocolCommit) -> Result<DbGatewayIntroOutcome, ::rpc_api::ProgramError>;
    
    async fn dbgateway_get_items(&self, arg0 : AddressAndKeyBatch) -> Result<DbGatewayResult, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct DbGateway_DbGateway1_Dispatcher {
  inner: Box<dyn DbGateway_DbGateway1>,
}

impl DbGateway_DbGateway1_Dispatcher {
  pub fn new(inner: Box<dyn DbGateway_DbGateway1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for DbGateway_DbGateway1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x44424757, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // dbgateway_intro
            0 => {
                let args: (ProtocolCommit, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbgateway_intro(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // dbgateway_get_items
            1 => {
                let args: (AddressAndKeyBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbgateway_get_items(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct DbGateway_DbGateway1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl DbGateway_DbGateway1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x44424757, 1)
    }
    pub async fn dbgateway_intro(&self, arg0 : &ProtocolCommit) -> Result<DbGatewayIntroOutcome, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn dbgateway_get_items(&self, arg0 : &AddressAndKeyBatch) -> Result<DbGatewayResult, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for DbGateway_DbGateway1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl DbGatewayItemResult {
    pub fn error(&self) -> RPCError {
        match self {
            DbGatewayItemResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl DbGatewayResult {
    pub fn error(&self) -> RPCError {
        match self {
            DbGatewayResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


