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




#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait CoordScheduler_CoordScheduler1: Send + Sync {    
    async fn coordscheduler_null(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>) -> Result<(), ::rpc_driver::ProgramError>;
        // The scheduler calls coordscheduler_notify_vetted to indicate that the
    // given transactions are vetted, meaning the coordinator can broadcast
    // them to the network.

    async fn coordscheduler_notify_vetted(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : ParallelSignedTransactionBatch) -> Result<(), ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct CoordScheduler_CoordScheduler1_Dispatcher {
  inner: Box<dyn CoordScheduler_CoordScheduler1>,
}

impl CoordScheduler_CoordScheduler1_Dispatcher {
  pub fn new(inner: Box<dyn CoordScheduler_CoordScheduler1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::ServerDispatch for CoordScheduler_CoordScheduler1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x11117667, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8], client: &::std::sync::Arc<::rpc_driver::ClientHandle>) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // coordscheduler_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.coordscheduler_null(client).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // coordscheduler_notify_vetted
            1 => {
                let args: (ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.coordscheduler_notify_vetted(client, args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CoordScheduler_CoordScheduler1_Client {
    handle: ::std::sync::Arc<::rpc_driver::ClientHandle>,
}

impl CoordScheduler_CoordScheduler1_Client {
    pub fn new(handle: ::std::sync::Arc<::rpc_driver::ClientHandle>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x11117667, 1)
    }
    pub async fn coordscheduler_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn coordscheduler_notify_vetted(&self, arg0 : &ParallelSignedTransactionBatch) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//


