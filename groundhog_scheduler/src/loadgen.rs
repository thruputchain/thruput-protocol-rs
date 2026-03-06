// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: c95abcc628bcb7818be237c2f2081a6b45f156b4

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
pub enum ExperimentType {
    kExperimentTypeERC20Payments = 0 as i32,
    kExperimentTypeERC20BasicAuthPayments = 1 as i32,
}
impl TryFrom<i32> for ExperimentType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kExperimentTypeERC20Payments),
            1 => Ok(Self::kExperimentTypeERC20BasicAuthPayments),
            _ => Err(()),
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ERC20PaymentsConfig {
    pub num_addresses: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum ExperimentConfig {
    // case kExperimentTypeERC20Payments:
    // case kExperimentTypeERC20BasicAuthPayments:
    //   ERC20PaymentsConfig erc20_payments;
    kExperimentTypeERC20Payments { erc20_payments: ERC20PaymentsConfig } = ExperimentType::kExperimentTypeERC20Payments as i32,
    kExperimentTypeERC20BasicAuthPayments { erc20_payments: ERC20PaymentsConfig } = ExperimentType::kExperimentTypeERC20BasicAuthPayments as i32,
    
}

pub type InitializationBlock = VecM::<SignedTransaction>;

pub type LoadgenContextID = u64;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LoadgenSuccessResult {
    pub genesis_configuration: GenesisConfiguration,
    pub initialization_txs: VecM<InitializationBlock>,
    pub context_id: LoadgenContextID,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LoadgenExtension {
    pub work_txs: ParallelSignedTransactionBatch,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum LoadgenRPCError {
    kLoadgenRPCErrorOK = 0 as i32,
    kLoadgenRPCErrorInvalidConfig = 1 as i32,
    kLoadgenRPCErrorInvalidContext = 2 as i32,
}
impl TryFrom<i32> for LoadgenRPCError {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kLoadgenRPCErrorOK),
            1 => Ok(Self::kLoadgenRPCErrorInvalidConfig),
            2 => Ok(Self::kLoadgenRPCErrorInvalidContext),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum LoadgenResult {
    // case kLoadgenRPCErrorOK:
    //   LoadgenSuccessResult success;
    kLoadgenRPCErrorOK { success: LoadgenSuccessResult } = LoadgenRPCError::kLoadgenRPCErrorOK as i32,
    // default:
    //   void;
    Default { error: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum LoadgenExtensionResult {
    // case kLoadgenRPCErrorOK:
    //   LoadgenExtension success;
    kLoadgenRPCErrorOK { success: LoadgenExtension } = LoadgenRPCError::kLoadgenRPCErrorOK as i32,
    // default:
    //   void;
    Default { error: i32 },
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait LoadgenInterface_LoadgenInterfaceV1: Send + Sync {    
    async fn generate_experiment(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : ExperimentConfig) -> Result<LoadgenResult, ::rpc_driver::ProgramError>;
        // arguments:
    // LoadgenContextID context_id (a handle returned by generate_experiment)
    // unsigned hyper num_txs (the number of transactions to return)
    // BlockHeight expiry_height (the height at which the transactions will expire)

    async fn extend_experiment(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : LoadgenContextID, arg1 : u64, arg2 : BlockHeight) -> Result<LoadgenExtensionResult, ::rpc_driver::ProgramError>;
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
impl ::rpc_driver::ServerDispatch for LoadgenInterface_LoadgenInterfaceV1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(12341234, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8], client: &::std::sync::Arc<::rpc_driver::ClientHandle>) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // generate_experiment
            0 => {
                let args: (ExperimentConfig, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.generate_experiment(client, args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // extend_experiment
            1 => {
                let args: (LoadgenContextID, u64, BlockHeight, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.extend_experiment(client, args.0, args.1, args.2).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LoadgenInterface_LoadgenInterfaceV1_Client {
    handle: ::std::sync::Arc<::rpc_driver::ClientHandle>,
}

impl LoadgenInterface_LoadgenInterfaceV1_Client {
    pub fn new(handle: ::std::sync::Arc<::rpc_driver::ClientHandle>) -> Self {
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
    pub fn experiment_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl LoadgenResult {
    pub fn error(&self) -> i32 {
        match self {
            LoadgenResult::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl LoadgenExtensionResult {
    pub fn error(&self) -> i32 {
        match self {
            LoadgenExtensionResult::Default{ error } => *error ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


