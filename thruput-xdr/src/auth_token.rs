// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0c2c59fb8ebeb0d6d37a1dad9cf88c4f22187ce6

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
/// Levels of node nesting a token may carry when a configuration does not
/// state `kConfigParamMaxAuthTokenDepth`. Consensus-critical, so frozen.
pub const kAuthTokenMaxLen : u32 = 256;

// 
pub const kAuthTokenMaxDepth : u32 = 4;

// 
pub const kAuthTokenMaxChildNodes : u32 = 4;

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct AuthTokenPayload {
    inner: BytesM<kAuthTokenMaxLen>,
}
impl From<BytesM<kAuthTokenMaxLen>> for AuthTokenPayload {
    fn from(_inner: BytesM<kAuthTokenMaxLen>) -> Self {
        Self { inner: _inner }
    }
}
impl AuthTokenPayload {
    pub fn into_inner(self) -> BytesM<kAuthTokenMaxLen> {
        self.inner
    }
    pub const fn new(_inner: BytesM<kAuthTokenMaxLen>) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for AuthTokenPayload {
    type Target = BytesM<kAuthTokenMaxLen>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for AuthTokenPayload {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthTokenNode {
    pub consumer_address: ContractAddress,
    pub payload: AuthTokenPayload,
    pub child_nodes: VecM<AuthTokenNode, kAuthTokenMaxChildNodes>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct AuthToken {
    pub mint_address: ContractAddress,
    pub expiry_height: BlockHeight,
    pub root_node: AuthTokenNode,
}


//
// XDR internals below this point
//


