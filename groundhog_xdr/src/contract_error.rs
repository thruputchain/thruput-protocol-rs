// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 5fcc48434475432f0002d5f4bf88d312484a410d

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

#[cfg(feature="xdrs")]
use ::xdrs::{ReadXDR, WriteXDR};




// clang-format on

// This enum is intended to match the ContractError enum in the SDK.
// This enables parsing errors returned from contracts into other languages.
//
// This would normally be generated here and then referenced in the SDK,
// but we get more ergonomic code by handling it as a special-case there,
// and then generating a corresponding enum here (needless to say,
// these need to stay in sync).
// Additionally, XDR does not have support for generics,
// so the full Result type cannot be defined here.

// ErrorMsg is a string in testing, but is compiled out to nothing
// when a contract is built to a wasm.

// SysErr is a int32, which is derived from a SyscallErrorCode.
pub type SysErrXDR = i32;

// Storage Error:
// pub enum Error {
//    #[error("Storage error due to {0}")]
//    Syscall(#[from] SysErr),
//
//    #[error("Storage error due to serialization")]
//    Serialization,
//
//    #[error("Vector index out of bounds: len={len}, access={access}")]
//    IndexOutOfBounds { len: u64, access: u64 },
//}


#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum StorageErrorType {
    kStorageErrorTypeSyscall = 0,
    kStorageErrorTypeSerialization = 1,
    kStorageErrorTypeIndexOutOfBounds = 2,
}

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct IndexOutOfBoundsXDR {
    pub len: u64,
    pub access: u64,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
pub enum StorageErrXDR {
    // case kStorageErrorTypeSyscall:
    //   SysErrXDR syscall_err;
    kStorageErrorTypeSyscall { syscall_err: SysErrXDR } = StorageErrorType::kStorageErrorTypeSyscall as i32,
    // case kStorageErrorTypeSerialization:
    //   void;
    kStorageErrorTypeSerialization = StorageErrorType::kStorageErrorTypeSerialization as i32,
    // case kStorageErrorTypeIndexOutOfBounds:
    //   IndexOutOfBoundsXDR index_out_of_bounds_err;
    kStorageErrorTypeIndexOutOfBounds { index_out_of_bounds_err: IndexOutOfBoundsXDR } = StorageErrorType::kStorageErrorTypeIndexOutOfBounds as i32,
    
}

// pub enum ContractError {
//     #[error(": {0}")]
//     Custom(ErrorMsg),
//
//     #[error(": failed due to syscall error: {0}")]
//     Syscall(#[from] SysErr),
//
//     #[cfg(feature = "storage")]
//     #[error(": failed due to storage error: {0}")]
//     Storage(#[from] StorageErr),
// }


#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum ContractErrorType {
    kContractErrorTypeCustom = 0,
    kContractErrorTypeSyscall = 1,
    kContractErrorTypeStorage = 2,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
pub enum ContractErrorXDR {
    // case kContractErrorTypeCustom:
    //   // ErrorMsg
    //   void;
    kContractErrorTypeCustom = ContractErrorType::kContractErrorTypeCustom as i32,
    // case kContractErrorTypeSyscall:
    //   SysErrXDR syscall_err;
    kContractErrorTypeSyscall { syscall_err: SysErrXDR } = ContractErrorType::kContractErrorTypeSyscall as i32,
    // case kContractErrorTypeStorage:
    //   StorageErrXDR storage_err;
    kContractErrorTypeStorage { storage_err: StorageErrXDR } = ContractErrorType::kContractErrorTypeStorage as i32,
    
}

// Top level error returned from contract:
// pub enum Error {
//    #[error("Caller error{0}")]
//    Caller(ErrorMsg),
//
//    #[error("Contract internal error{0}")]
//    Contract(#[from] ContractError),
//}


#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum ErrorType {
    kErrorTypeCaller = 0,
    kErrorTypeContract = 1,
}

#[repr(i32)]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
pub enum ErrorXDR {
    // case kErrorTypeCaller:
    //   // ErrorMsg
    //   void;
    kErrorTypeCaller = ErrorType::kErrorTypeCaller as i32,
    // case kErrorTypeContract:
    //   ContractErrorXDR contract_err;
    kErrorTypeContract { contract_err: ContractErrorXDR } = ErrorType::kErrorTypeContract as i32,
    
}


//
// XDR internals below this point
//

impl StorageErrXDR {
    pub fn error_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ContractErrorXDR {
    pub fn error_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl ErrorXDR {
    pub fn error_type(&self) -> i32 {
        match self {
            
            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


