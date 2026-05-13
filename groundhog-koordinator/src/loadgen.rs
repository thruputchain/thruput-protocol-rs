// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: ca84ca7f43039d74b7bb3e1dfd7a32766d798d77

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
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ExperimentType(pub i32);

impl ExperimentType {
    pub const kExperimentTypeERC20Payments: Self = Self(0);
    pub const kExperimentTypeERC20BasicAuthPayments: Self = Self(1);
}
impl From<i32> for ExperimentType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ExperimentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kExperimentTypeERC20Payments.0 => {
                write!(f, "ExperimentType::kExperimentTypeERC20Payments")
            },
            x if x == Self::kExperimentTypeERC20BasicAuthPayments.0 => {
                write!(f, "ExperimentType::kExperimentTypeERC20BasicAuthPayments")
            },
            _ => {
                write!(f, "ExperimentType({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ERC20PaymentsConfig {
    pub num_addresses: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum ExperimentConfig {
    // case kExperimentTypeERC20Payments:
    // case kExperimentTypeERC20BasicAuthPayments:
    //   ERC20PaymentsConfig erc20_payments;
    kExperimentTypeERC20Payments { erc20_payments: ERC20PaymentsConfig } = ExperimentType::kExperimentTypeERC20Payments.0 as i32,
    kExperimentTypeERC20BasicAuthPayments { erc20_payments: ERC20PaymentsConfig } = ExperimentType::kExperimentTypeERC20BasicAuthPayments.0 as i32,
    
}

pub type InitializationBlock = VecM::<SignedTransaction>;

pub type LoadgenContextID = u64;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LoadgenSuccessResult {
    pub genesis_configuration: GenesisConfiguration,
    pub initialization_txs: VecM<InitializationBlock>,
    pub context_id: LoadgenContextID,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LoadgenExtension {
    pub work_txs: ParallelSignedTransactionBatch,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct LoadgenRPCError(pub i32);

impl LoadgenRPCError {
    pub const kLoadgenRPCErrorOK: Self = Self(0);
    pub const kLoadgenRPCErrorInvalidConfig: Self = Self(1);
    pub const kLoadgenRPCErrorInvalidContext: Self = Self(2);
}
impl From<i32> for LoadgenRPCError {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for LoadgenRPCError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kLoadgenRPCErrorOK.0 => {
                write!(f, "LoadgenRPCError::kLoadgenRPCErrorOK")
            },
            x if x == Self::kLoadgenRPCErrorInvalidConfig.0 => {
                write!(f, "LoadgenRPCError::kLoadgenRPCErrorInvalidConfig")
            },
            x if x == Self::kLoadgenRPCErrorInvalidContext.0 => {
                write!(f, "LoadgenRPCError::kLoadgenRPCErrorInvalidContext")
            },
            _ => {
                write!(f, "LoadgenRPCError({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum LoadgenResult {
    // case kLoadgenRPCErrorOK:
    //   LoadgenSuccessResult success;
    kLoadgenRPCErrorOK { success: LoadgenSuccessResult } = LoadgenRPCError::kLoadgenRPCErrorOK.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum LoadgenExtensionResult {
    // case kLoadgenRPCErrorOK:
    //   LoadgenExtension success;
    kLoadgenRPCErrorOK { success: LoadgenExtension } = LoadgenRPCError::kLoadgenRPCErrorOK.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { error: i32 },
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait LoadgenInterface_LoadgenInterfaceV1: Send + Sync {    
    async fn generate_experiment(&self, arg0 : ExperimentConfig) -> Result<LoadgenResult, ::rpc_driver::ProgramError>;
        // arguments:
    // LoadgenContextID context_id (a handle returned by generate_experiment)
    // unsigned hyper num_txs (the number of transactions to return)
    // BlockHeight expiry_height (the height at which the transactions will expire)

    async fn extend_experiment(&self, arg0 : LoadgenContextID, arg1 : u64, arg2 : BlockHeight) -> Result<LoadgenExtensionResult, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct LoadgenInterface_LoadgenInterfaceV1_Dispatcher {
  inner: Box<dyn LoadgenInterface_LoadgenInterfaceV1>,
}

impl LoadgenInterface_LoadgenInterfaceV1_Dispatcher {
  pub fn new(inner: Box<dyn LoadgenInterface_LoadgenInterfaceV1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for LoadgenInterface_LoadgenInterfaceV1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(12341234, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // generate_experiment
            0 => {
                let args: (ExperimentConfig, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.generate_experiment(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // extend_experiment
            1 => {
                let args: (LoadgenContextID, u64, BlockHeight, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.extend_experiment(args.0, args.1, args.2).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct LoadgenInterface_LoadgenInterfaceV1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl LoadgenInterface_LoadgenInterfaceV1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(12341234, 1)
    }
    pub async fn generate_experiment(&self, arg0 : &ExperimentConfig) -> Result<LoadgenResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn extend_experiment(&self, arg0 : &LoadgenContextID, arg1 : u64, arg2 : &BlockHeight) -> Result<LoadgenExtensionResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, arg1, &arg2)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//

impl ExperimentConfig {
    pub fn experiment_type(&self) -> ExperimentType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl LoadgenResult {
    pub fn error(&self) -> LoadgenRPCError {
        match self {
            LoadgenResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl LoadgenExtensionResult {
    pub fn error(&self) -> LoadgenRPCError {
        match self {
            LoadgenExtensionResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


