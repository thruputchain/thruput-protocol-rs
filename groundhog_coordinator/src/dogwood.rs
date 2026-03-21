// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0b35b4bc56dc32f40c6de155f35b76f18a1da5f2

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


// trees

pub type DogwoodTreeHash = SHA256Hash;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodTreeNode {
    pub node: ValidatorID,
    pub children: VecM<DogwoodTreeHash>,
}

pub type DogwoodTreeNodes = VecM::<DogwoodTreeNode>;

// protocol

pub type DogwoodStreamID = DogwoodInt64;

pub type DogwoodTransmissionID = DogwoodInt64;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodStream {
    pub source_id: ValidatorID,
    pub stream_id: DogwoodStreamID,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodRoute {
    pub route_hash: Option<DogwoodTreeHash>,
    pub route_nodes: Option<DogwoodTreeNodes>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DogwoodTransmissionType(pub i32);

impl DogwoodTransmissionType {
    pub const DogwoodTransmissionPayload: Self = Self(0);
    pub const DogwoodTransmissionRoute: Self = Self(1);
}
impl From<i32> for DogwoodTransmissionType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for DogwoodTransmissionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::DogwoodTransmissionPayload.0 => {
                write!(f, "DogwoodTransmissionType::DogwoodTransmissionPayload")
            },
            x if x == Self::DogwoodTransmissionRoute.0 => {
                write!(f, "DogwoodTransmissionType::DogwoodTransmissionRoute")
            },
            _ => {
                write!(f, "DogwoodTransmissionType({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodTransmission {
    pub transmission_id: DogwoodTransmissionID,
    pub type_: DogwoodTransmissionType,
    pub num_payload_bytes: DogwoodInt64,
    pub num_payload_sheaves: DogwoodInt32,
    pub num_parity_sheaves: DogwoodInt32,
    pub max_bytes_per_chunk: DogwoodInt32,
    // used to shuffle sheafs for cross-sheaf coding
    pub shuffle_seed: DogwoodInt32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodChunk {
    pub sheaf_index: DogwoodInt32,
    pub chunk_index: DogwoodInt32,
    pub chunk: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodPackage {
    pub transmission: DogwoodTransmission,
    pub chunk: DogwoodChunk,
}

// messages

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodDelivery {
    pub stream: DogwoodStream,
    pub route: DogwoodRoute,
    pub pkg: DogwoodPackage,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodRouting {
    pub routes: VecM<DogwoodTreeNodes>,
}

// signed messages

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DogwoodDeliveryEnvelope {
    pub signature: Signature,
    pub delivery: DogwoodDelivery,
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait Dogwood_Dogwood1: Send + Sync {    
    async fn dogwood_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn dogwood_deliver(&self, arg0 : DogwoodDeliveryEnvelope) -> Result<(), ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct Dogwood_Dogwood1_Dispatcher {
  inner: Box<dyn Dogwood_Dogwood1>,
}

impl Dogwood_Dogwood1_Dispatcher {
  pub fn new(inner: Box<dyn Dogwood_Dogwood1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for Dogwood_Dogwood1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x8ee82002, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // dogwood_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dogwood_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // dogwood_deliver
            10 => {
                let args: (DogwoodDeliveryEnvelope, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.dogwood_deliver(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct Dogwood_Dogwood1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl Dogwood_Dogwood1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x8ee82002, 1)
    }
    pub async fn dogwood_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn dogwood_deliver(&self, arg0 : &DogwoodDeliveryEnvelope) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//


