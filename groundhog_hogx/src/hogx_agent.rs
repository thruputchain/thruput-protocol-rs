// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 667c466d8f5daf4d16a68920704d5ae7fc1660b0

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
pub struct HogxOutcome(pub i32);

impl HogxOutcome {
    pub const HOGX_OK: Self = Self(0);
    pub const HOGX_ERROR: Self = Self(1);
}
impl From<i32> for HogxOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for HogxOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::HOGX_OK.0 => {
                write!(f, "HogxOutcome::HOGX_OK")
            },
            x if x == Self::HOGX_ERROR.0 => {
                write!(f, "HogxOutcome::HOGX_ERROR")
            },
            _ => {
                write!(f, "HogxOutcome({})", self.0)
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
pub enum HogxHostConfigureResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK.0 as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR.0 as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum HogxHostQueryResult {
    // case HOGX_OK:
    //   HostState host_state;
    HOGX_OK { host_state: HostState } = HogxOutcome::HOGX_OK.0 as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR.0 as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum HogxUnitStartResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK.0 as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR.0 as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum HogxUnitStopResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK.0 as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR.0 as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum HogxUnitQueryResult {
    // case HOGX_OK:
    //   UnitState unit_state;
    HOGX_OK { unit_state: UnitState } = HogxOutcome::HOGX_OK.0 as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR.0 as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum HogxUnitListResult {
    // case HOGX_OK:
    //   VecM<UnitState> unit_state;
    HOGX_OK { unit_state: VecM<UnitState> } = HogxOutcome::HOGX_OK.0 as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR.0 as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum HogxUnitClearResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK.0 as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR.0 as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum HogxUnitTailResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK.0 as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR.0 as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HogxTailKick {
    pub tail_id: StringM,
    pub unit_id: UnitID,
    pub delivery_address: SocketAddress,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait HogxAgent_HogxAgent1: Send + Sync {    
    async fn hogxagent_null(&self) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn hogxagent_host_configure(&self, arg0 : HostConfig) -> Result<HogxHostConfigureResult, ::rpc_api::ProgramError>;
    
    async fn hogxagent_host_query(&self) -> Result<HogxHostQueryResult, ::rpc_api::ProgramError>;
    
    async fn hogxagent_unit_start(&self, arg0 : UnitDef) -> Result<HogxUnitStartResult, ::rpc_api::ProgramError>;
    
    async fn hogxagent_unit_stop(&self, arg0 : UnitID) -> Result<HogxUnitStopResult, ::rpc_api::ProgramError>;
    
    async fn hogxagent_unit_query(&self, arg0 : UnitID) -> Result<HogxUnitQueryResult, ::rpc_api::ProgramError>;
    
    async fn hogxagent_unit_list(&self) -> Result<HogxUnitListResult, ::rpc_api::ProgramError>;
    
    async fn hogxagent_unit_clear(&self, arg0 : UnitID) -> Result<HogxUnitClearResult, ::rpc_api::ProgramError>;
    
    async fn hogxagent_unit_tail(&self, arg0 : HogxTailKick) -> Result<HogxUnitTailResult, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct HogxAgent_HogxAgent1_Dispatcher {
  inner: Box<dyn HogxAgent_HogxAgent1>,
}

impl HogxAgent_HogxAgent1_Dispatcher {
  pub fn new(inner: Box<dyn HogxAgent_HogxAgent1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for HogxAgent_HogxAgent1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x20011004, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // hogxagent_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxagent_host_configure
            100 => {
                let args: (HostConfig, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_host_configure(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxagent_host_query
            110 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_host_query().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxagent_unit_start
            200 => {
                let args: (UnitDef, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_start(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxagent_unit_stop
            210 => {
                let args: (UnitID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_stop(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxagent_unit_query
            220 => {
                let args: (UnitID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_query(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxagent_unit_list
            230 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_list().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxagent_unit_clear
            240 => {
                let args: (UnitID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_clear(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxagent_unit_tail
            300 => {
                let args: (HogxTailKick, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_tail(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct HogxAgent_HogxAgent1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl HogxAgent_HogxAgent1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x20011004, 1)
    }
    pub async fn hogxagent_null(&self) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxagent_host_configure(&self, arg0 : &HostConfig) -> Result<HogxHostConfigureResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxagent_host_query(&self) -> Result<HogxHostQueryResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 110, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_start(&self, arg0 : &UnitDef) -> Result<HogxUnitStartResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 200, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_stop(&self, arg0 : &UnitID) -> Result<HogxUnitStopResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 210, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_query(&self, arg0 : &UnitID) -> Result<HogxUnitQueryResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 220, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_list(&self) -> Result<HogxUnitListResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 230, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_clear(&self, arg0 : &UnitID) -> Result<HogxUnitClearResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 240, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_tail(&self, arg0 : &HogxTailKick) -> Result<HogxUnitTailResult, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 300, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for HogxAgent_HogxAgent1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct HogxTailChunk {
    pub tail_id: StringM,
    pub unit_id: UnitID,
    pub index: u64,
    pub chunk: BytesM,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait HogxTail_HogxTail1: Send + Sync {    
    async fn hogxtail_null(&self) -> Result<(), ::rpc_api::ProgramError>;
    
    async fn hogxtail_deliver(&self, arg0 : HogxTailChunk) -> Result<bool, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct HogxTail_HogxTail1_Dispatcher {
  inner: Box<dyn HogxTail_HogxTail1>,
}

impl HogxTail_HogxTail1_Dispatcher {
  pub fn new(inner: Box<dyn HogxTail_HogxTail1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for HogxTail_HogxTail1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x20015004, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // hogxtail_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxtail_null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // hogxtail_deliver
            100 => {
                let args: (HogxTailChunk, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.hogxtail_deliver(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct HogxTail_HogxTail1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl HogxTail_HogxTail1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x20015004, 1)
    }
    pub async fn hogxtail_null(&self) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&()).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn hogxtail_deliver(&self, arg0 : &HogxTailChunk) -> Result<bool, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for HogxTail_HogxTail1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl HogxHostConfigureResult {
    pub fn outcome(&self) -> HogxOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl HogxHostQueryResult {
    pub fn outcome(&self) -> HogxOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl HogxUnitStartResult {
    pub fn outcome(&self) -> HogxOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl HogxUnitStopResult {
    pub fn outcome(&self) -> HogxOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl HogxUnitQueryResult {
    pub fn outcome(&self) -> HogxOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl HogxUnitListResult {
    pub fn outcome(&self) -> HogxOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl HogxUnitClearResult {
    pub fn outcome(&self) -> HogxOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl HogxUnitTailResult {
    pub fn outcome(&self) -> HogxOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


