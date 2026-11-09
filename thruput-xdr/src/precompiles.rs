// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 1a0af366169a99b67ad0a6ed7a8eb58f45da5b7c

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


// These define the native contract IDs within the Contract struct.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct HogexecPrecompiledNativeContracts(pub i32);

impl HogexecPrecompiledNativeContracts {
    pub const kNativeContractNativeToken: Self = Self(0);

    // Native contract ID 1 is retired and must remain unused.
    pub const kNativeContractDefaultWallet: Self = Self(2);
}
impl From<i32> for HogexecPrecompiledNativeContracts {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "xdrs")]
impl xdrs::XDRDiscriminant for HogexecPrecompiledNativeContracts {
    const ENCODING: xdrs::DiscriminantEncoding = xdrs::DiscriminantEncoding::Signed;
    fn discriminant(&self) -> i32 {
        self.0
    }
    fn from_discriminant(d: i32) -> Self {
        Self(d)
    }
}

#[cfg(feature = "std")]
impl core::fmt::Display for HogexecPrecompiledNativeContracts {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kNativeContractNativeToken.0 => {
                write!(f, "HogexecPrecompiledNativeContracts::kNativeContractNativeToken")
            },
            x if x == Self::kNativeContractDefaultWallet.0 => {
                write!(f, "HogexecPrecompiledNativeContracts::kNativeContractDefaultWallet")
            },
            _ => {
                write!(f, "HogexecPrecompiledNativeContracts({})", self.0)
            }
        }
    }
}



//
// XDR internals below this point
//


