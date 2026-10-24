// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 855ee2b6f1b64eea7b421ca083525da4e6371c5a

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
pub struct SCPValidationLevel(pub i32);

impl SCPValidationLevel {
    pub const SCP_INVALID: Self = Self(0);
    pub const SCP_MAYBE_VALID: Self = Self(1);
    pub const SCP_FULLY_VALID: Self = Self(2);
}
impl From<i32> for SCPValidationLevel {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SCPValidationLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SCP_INVALID.0 => {
                write!(f, "SCPValidationLevel::SCP_INVALID")
            },
            x if x == Self::SCP_MAYBE_VALID.0 => {
                write!(f, "SCPValidationLevel::SCP_MAYBE_VALID")
            },
            x if x == Self::SCP_FULLY_VALID.0 => {
                write!(f, "SCPValidationLevel::SCP_FULLY_VALID")
            },
            _ => {
                write!(f, "SCPValidationLevel({})", self.0)
            }
        }
    }
}



/// Each handler replies with anything that encodes like the reply type.
/// Implementations can be plain `async fn`s returning a concrete reply type
/// under `#[allow(refining_impl_trait)]`.
#[allow(non_camel_case_types)]
pub trait SCP_Handle_SCP_Handle1: Send + Sync {    
    fn scp_handle_null(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
        // hooks


    fn scp_handle_emit_envelope(&self, arg0 : SCPEnvelope) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_validate_ballot_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SCPValidationLevel>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_validate_nomination_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SCPValidationLevel>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_extract_valid_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SCPOptValue>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_combine_candidates(&self, arg0 : SCPSlotIndex, arg1 : SCPValues) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SCPOptValue>, ::thruput_rpc_api::ProgramError>> + Send;
        // notifications


    fn scp_handle_value_externalized(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_nominating_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_updated_candidate_value(&self, arg0 : SCPSlotIndex, arg1 : SCPValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_started_ballot_proto(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_accepted_ballot_prepared(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_confirmed_ballot_prepared(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_accepted_commit(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn scp_handle_ballot_did_hear_from_quorum(&self, arg0 : SCPSlotIndex, arg1 : SCPBallot) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
        // utils

    fn scp_handle_get_quorum_set(&self, arg0 : SCPHash) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SCPOptQuorumSet>, ::thruput_rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct SCP_Handle_SCP_Handle1_Dispatcher<__Handler> {
  inner: Box<__Handler>,
}

impl<__Handler: SCP_Handle_SCP_Handle1> SCP_Handle_SCP_Handle1_Dispatcher<__Handler> {
  pub fn new(inner: Box<__Handler>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<__Handler: SCP_Handle_SCP_Handle1> ::thruput_rpc_api::RpcHandler for SCP_Handle_SCP_Handle1_Dispatcher<__Handler> {
    fn prog_vers(&self) -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x9aa92002, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::thruput_rpc_api::RpcError> {
        match proc {
            // scp_handle_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_null().await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_emit_envelope
            10 => {
                let args: (SCPEnvelope, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_emit_envelope(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_validate_ballot_value
            20 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_validate_ballot_value(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_validate_nomination_value
            30 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_validate_nomination_value(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_extract_valid_value
            40 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_extract_valid_value(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_combine_candidates
            50 => {
                let args: (SCPSlotIndex, SCPValues, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_combine_candidates(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_value_externalized
            100 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_value_externalized(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_nominating_value
            200 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_nominating_value(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_updated_candidate_value
            210 => {
                let args: (SCPSlotIndex, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_updated_candidate_value(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_started_ballot_proto
            310 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_started_ballot_proto(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_accepted_ballot_prepared
            320 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_accepted_ballot_prepared(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_confirmed_ballot_prepared
            330 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_confirmed_ballot_prepared(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_accepted_commit
            340 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_accepted_commit(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_ballot_did_hear_from_quorum
            350 => {
                let args: (SCPSlotIndex, SCPBallot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_ballot_did_hear_from_quorum(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // scp_handle_get_quorum_set
            1000 => {
                let args: (SCPHash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_handle_get_quorum_set(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            _ => Err(::thruput_rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SCP_Handle_SCP_Handle1_Client {
    handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>,
}

impl SCP_Handle_SCP_Handle1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x9aa92002, 1)
    }
    pub async fn scp_handle_null(&self) -> Result<(), ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 0, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_emit_envelope(&self, arg0 : &(impl xdrs::EncodesAs<SCPEnvelope> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 10, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_validate_ballot_value(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPValue> + ?Sized)) -> Result<SCPValidationLevel, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 20, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_validate_nomination_value(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPValue> + ?Sized)) -> Result<SCPValidationLevel, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 30, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_extract_valid_value(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPValue> + ?Sized)) -> Result<SCPOptValue, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 40, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_combine_candidates(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPValues> + ?Sized)) -> Result<SCPOptValue, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 50, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_value_externalized(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPValue> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 100, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_nominating_value(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPValue> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 200, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_updated_candidate_value(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPValue> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 210, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_started_ballot_proto(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPBallot> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 310, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_accepted_ballot_prepared(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPBallot> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 320, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_confirmed_ballot_prepared(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPBallot> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 330, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_accepted_commit(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPBallot> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 340, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_ballot_did_hear_from_quorum(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPBallot> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 350, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn scp_handle_get_quorum_set(&self, arg0 : &(impl xdrs::EncodesAs<SCPHash> + ?Sized)) -> Result<SCPOptQuorumSet, ::thruput_rpc_api::RpcError> {
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
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 1000, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>> for SCP_Handle_SCP_Handle1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


