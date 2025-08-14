// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: d186232e27cd16b0bc090bb23cd5c474b02b30bf

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum StorageErrorType {
    kStorageErrorTypeSyscall = 0 as i32,
    kStorageErrorTypeSerialization = 1 as i32,
    kStorageErrorTypeIndexOutOfBounds = 2 as i32,
}
pub use StorageErrorType::*;

impl TryFrom<i32> for StorageErrorType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kStorageErrorTypeSyscall),
            1 => Ok(Self::kStorageErrorTypeSerialization),
            2 => Ok(Self::kStorageErrorTypeIndexOutOfBounds),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(C)]
pub struct IndexOutOfBoundsXDR {
    pub len: u64,
    pub access: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum ContractErrorType {
    kContractErrorTypeCustom = 0 as i32,
    kContractErrorTypeSyscall = 1 as i32,
    kContractErrorTypeStorage = 2 as i32,
}
pub use ContractErrorType::*;

impl TryFrom<i32> for ContractErrorType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kContractErrorTypeCustom),
            1 => Ok(Self::kContractErrorTypeSyscall),
            2 => Ok(Self::kContractErrorTypeStorage),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
pub enum ErrorType {
    kErrorTypeCaller = 0 as i32,
    kErrorTypeContract = 1 as i32,
}
pub use ErrorType::*;

impl TryFrom<i32> for ErrorType {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kErrorTypeCaller),
            1 => Ok(Self::kErrorTypeContract),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[repr(i32)]
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


