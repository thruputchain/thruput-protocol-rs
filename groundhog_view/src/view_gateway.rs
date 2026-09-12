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

/// Successful view payload returned by ViewGateway1::viewgateway_view.
///
/// The height is the committed snapshot height used for the full view
/// execution, not a post-execution tip sample.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ViewGatewayPayload {
    pub height: BlockHeight,
    pub output: InvocationFrameOutput,
    pub events: VecM<TxEmittedEvent>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ViewGatewayOutcome(pub i32);

impl ViewGatewayOutcome {
    pub const ViewGatewayOk: Self = Self(0);

    // Raw view attempts returned Abandoned until the bounded service retry
    // budget was exhausted. Callers may retry the public view request later.
    pub const ViewGatewayAbandoned: Self = Self(1);

    // Server-side invariant failure or unavailable execution dependency.
    pub const ViewGatewayInternalError: Self = Self(2);
}
impl From<i32> for ViewGatewayOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ViewGatewayOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::ViewGatewayOk.0 => {
                write!(f, "ViewGatewayOutcome::ViewGatewayOk")
            },
            x if x == Self::ViewGatewayAbandoned.0 => {
                write!(f, "ViewGatewayOutcome::ViewGatewayAbandoned")
            },
            x if x == Self::ViewGatewayInternalError.0 => {
                write!(f, "ViewGatewayOutcome::ViewGatewayInternalError")
            },
            _ => {
                write!(f, "ViewGatewayOutcome({})", self.0)
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
pub enum ViewGatewayResult {
    // case ViewGatewayOk:
    //   ViewGatewayPayload payload;
    ViewGatewayOk { payload: ViewGatewayPayload } = ViewGatewayOutcome::ViewGatewayOk.0 as i32,
    // case ViewGatewayAbandoned:
    //   void;
    ViewGatewayAbandoned = ViewGatewayOutcome::ViewGatewayAbandoned.0 as i32,
    // case ViewGatewayInternalError:
    //   void;
    ViewGatewayInternalError = ViewGatewayOutcome::ViewGatewayInternalError.0 as i32,
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
pub struct ViewGatewayIntroOutcome(pub i32);

impl ViewGatewayIntroOutcome {
    pub const ViewGatewayIntroSuccess: Self = Self(0);
    pub const ViewGatewayIntroMismatchedProtocol: Self = Self(1);
}
impl From<i32> for ViewGatewayIntroOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ViewGatewayIntroOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::ViewGatewayIntroSuccess.0 => {
                write!(f, "ViewGatewayIntroOutcome::ViewGatewayIntroSuccess")
            },
            x if x == Self::ViewGatewayIntroMismatchedProtocol.0 => {
                write!(f, "ViewGatewayIntroOutcome::ViewGatewayIntroMismatchedProtocol")
            },
            _ => {
                write!(f, "ViewGatewayIntroOutcome({})", self.0)
            }
        }
    }
}



#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait ViewGateway_ViewGateway1: Send + Sync {    
    async fn viewgateway_intro(&self, arg0 : ProtocolCommit) -> Result<ViewGatewayIntroOutcome, ::rpc_api::ProgramError>;
    
    async fn viewgateway_view(&self, arg0 : TransactionViewQuery) -> Result<ViewGatewayResult, ::rpc_api::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct ViewGateway_ViewGateway1_Dispatcher {
  inner: Box<dyn ViewGateway_ViewGateway1>,
}

impl ViewGateway_ViewGateway1_Dispatcher {
  pub fn new(inner: Box<dyn ViewGateway_ViewGateway1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_api::RpcHandler for ViewGateway_ViewGateway1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x56475759, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // viewgateway_intro
            0 => {
                let args: (ProtocolCommit, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.viewgateway_intro(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // viewgateway_view
            1 => {
                let args: (TransactionViewQuery, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.viewgateway_view(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct ViewGateway_ViewGateway1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl ViewGateway_ViewGateway1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(0x56475759, 1)
    }
    pub async fn viewgateway_intro(&self, arg0 : &ProtocolCommit) -> Result<ViewGatewayIntroOutcome, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn viewgateway_view(&self, arg0 : &TransactionViewQuery) -> Result<ViewGatewayResult, ::rpc_api::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for ViewGateway_ViewGateway1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl ViewGatewayResult {
    pub fn outcome(&self) -> ViewGatewayOutcome {
        match self {
            ViewGatewayResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


