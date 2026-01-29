// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: c3c033fa8d0a0e017b73a0f38379c2491cd85e73

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};

#[cfg(feature="xdrs")]
use xdrs::*;
#[cfg(feature="spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;


// These define the native contract IDs within the Contract struct.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum HogexecPrecompiledNativeContracts {
    kNativeContractHogToken = 0 as i32,
    kNativeContractInitialTokenClaim = 1 as i32,
}
impl TryFrom<i32> for HogexecPrecompiledNativeContracts {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kNativeContractHogToken),
            1 => Ok(Self::kNativeContractInitialTokenClaim),
            _ => Err(()),
        }
    }
}



//
// XDR internals below this point
//


