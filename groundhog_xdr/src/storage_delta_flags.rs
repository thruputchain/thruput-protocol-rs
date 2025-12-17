// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 886f03972a2c44b0feddb4b5ba95d1b3d049d38a

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



#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[repr(i32)]
pub enum StorageItemRPCFlags {
    kStorageNone = 0 as i32,

    // Should the modified item be exclusively locked, in addition to the
    // modification performed.
    kStorageExclusiveLock = 0x00000001 as i32,
}
pub use StorageItemRPCFlags::*;

impl TryFrom<i32> for StorageItemRPCFlags {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kStorageNone),
            0x00000001 => Ok(Self::kStorageExclusiveLock),
            _ => Err(()),
        }
    }
}


pub type StorageItemRPCFlagSet = i32;


//
// XDR internals below this point
//


