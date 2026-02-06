// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 8afc1e2714d6876f31ee25c4344341aefc1b9c5f

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
#[derive(Copy)]
#[repr(i32)]
pub enum HogxOutcome {
    HOGX_OK = 0 as i32,
    HOGX_ERROR = 1 as i32,
}
impl TryFrom<i32> for HogxOutcome {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::HOGX_OK),
            1 => Ok(Self::HOGX_ERROR),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum HogxHostConfigureResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum HogxHostQueryResult {
    // case HOGX_OK:
    //   HostState host_state;
    HOGX_OK { host_state: HostState } = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum HogxUnitStartResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum HogxUnitStopResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum HogxUnitQueryResult {
    // case HOGX_OK:
    //   UnitState unit_state;
    HOGX_OK { unit_state: UnitState } = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum HogxUnitListResult {
    // case HOGX_OK:
    //   VecM<UnitState> unit_state;
    HOGX_OK { unit_state: VecM<UnitState> } = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum HogxUnitClearResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum HogxUnitTailResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   StringM error;
    HOGX_ERROR { error: StringM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

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
    async fn hogxagent_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn hogxagent_host_configure(&self, arg0 : HostConfig) -> Result<HogxHostConfigureResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_host_query(&self) -> Result<HogxHostQueryResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_unit_start(&self, arg0 : UnitDef) -> Result<HogxUnitStartResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_unit_stop(&self, arg0 : UnitID) -> Result<HogxUnitStopResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_unit_query(&self, arg0 : UnitID) -> Result<HogxUnitQueryResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_unit_list(&self) -> Result<HogxUnitListResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_unit_clear(&self, arg0 : UnitID) -> Result<HogxUnitClearResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_unit_tail(&self, arg0 : HogxTailKick) -> Result<HogxUnitTailResult, ::rpc_driver::ProgramError>;
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
impl ::rpc_driver::ServerDispatch for HogxAgent_HogxAgent1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x20011004, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // hogxagent_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_host_configure
            100 => {
                let args: (HostConfig, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_host_configure(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_host_query
            110 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_host_query().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_unit_start
            200 => {
                let args: (UnitDef, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_start(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_unit_stop
            210 => {
                let args: (UnitID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_stop(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_unit_query
            220 => {
                let args: (UnitID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_query(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_unit_list
            230 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_list().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_unit_clear
            240 => {
                let args: (UnitID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_clear(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_unit_tail
            300 => {
                let args: (HogxTailKick, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_unit_tail(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HogxAgent_HogxAgent1_Client {
    handle: ::std::sync::Arc<::rpc_driver::ClientHandle>,
}

impl HogxAgent_HogxAgent1_Client {
    pub fn new(handle: ::std::sync::Arc<::rpc_driver::ClientHandle>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x20011004, 1)
    }
    pub async fn hogxagent_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_host_configure(&self, arg0 : &HostConfig) -> Result<HogxHostConfigureResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_host_query(&self) -> Result<HogxHostQueryResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 110, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_start(&self, arg0 : &UnitDef) -> Result<HogxUnitStartResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 200, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_stop(&self, arg0 : &UnitID) -> Result<HogxUnitStopResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 210, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_query(&self, arg0 : &UnitID) -> Result<HogxUnitQueryResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 220, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_list(&self) -> Result<HogxUnitListResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 230, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_clear(&self, arg0 : &UnitID) -> Result<HogxUnitClearResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 240, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_unit_tail(&self, arg0 : &HogxTailKick) -> Result<HogxUnitTailResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 300, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

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
    async fn hogxtail_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn hogxtail_deliver(&self, arg0 : HogxTailChunk) -> Result<bool, ::rpc_driver::ProgramError>;
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
impl ::rpc_driver::ServerDispatch for HogxTail_HogxTail1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x20015004, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // hogxtail_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxtail_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxtail_deliver
            100 => {
                let args: (HogxTailChunk, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxtail_deliver(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HogxTail_HogxTail1_Client {
    handle: ::std::sync::Arc<::rpc_driver::ClientHandle>,
}

impl HogxTail_HogxTail1_Client {
    pub fn new(handle: ::std::sync::Arc<::rpc_driver::ClientHandle>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x20015004, 1)
    }
    pub async fn hogxtail_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxtail_deliver(&self, arg0 : &HogxTailChunk) -> Result<bool, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//

impl HogxHostConfigureResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxHostQueryResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxUnitStartResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxUnitStopResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxUnitQueryResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxUnitListResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxUnitClearResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxUnitTailResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


