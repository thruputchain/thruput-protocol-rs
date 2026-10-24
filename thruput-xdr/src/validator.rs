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
#[cfg(feature = "xdrs")]
use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};



// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ValidatorID {
    #[cfg_attr(feature = "serde", serde(with = "xdr_common::serde_hex"))]
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
#[cfg(feature = "xdrs")]
impl<'r: 'a, 'a> ReadXDR<'r> for &'a ValidatorID {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 32] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}

#[cfg(feature = "schema")]
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



pub type ValidatorPublicKey = PublicKey;

pub type ValidatorSecretKey = SecretKey;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorPublicInfo {
    pub public_key: ValidatorPublicKey,
    pub address: ServerAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ValidatorPrivateInfo {
    pub public_info: ValidatorPublicInfo,
    pub secret_key: ValidatorSecretKey,
}


//
// XDR internals below this point
//


