// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 

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
pub enum ErrorType {
    ErrorTypeNone = 0 as i32,

    // Contract already initialized
    AlreadyInitialized = 1 as i32,

    // Caller must be contract owner
    MustBeContractOwner = 2 as i32,

    // "Internal contract error"
    InternalErr = -1 as i32,
}pub use ErrorType::*;


pub type Amount = uint128_xdr;

pub type Difference = int128_xdr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum ExpectedAmount {
    // case ErrorTypeNone:
    //   Amount amount;
    ErrorTypeNone { amount: Amount } = ErrorType::ErrorTypeNone as i32,
    // default:
    //   void;
    Default { err_type: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[repr(i32)]
pub enum ExpectedVoid {
    // default:
    //   void;
    Default { err_type: i32 },
}



//
// XDR internals below this point
//

impl ExpectedAmount {
    pub fn err_type(&self) -> i32 {
        match self {
            ExpectedAmount::Default{ err_type } => *err_type ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ExpectedVoid {
    pub fn err_type(&self) -> i32 {
        match self {
            ExpectedVoid::Default{ err_type } => *err_type ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


