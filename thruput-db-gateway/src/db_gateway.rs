// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: e0e14dde6a5cd5db1bcaab5a5c5c8006735576c1

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

/// Input: a batch of items to read, identified by (address, key).
pub type AddressAndKeyBatch = VecM::<AddressAndKey>;

/// Per-item result for DbGateway1::dbgateway_get_items.
///
/// The success case carries the WireStorageItem; kRPCNotFound is the
/// legitimate outcome for an item that does not exist in the committed
/// snapshot (e.g., a balance item for an address that has never received
/// native tokens); other errors indicate server-side failures.  Defined locally
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

/// Network-supplied execution and storage rules read at one committed tip.
/// `tip.network_config` preserves the parameters explicitly stated by the chain.
/// `effective_config` states every parameter understood by the serving runtime,
/// resolving omitted values through the same rules used by execution. It is a
/// resolved reading, not a replacement for the hashed network configuration.
/// The rules apply to execution following `tip`; they are not a promise about
/// the block in which a subsequently submitted transaction will be included.
/// Storage prices are integer native-token base units (10^7 per token).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DbGatewayConfig {
    pub tip: DbTip,
    pub effective_config: NetworkConfig,
    pub storage_cost_model: StorageCostModelXdr,
}

/// No committed tip: kRPCNotFound. Unreadable rules: kRPCUnrecoverable.
/// A missing reading must never be represented as zero prices or empty rules.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum DbGatewayConfigResult {
    // case kRPCNone:
    //   DbGatewayConfig config;
    kRPCNone { config: DbGatewayConfig } = RPCError::kRPCNone.0 as i32,
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
pub trait DbGateway_DbGateway1: Send + Sync {    
    fn dbgateway_intro(&self, arg0 : ProtocolCommit) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbGatewayIntroOutcome>, ::rpc_api::ProgramError>> + Send;
    
    fn dbgateway_get_items(&self, arg0 : AddressAndKeyBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbGatewayResult>, ::rpc_api::ProgramError>> + Send;
    
    fn dbgateway_get_config(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbGatewayConfigResult>, ::rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct DbGateway_DbGateway1_Dispatcher<T> {
  inner: Box<T>,
}

impl<T: DbGateway_DbGateway1> DbGateway_DbGateway1_Dispatcher<T> {
  pub fn new(inner: Box<T>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<T: DbGateway_DbGateway1> ::rpc_api::RpcHandler for DbGateway_DbGateway1_Dispatcher<T> {
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
            // dbgateway_get_config
            2 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.dbgateway_get_config().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
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
    pub async fn dbgateway_intro(&self, arg0 : &(impl xdrs::EncodesAs<ProtocolCommit> + ?Sized)) -> Result<DbGatewayIntroOutcome, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn dbgateway_get_items(&self, arg0 : &(impl xdrs::EncodesAs<AddressAndKeyBatch> + ?Sized)) -> Result<DbGatewayResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn dbgateway_get_config(&self) -> Result<DbGatewayConfigResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, call).await?;
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

impl DbGatewayConfigResult {
    pub fn error(&self) -> RPCError {
        match self {
            DbGatewayConfigResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


