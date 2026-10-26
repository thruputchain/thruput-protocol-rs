// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 6ab835b07c234abb4b0b50b61d687904727a4f32

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



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstWriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DomainSeparator {
    pub ext: ExtensionPoint,
    pub domain_separator: BytesM<256>,
}


//
// XDR internals below this point
//


