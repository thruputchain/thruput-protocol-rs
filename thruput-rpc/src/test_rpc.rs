// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: e0e14dde6a5cd5db1bcaab5a5c5c8006735576c1

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

// A program to test the rpc driver, used in {s}rpcthroughput.
// This file is present in protocol/ to support cross-language RPC testing.

pub type Data = BytesM;


#[allow(non_camel_case_types)]
pub trait blank_rpc_blank_rpc_v1: Send + Sync {    
    fn null(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::rpc_api::ProgramError>> + Send;
    
    fn echo(&self, arg0 : Data) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<Data>, ::rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct blank_rpc_blank_rpc_v1_Dispatcher<T> {
  inner: Box<T>,
}

impl<T: blank_rpc_blank_rpc_v1> blank_rpc_blank_rpc_v1_Dispatcher<T> {
  pub fn new(inner: Box<T>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<T: blank_rpc_blank_rpc_v1> ::rpc_api::RpcHandler for blank_rpc_blank_rpc_v1_Dispatcher<T> {
    fn prog_vers(&self) -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(3123124, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_api::RpcError> {
        match proc {
            // null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.null().await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            // echo
            1 => {
                let args: (Data, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.echo(args.0).await.map_err(|_| ::rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_api::RpcError::SystemError)
            }
            _ => Err(::rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct blank_rpc_blank_rpc_v1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>,
}

impl blank_rpc_blank_rpc_v1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_api::ProgVers {
        ::rpc_api::ProgVers::new(3123124, 1)
    }
    pub async fn null(&self) -> Result<(), ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
    pub async fn echo(&self, arg0 : &(impl xdrs::EncodesAs<Data> + ?Sized)) -> Result<Data, ::rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::rpc_api::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, call).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_api::RpcSender>> for blank_rpc_blank_rpc_v1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}


//
// XDR internals below this point
//


