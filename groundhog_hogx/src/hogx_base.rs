// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: bc2bfbfe6f157bc6b3da02ab427232e6dd9b855c

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



// clang-format on

// TODO: x23c does not allow an empty struct, it requires "void;" inside, whereas
// goxdr does not allow a "void;" field.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Void {
    pub xxx: Option<i32>,
}

pub type Timestamp = i64;

pub type Path = StringM;

pub type Tag = StringM;

pub type Flag = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct KeyString {
    pub key: StringM,
    pub value: StringM,
}

pub type StringMap = VecM::<KeyString>;

pub type SocketAddress = StringM;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SocketConnection {
    pub local: SocketAddress,
    pub remote: SocketAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct KeySocketAddress {
    pub key: StringM,
    // ip4 or ip6 + port
    pub address: StringM,
}

pub type SocketAddressMap = VecM::<KeySocketAddress>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct File {
    pub path: Path,
    pub content: StringM,
}

pub type Files = VecM::<File>;


//
// XDR internals below this point
//


