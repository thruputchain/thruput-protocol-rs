// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 443aa4e97f3a0684c47b4f27fa9b4cd32b74f7fb

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
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;


// These define the native contract IDs within the Contract struct.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct HogexecPrecompiledNativeContracts(pub i32);

impl HogexecPrecompiledNativeContracts {
    pub const kNativeContractHogToken: Self = Self(0);
    pub const kNativeContractInitialTokenClaim: Self = Self(1);
}
impl From<i32> for HogexecPrecompiledNativeContracts {
    fn from(v: i32) -> Self {
        Self(v)
    }
}



//
// XDR internals below this point
//


