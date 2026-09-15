// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 658b4a40c8451ea28d2bddb17f52ee1d5c485c65

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



// Uniquely describes an event server instance.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventCard {
    pub protocol: ProtocolCommit,
    pub upid: UPID,
    pub address_for_exec: ServerAddress,
    pub address_for_sched: ServerAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventCanonicalTip {
    pub block_id: BlockID,
    pub consensus_value: ConsensusValue,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct EventRegistrationSnapshot {
    pub tip: EventCanonicalTip,
    pub token: u64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct EventPrepareOutcome(pub i32);

impl EventPrepareOutcome {
    pub const EventPrepareOK: Self = Self(0);
    pub const EventPrepareNotReady: Self = Self(1);
    pub const EventPrepareError: Self = Self(2);
}
impl From<i32> for EventPrepareOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for EventPrepareOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::EventPrepareOK.0 => {
                write!(f, "EventPrepareOutcome::EventPrepareOK")
            },
            x if x == Self::EventPrepareNotReady.0 => {
                write!(f, "EventPrepareOutcome::EventPrepareNotReady")
            },
            x if x == Self::EventPrepareError.0 => {
                write!(f, "EventPrepareOutcome::EventPrepareError")
            },
            _ => {
                write!(f, "EventPrepareOutcome({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum EventPrepareResult {
    // case EventPrepareOK:
    //   EventRegistrationSnapshot snapshot;
    EventPrepareOK { snapshot: EventRegistrationSnapshot } = EventPrepareOutcome::EventPrepareOK.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct EventActivationOutcome(pub i32);

impl EventActivationOutcome {
    pub const EventActivationOK: Self = Self(0);
    pub const EventActivationRetry: Self = Self(1);
    pub const EventActivationError: Self = Self(2);
}
impl From<i32> for EventActivationOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for EventActivationOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::EventActivationOK.0 => {
                write!(f, "EventActivationOutcome::EventActivationOK")
            },
            x if x == Self::EventActivationRetry.0 => {
                write!(f, "EventActivationOutcome::EventActivationRetry")
            },
            x if x == Self::EventActivationError.0 => {
                write!(f, "EventActivationOutcome::EventActivationError")
            },
            _ => {
                write!(f, "EventActivationOutcome({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum EventActivationResult {
    // case EventActivationRetry:
    //   EventCanonicalTip current_tip;
    EventActivationRetry { current_tip: EventCanonicalTip } = EventActivationOutcome::EventActivationRetry.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SchedEvent_SchedEvent1: Send + Sync {    
    async fn schedevent_prepare_event(&self, arg0 : EventCard) -> Result<EventPrepareResult, ::rpc_api::ProgramError>;
    
    async fn schedevent_activate_event(&self, arg0 : EventRegistrationSnapshot) -> Result<EventActivationResult, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct SchedEvent_SchedEvent1_Dispatcher {
  inner: Box<dyn SchedEvent_SchedEvent1>,
}

impl SchedEvent_SchedEvent1_Dispatcher {
  pub fn new(inner: Box<dyn SchedEvent_SchedEvent1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for SchedEvent_SchedEvent1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x7ee79998, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // schedevent_prepare_event
            1 => {
                let args: (EventCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedevent_prepare_event(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // schedevent_activate_event
            2 => {
                let args: (EventRegistrationSnapshot, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedevent_activate_event(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SchedEvent_SchedEvent1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl SchedEvent_SchedEvent1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x7ee79998, 1)
    }
    pub async fn schedevent_prepare_event(&self, arg0 : &EventCard) -> Result<EventPrepareResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn schedevent_activate_event(&self, arg0 : &EventRegistrationSnapshot) -> Result<EventActivationResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for SchedEvent_SchedEvent1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl EventPrepareResult {
    pub fn outcome(&self) -> EventPrepareOutcome {
        match self {
            EventPrepareResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl EventActivationResult {
    pub fn outcome(&self) -> EventActivationOutcome {
        match self {
            EventActivationResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


