// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 738ffcebf4d4aa33f5880797b1f889b551bb5a62

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

// A program to test the rpc driver, used in {s}rpcthroughput.
// This file is present in protocol/ to support cross-language RPC testing.

pub type Data = BytesM;


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait blank_rpc_blank_rpc_v1: Send + Sync {    
    async fn null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn echo(&self, arg0 : Data) -> Result<Data, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct blank_rpc_blank_rpc_v1_Dispatcher {
  inner: Box<dyn blank_rpc_blank_rpc_v1>,
}

impl blank_rpc_blank_rpc_v1_Dispatcher {
  pub fn new(inner: Box<dyn blank_rpc_blank_rpc_v1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for blank_rpc_blank_rpc_v1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(3123124, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // echo
            1 => {
                let args: (Data, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.echo(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct blank_rpc_blank_rpc_v1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl blank_rpc_blank_rpc_v1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(3123124, 1)
    }
    pub async fn null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn echo(&self, arg0 : &Data) -> Result<Data, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}


//
// XDR internals below this point
//


