// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature="std")]
use std::io::{Read, Write};

use ::xdrs::{ReadXDR, WriteXDR};




#[derive(Clone, PartialEq, Debug)]
#[repr(i32)]
pub enum ErrorType {
    ErrorTypeNone = 0,

    // Contract already initialized
    AlreadyInitialized = 1,

    // Caller must be contract owner
    MustBeContractOwner = 2,

    // "Internal contract error"
    InternalErr = -1,
}

pub type Amount = uint128_xdr;

pub type Difference = int128_xdr;

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[derive(ReadXDR, WriteXDR)]
pub enum ExpectedAmount {
    // case ErrorTypeNone:
    //   Amount amount;
    ErrorTypeNone { amount: Amount } = ErrorType::ErrorTypeNone as i32,
    // default:
    //   void;
    Default { err_type: i32 },
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[derive(ReadXDR, WriteXDR)]
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


