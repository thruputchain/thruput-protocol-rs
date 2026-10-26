// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 6ab835b07c234abb4b0b50b61d687904727a4f32

// clang-format off

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
    // The transaction was admitted to the mempool, or the same transaction body
    // was already pending. This does not mean it was nominated or committed.
    pub const SchedGatewaySubmitAdmitted: Self = Self(0);

    // This request definitely did not admit the transaction because
    // authentication failed against the state observed for this request.
    pub const SchedGatewaySubmitAuthenticationRejected: Self = Self(1);

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
            x if x == Self::SchedGatewaySubmitAdmitted.0 => {
                write!(f, "SchedGatewaySubmitOutcome::SchedGatewaySubmitAdmitted")
            },
            x if x == Self::SchedGatewaySubmitAuthenticationRejected.0 => {
                write!(f, "SchedGatewaySubmitOutcome::SchedGatewaySubmitAuthenticationRejected")
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


/// Each handler replies with anything that encodes like the reply type.
/// Implementations can be plain `async fn`s returning a concrete reply type
/// under `#[allow(refining_impl_trait)]`.
#[allow(non_camel_case_types)]
pub trait SchedGateway_SchedGateway1: Send + Sync {    
    fn schedgateway_intro(&self, arg0 : ProtocolCommit) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SchedGatewayIntroOutcome>, ::thruput_rpc_api::ProgramError>> + Send;
        // Submit wallet-originated transactions and return the request-scoped,
    // exact-cardinality admission outcomes described above. A malformed batch
    // fails at the RPC level because its transactions cannot be identified
    // reliably. Admission does not imply commitment; clients learn about commit
    // via getTransactionStatus / getTransactionReceipt polling.

    fn schedgateway_submit_tx_batch(&self, arg0 : ParallelSignedTransactionBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SchedGatewaySubmitOutcomeBatch>, ::thruput_rpc_api::ProgramError>> + Send;
        // Current committed tip.  Returns kRPCNotFound when the node
    // has no committed block yet (pre-genesis bootstrap).

    fn schedgateway_get_tip(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<ExpectedDBTip>, ::thruput_rpc_api::ProgramError>> + Send;
        // Lookup in-flight (not-yet-committed) transactions by TransactionHash, returning
    // whichever signing of each body is cached.
    // Returns one OptSignedTransaction per input hash (same order).

    fn schedgateway_lookup_tx(&self, arg0 : TransactionHashBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<OptSignedTransactionBatch>, ::thruput_rpc_api::ProgramError>> + Send;
        // Number of transactions currently pending in the mempool.

    fn schedgateway_pending_tx_count(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<PendingTxCount>, ::thruput_rpc_api::ProgramError>> + Send;
        // Returns one pending or recent outcome per input hash (same order).

    fn schedgateway_lookup_transaction_outcomes(&self, arg0 : TransactionHashBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SchedulerTransactionOutcomeBatch>, ::thruput_rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct SchedGateway_SchedGateway1_Dispatcher<__Handler> {
  inner: Box<__Handler>,
}

impl<__Handler: SchedGateway_SchedGateway1> SchedGateway_SchedGateway1_Dispatcher<__Handler> {
  pub fn new(inner: Box<__Handler>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<__Handler: SchedGateway_SchedGateway1> ::thruput_rpc_api::RpcHandler for SchedGateway_SchedGateway1_Dispatcher<__Handler> {
    fn prog_vers(&self) -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x7ee7aaaa, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::thruput_rpc_api::RpcError> {
        match proc {
            // schedgateway_intro
            0 => {
                let args: (ProtocolCommit, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_intro(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedgateway_submit_tx_batch
            1 => {
                let args: (ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_submit_tx_batch(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedgateway_get_tip
            2 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_get_tip().await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedgateway_lookup_tx
            3 => {
                let args: (TransactionHashBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_lookup_tx(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedgateway_pending_tx_count
            4 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_pending_tx_count().await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedgateway_lookup_transaction_outcomes
            5 => {
                let args: (TransactionHashBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedgateway_lookup_transaction_outcomes(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            _ => Err(::thruput_rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SchedGateway_SchedGateway1_Client {
    handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>,
}

impl SchedGateway_SchedGateway1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x7ee7aaaa, 1)
    }
    pub async fn schedgateway_intro(&self, arg0 : &(impl xdrs::EncodesAs<ProtocolCommit> + ?Sized)) -> Result<SchedGatewayIntroOutcome, ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 0, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_submit_tx_batch(&self, arg0 : &(impl xdrs::EncodesAs<ParallelSignedTransactionBatch> + ?Sized)) -> Result<SchedGatewaySubmitOutcomeBatch, ::thruput_rpc_api::RpcError> {
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
    pub async fn schedgateway_get_tip(&self) -> Result<ExpectedDBTip, ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 2, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_lookup_tx(&self, arg0 : &(impl xdrs::EncodesAs<TransactionHashBatch> + ?Sized)) -> Result<OptSignedTransactionBatch, ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 3, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_pending_tx_count(&self) -> Result<PendingTxCount, ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 4, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedgateway_lookup_transaction_outcomes(&self, arg0 : &(impl xdrs::EncodesAs<TransactionHashBatch> + ?Sized)) -> Result<SchedulerTransactionOutcomeBatch, ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 5, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>> for SchedGateway_SchedGateway1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


