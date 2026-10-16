// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 6274c208f1d444c1450f0a2111b8b1918e067237

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




#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SCP_Control_SCP_Control1: Send + Sync {    
    async fn scp_control_null(&self) -> Result<(), ::rpc_api::ProgramError>;
        // arguments: slot index, value to nominate, previous value

    async fn scp_control_nominate(&self, arg0 : SCPSlotIndex, arg1 : SCPValue, arg2 : SCPValue) -> Result<bool, ::rpc_api::ProgramError>;
    
    async fn scp_control_stop_nomination(&self, arg0 : SCPSlotIndex) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_control_update_local_quorum_set(&self, arg0 : SCPQuorumSet) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_control_received_envelope(&self, arg0 : SCPEnvelope) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_control_set_state_from_envelope(&self, arg0 : SCPSlotIndex, arg1 : SCPEnvelope) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn scp_control_set_qset(&self, arg0 : SCPQuorumSet) -> Result<SCPOptQuorumSet, ::rpc_api::ProgramError>;
    
    async fn scp_control_get_qset(&self, arg0 : SCPHash) -> Result<SCPOptQuorumSet, ::rpc_api::ProgramError>;
    
    async fn scp_control_remove_qset(&self, arg0 : SCPHash) -> Result<SCPOptQuorumSet, ::rpc_api::ProgramError>;
    
    async fn scp_control_set_lowest_non_externalized_slot(&self, arg0 : SCPSlotIndex) -> Result<(), ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct SCP_Control_SCP_Control1_Dispatcher {
  inner: Box<dyn SCP_Control_SCP_Control1>,
}

impl SCP_Control_SCP_Control1_Dispatcher {
  pub fn new(inner: Box<dyn SCP_Control_SCP_Control1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for SCP_Control_SCP_Control1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x9aa93003, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // scp_control_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_nominate
            10 => {
                let args: (SCPSlotIndex, SCPValue, SCPValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_nominate(args.0, args.1, args.2).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_stop_nomination
            20 => {
                let args: (SCPSlotIndex, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_stop_nomination(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_update_local_quorum_set
            100 => {
                let args: (SCPQuorumSet, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_update_local_quorum_set(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_received_envelope
            200 => {
                let args: (SCPEnvelope, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_received_envelope(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_set_state_from_envelope
            201 => {
                let args: (SCPSlotIndex, SCPEnvelope, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_set_state_from_envelope(args.0, args.1).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_set_qset
            300 => {
                let args: (SCPQuorumSet, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_set_qset(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_get_qset
            301 => {
                let args: (SCPHash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_get_qset(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_remove_qset
            302 => {
                let args: (SCPHash, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_remove_qset(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // scp_control_set_lowest_non_externalized_slot
            400 => {
                let args: (SCPSlotIndex, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.scp_control_set_lowest_non_externalized_slot(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SCP_Control_SCP_Control1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl SCP_Control_SCP_Control1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x9aa93003, 1)
    }
    pub async fn scp_control_null(&self) -> Result<(), ::rpc_api::RpcError> {
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
    pub async fn scp_control_nominate(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPValue> + ?Sized), arg2 : &(impl xdrs::EncodesAs<SCPValue> + ?Sized)) -> Result<bool, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1, arg2), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_control_stop_nomination(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized)) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 20, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_control_update_local_quorum_set(&self, arg0 : &(impl xdrs::EncodesAs<SCPQuorumSet> + ?Sized)) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_control_received_envelope(&self, arg0 : &(impl xdrs::EncodesAs<SCPEnvelope> + ?Sized)) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 200, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_control_set_state_from_envelope(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized), arg1 : &(impl xdrs::EncodesAs<SCPEnvelope> + ?Sized)) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 201, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_control_set_qset(&self, arg0 : &(impl xdrs::EncodesAs<SCPQuorumSet> + ?Sized)) -> Result<SCPOptQuorumSet, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 300, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_control_get_qset(&self, arg0 : &(impl xdrs::EncodesAs<SCPHash> + ?Sized)) -> Result<SCPOptQuorumSet, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 301, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_control_remove_qset(&self, arg0 : &(impl xdrs::EncodesAs<SCPHash> + ?Sized)) -> Result<SCPOptQuorumSet, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 302, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn scp_control_set_lowest_non_externalized_slot(&self, arg0 : &(impl xdrs::EncodesAs<SCPSlotIndex> + ?Sized)) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 400, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for SCP_Control_SCP_Control1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//


