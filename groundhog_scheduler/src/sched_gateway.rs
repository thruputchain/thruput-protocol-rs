// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 14959ad52a564640d44d7079849fb9f146d330c0

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
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedGatewayIntroOutcome(pub i32);

impl SchedGatewayIntroOutcome {
    pub const SchedGatewayIntroSuccess: Self = Self(0);
    pub const SchedGatewayIntroMismatchedProtocol: Self = Self(1);
}
impl From<i32> for SchedGatewayIntroOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedGatewayIntroOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedGatewayIntroSuccess.0 => {
                write!(f, "SchedGatewayIntroOutcome::SchedGatewayIntroSuccess")
            },
            x if x == Self::SchedGatewayIntroMismatchedProtocol.0 => {
                write!(f, "SchedGatewayIntroOutcome::SchedGatewayIntroMismatchedProtocol")
            },
            _ => {
                write!(f, "SchedGatewayIntroOutcome({})", self.0)
            }
        }
    }
}


// The admission result for one exact SignedTransaction encoding in one
// gateway submission request. A rejection must not be cached or reused by
// TransactionHash: signatures are not part of that hash.
//
// A successful RPC returns exactly one outcome per decoded transaction, in
// submission order. A malformed request or cardinality mismatch is an RPC or
// protocol error; consumers must not infer partial acceptance from it.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedGatewaySubmitOutcome(pub i32);

impl SchedGatewaySubmitOutcome {
    // The transaction was admitted, or the same transaction body was already
    // pending.
    pub const SchedGatewaySubmitAccepted: Self = Self(0);

    // This request definitely did not admit the transaction because transaction
    // or auth validation failed against the state observed for this request.
    pub const SchedGatewaySubmitRejected: Self = Self(1);

    // This request definitely did not admit the transaction because of a
    // transient admission condition; retrying it unchanged may succeed.
    pub const SchedGatewaySubmitRetryable: Self = Self(2);

    // This request definitely did not admit the transaction because an internal
    // dependency failed. An RPC failure instead makes admission indeterminate.
    pub const SchedGatewaySubmitInternalError: Self = Self(3);
}
impl From<i32> for SchedGatewaySubmitOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedGatewaySubmitOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedGatewaySubmitAccepted.0 => {
                write!(f, "SchedGatewaySubmitOutcome::SchedGatewaySubmitAccepted")
            },
            x if x == Self::SchedGatewaySubmitRejected.0 => {
                write!(f, "SchedGatewaySubmitOutcome::SchedGatewaySubmitRejected")
            },
            x if x == Self::SchedGatewaySubmitRetryable.0 => {
                write!(f, "SchedGatewaySubmitOutcome::SchedGatewaySubmitRetryable")
            },
            x if x == Self::SchedGatewaySubmitInternalError.0 => {
                write!(f, "SchedGatewaySubmitOutcome::SchedGatewaySubmitInternalError")
            },
            _ => {
                write!(f, "SchedGatewaySubmitOutcome({})", self.0)
            }
        }
    }
}


pub type SchedGatewaySubmitOutcomeBatch = VecM::<SchedGatewaySubmitOutcome>;

// Number of transactions currently pending in the mempool.
pub type PendingTxCount = i64;


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SchedGateway_SchedGateway1: Send + Sync {    
    async fn schedgateway_intro(&self, arg0 : ProtocolCommit) -> Result<SchedGatewayIntroOutcome, ::rpc_api::ProgramError>;
        // Submit a batch of wallet-originated transactions to the
    // mempool.  Wallet clients learn about commit via
    // getTransactionStatus / getTransactionReceipt polling.

    async fn schedgateway_submit_tx_batch(&self, arg0 : ParallelSignedTransactionBatch) -> Result<(), ::rpc_api::ProgramError>;
        // Like schedgateway_submit_tx_batch, but returns the request-scoped,
    // exact-cardinality admission outcomes described above. A malformed batch
    // fails at the RPC level because its transactions cannot be identified
    // reliably.

    async fn schedgateway_submit_tx_batch_with_outcomes(&self, arg0 : ParallelSignedTransactionBatch) -> Result<SchedGatewaySubmitOutcomeBatch, ::rpc_api::ProgramError>;
        // Current committed tip.  Returns kRPCNotFound when the node
    // has no committed block yet (pre-genesis bootstrap).

    async fn schedgateway_get_tip(&self) -> Result<ExpectedDBTip, ::rpc_api::ProgramError>;
        // Lookup in-flight (not-yet-committed) transactions by hash.
    // Returns one OptSignedTransaction per input hash (same order).

    async fn schedgateway_lookup_tx(&self, arg0 : TransactionHashBatch) -> Result<OptSignedTransactionBatch, ::rpc_api::ProgramError>;
        // Number of transactions currently pending in the mempool.

    async fn schedgateway_pending_tx_count(&self) -> Result<PendingTxCount, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct SchedGateway_SchedGateway1_Dispatcher {
  inner: Box<dyn SchedGateway_SchedGateway1>,
}

impl SchedGateway_SchedGateway1_Dispatcher {
  pub fn new(inner: Box<dyn SchedGateway_SchedGateway1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for SchedGateway_SchedGateway1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x7ee7aaaa, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // schedgateway_intro
            0 => {
                let args: (ProtocolCommit, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_intro(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // schedgateway_submit_tx_batch
            1 => {
                let args: (ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_submit_tx_batch(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // schedgateway_submit_tx_batch_with_outcomes
            5 => {
                let args: (ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_submit_tx_batch_with_outcomes(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // schedgateway_get_tip
            2 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_get_tip().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // schedgateway_lookup_tx
            3 => {
                let args: (TransactionHashBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_lookup_tx(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // schedgateway_pending_tx_count
            4 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_pending_tx_count().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SchedGateway_SchedGateway1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl SchedGateway_SchedGateway1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x7ee7aaaa, 1)
    }
    pub async fn schedgateway_intro(&self, arg0 : &ProtocolCommit) -> Result<SchedGatewayIntroOutcome, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_submit_tx_batch(&self, arg0 : &ParallelSignedTransactionBatch) -> Result<(), ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_submit_tx_batch_with_outcomes(&self, arg0 : &ParallelSignedTransactionBatch) -> Result<SchedGatewaySubmitOutcomeBatch, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 5, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_get_tip(&self) -> Result<ExpectedDBTip, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_lookup_tx(&self, arg0 : &TransactionHashBatch) -> Result<OptSignedTransactionBatch, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 3, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_pending_tx_count(&self) -> Result<PendingTxCount, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 4, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for SchedGateway_SchedGateway1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


