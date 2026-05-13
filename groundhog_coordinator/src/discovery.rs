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



// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ValidatorID {
    #[cfg_attr(feature = "serde", serde(with = "serde_arrays::hex"))]
    inner: [u8; 32 as usize],
}
impl From<[u8; 32 as usize]> for ValidatorID {
    fn from(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl ValidatorID {
    pub fn into_inner(self) -> [u8; 32 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ValidatorID {
    type Target = [u8; 32 as usize];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ValidatorID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for ValidatorID {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}
impl<'a> From<&'a [u8; 32]> for &'a ValidatorID {
    fn from(arr: &[u8; 32]) -> Self {
        let ptr = arr as *const [u8; 32] as *const ValidatorID;
        unsafe { &*ptr }
    }
}
impl<'r> ReadXDR<'r> for &'r ValidatorID {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 32] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}

impl schemars::JsonSchema for ValidatorID {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "ValidatorID".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 64,
            "maxLength": 64
        })
    }
}



pub type ValidatorPublicKey = Ed25519PublicKey;

pub type ValidatorSecretKey = Ed25519SecretKey;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorPublicInfo {
    pub public_key: ValidatorPublicKey,
    pub address: ServerAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorPrivateInfo {
    pub public_info: ValidatorPublicInfo,
    pub secret_key: ValidatorSecretKey,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct QuorumSetSnapshot {
    pub timestamp: TimePoint,
    pub quorum_set: SCPQuorumSet,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedQuorumSetSnapshot {
    pub signer: ValidatorPublicInfo,
    pub signature: Signature,
    pub quorum_set_snapshot: QuorumSetSnapshot,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorInfoSnapshot {
    pub timestamp: TimePoint,
    pub validator: ValidatorPublicInfo,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedValidatorInfoSnapshot {
    pub signer: ValidatorPublicInfo,
    pub signature: Signature,
    pub validator_snapshot: ValidatorInfoSnapshot,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct KnownQuorumSets {
    pub quorum_set_snapshots: VecM<SignedQuorumSetSnapshot>,
    pub validator_snapshots: VecM<SignedValidatorInfoSnapshot>,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait Discovery_Discovery1: Send + Sync {    
    async fn discovery_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn discovery_update_quorums(&self, arg0 : KnownQuorumSets) -> Result<KnownQuorumSets, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct Discovery_Discovery1_Dispatcher {
  inner: Box<dyn Discovery_Discovery1>,
}

impl Discovery_Discovery1_Dispatcher {
  pub fn new(inner: Box<dyn Discovery_Discovery1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for Discovery_Discovery1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x3eef0505, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // discovery_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // discovery_update_quorums
            10 => {
                let args: (KnownQuorumSets, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.discovery_update_quorums(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct Discovery_Discovery1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl Discovery_Discovery1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x3eef0505, 1)
    }
    pub async fn discovery_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn discovery_update_quorums(&self, arg0 : &KnownQuorumSets) -> Result<KnownQuorumSets, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//


