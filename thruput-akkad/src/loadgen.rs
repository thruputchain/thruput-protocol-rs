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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ERC20PaymentsConfig {
    pub num_addresses: u64,
    pub network_hash: NetworkHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LoadgenSuccessResult {
    pub genesis_configuration: GenesisConfiguration,
    pub genesis_ceremony_txs: InitializationBlock,
    pub initialization_txs: VecM<InitializationBlock>,
    pub context_id: LoadgenContextID,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct LoadgenExtension {
    pub work_txs: ParallelSignedTransactionBatch,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
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
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
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


/// Each handler replies with anything that encodes like the reply type.
/// Implementations can be plain `async fn`s returning a concrete reply type
/// under `#[allow(refining_impl_trait)]`.
#[allow(non_camel_case_types)]
pub trait LoadgenInterface_LoadgenInterfaceV1: Send + Sync {    
    fn generate_experiment(&self, arg0 : ExperimentConfig) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<LoadgenResult>, ::thruput_rpc_api::ProgramError>> + Send;
        // arguments:
    // LoadgenContextID context_id (a handle returned by generate_experiment)
    // unsigned hyper num_txs (the number of transactions to return)
    // BlockHeight expiry_height (the height at which the transactions will expire)

    fn extend_experiment(&self, arg0 : LoadgenContextID, arg1 : u64, arg2 : BlockHeight) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<LoadgenExtensionResult>, ::thruput_rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct LoadgenInterface_LoadgenInterfaceV1_Dispatcher<__Handler> {
  inner: Box<__Handler>,
}

impl<__Handler: LoadgenInterface_LoadgenInterfaceV1> LoadgenInterface_LoadgenInterfaceV1_Dispatcher<__Handler> {
  pub fn new(inner: Box<__Handler>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<__Handler: LoadgenInterface_LoadgenInterfaceV1> ::thruput_rpc_api::RpcHandler for LoadgenInterface_LoadgenInterfaceV1_Dispatcher<__Handler> {
    fn prog_vers(&self) -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(12341234, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::thruput_rpc_api::RpcError> {
        match proc {
            // generate_experiment
            0 => {
                let args: (ExperimentConfig, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.generate_experiment(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // extend_experiment
            1 => {
                let args: (LoadgenContextID, u64, BlockHeight, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.extend_experiment(args.0, args.1, args.2).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            _ => Err(::thruput_rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct LoadgenInterface_LoadgenInterfaceV1_Client {
    handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>,
}

impl LoadgenInterface_LoadgenInterfaceV1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(12341234, 1)
    }
    pub async fn generate_experiment(&self, arg0 : &(impl xdrs::EncodesAs<ExperimentConfig> + ?Sized)) -> Result<LoadgenResult, ::thruput_rpc_api::RpcError> {
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
    pub async fn extend_experiment(&self, arg0 : &(impl xdrs::EncodesAs<LoadgenContextID> + ?Sized), arg1 : u64, arg2 : &(impl xdrs::EncodesAs<BlockHeight> + ?Sized)) -> Result<LoadgenExtensionResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1, arg2), &mut w);
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
}

impl From<::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>> for LoadgenInterface_LoadgenInterfaceV1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl ExperimentConfig {
    #[allow(clippy::match_single_binding)]
    pub fn experiment_type(&self) -> ExperimentType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl LoadgenResult {
    #[allow(clippy::match_single_binding)]
    pub fn error(&self) -> LoadgenRPCError {
        match self {
            LoadgenResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl LoadgenExtensionResult {
    #[allow(clippy::match_single_binding)]
    pub fn error(&self) -> LoadgenRPCError {
        match self {
            LoadgenExtensionResult::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


