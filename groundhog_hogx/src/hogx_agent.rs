// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 3e16fec198246fef8b0deaab118d9f65fe1c6a9e

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};

#[cfg(feature="xdrs")]
use xdrs::*;
#[cfg(feature="spec")]
use xdrs::xdr_repr::*;


#[cfg(not(feature="xdrs"))]
use xdr_common::*;



// clang-format on

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum HogxOutcome {
    HOGX_OK = 0 as i32,
    HOGX_ERROR = 1 as i32,
}
pub use HogxOutcome::*;

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
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum HogxHostConfigureResult {
    // case HOGX_OK:
    //   void;
    HOGX_OK = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   BytesM error;
    HOGX_ERROR { error: BytesM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum HogxHostQueryResult {
    // case HOGX_OK:
    //   HostState host_state;
    HOGX_OK { host_state: HostState } = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   BytesM error;
    HOGX_ERROR { error: BytesM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum HogxServiceStartResult {
    // case HOGX_OK:
    //   ServiceState service_state;
    HOGX_OK { service_state: ServiceState } = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   BytesM error;
    HOGX_ERROR { error: BytesM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum HogxServiceStopResult {
    // case HOGX_OK:
    //   ServiceState service_state;
    HOGX_OK { service_state: ServiceState } = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   BytesM error;
    HOGX_ERROR { error: BytesM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum HogxServiceQueryResult {
    // case HOGX_OK:
    //   ServiceState service_state;
    HOGX_OK { service_state: ServiceState } = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   BytesM error;
    HOGX_ERROR { error: BytesM } = HogxOutcome::HOGX_ERROR as i32,
    
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum HogxServiceListResult {
    // case HOGX_OK:
    //   VecM<ServiceState> service_state;
    HOGX_OK { service_state: VecM<ServiceState> } = HogxOutcome::HOGX_OK as i32,
    // case HOGX_ERROR:
    //   BytesM error;
    HOGX_ERROR { error: BytesM } = HogxOutcome::HOGX_ERROR as i32,
    
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait HogxAgent_HogxAgent1: Send + Sync {    
    async fn hogxagent_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn hogxagent_host_configure(&self, arg0 : HostConfig) -> Result<HogxHostConfigureResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_host_query(&self) -> Result<HogxHostQueryResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_service_start(&self, arg0 : ServiceDef) -> Result<HogxServiceStartResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_service_stop(&self, arg0 : ServiceID) -> Result<HogxServiceStopResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_service_query(&self, arg0 : ServiceID) -> Result<HogxServiceQueryResult, ::rpc_driver::ProgramError>;
    
    async fn hogxagent_service_list(&self) -> Result<HogxServiceListResult, ::rpc_driver::ProgramError>;
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
                    xdrs::serialize_txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::serialize_txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_host_configure
            100 => {
                let args: (HostConfig, ) =
                    xdrs::serialize_txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_host_configure(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::serialize_txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_host_query
            110 => {
                let args: () =
                    xdrs::serialize_txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_host_query().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::serialize_txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_service_start
            200 => {
                let args: (ServiceDef, ) =
                    xdrs::serialize_txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_service_start(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::serialize_txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_service_stop
            210 => {
                let args: (ServiceID, ) =
                    xdrs::serialize_txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_service_stop(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::serialize_txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_service_query
            220 => {
                let args: (ServiceID, ) =
                    xdrs::serialize_txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_service_query(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::serialize_txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // hogxagent_service_list
            230 => {
                let args: () =
                    xdrs::serialize_txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.hogxagent_service_list().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::serialize_txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
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
         let data = xdrs::serialize_txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::serialize_txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_host_configure(&self, arg0 : &HostConfig) -> Result<HogxHostConfigureResult, ::rpc_driver::RpcError> {
         let data = xdrs::serialize_txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, &data).await?;
         xdrs::serialize_txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_host_query(&self) -> Result<HogxHostQueryResult, ::rpc_driver::RpcError> {
         let data = xdrs::serialize_txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 110, &data).await?;
         xdrs::serialize_txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_service_start(&self, arg0 : &ServiceDef) -> Result<HogxServiceStartResult, ::rpc_driver::RpcError> {
         let data = xdrs::serialize_txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 200, &data).await?;
         xdrs::serialize_txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_service_stop(&self, arg0 : &ServiceID) -> Result<HogxServiceStopResult, ::rpc_driver::RpcError> {
         let data = xdrs::serialize_txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 210, &data).await?;
         xdrs::serialize_txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_service_query(&self, arg0 : &ServiceID) -> Result<HogxServiceQueryResult, ::rpc_driver::RpcError> {
         let data = xdrs::serialize_txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 220, &data).await?;
         xdrs::serialize_txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn hogxagent_service_list(&self) -> Result<HogxServiceListResult, ::rpc_driver::RpcError> {
         let data = xdrs::serialize_txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 230, &data).await?;
         xdrs::serialize_txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
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

impl HogxServiceStartResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxServiceStopResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxServiceQueryResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl HogxServiceListResult {
    pub fn outcome(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


