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




// clang-format on

#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[derive(Copy)]
#[repr(i32)]
pub enum SyscallErrorCode {
    // No error
    kNone = 0 as i32,

    // Out of bounds pointer to wasm memory passed to syscall
    kFault = 1 as i32,

    // System call for supplied number is not supported
    kNoSyscall = 2 as i32,

    // Cannot call get_msg_sender on root invocation
    kIsRootTransaction = 3 as i32,

    // Failed to find contract
    kContractLookupFailed = 4 as i32,

    // Attempt made to deploy a contract twice (same nonce and contract hash, from
    // same address)
    kContractAlreadyDeployed = 5 as i32,

    // No return buffer is available.  Either the most recent subcall failed, or
    // there have been no subcalls within this call frame.
    kNoReturnBufferAvailable = 6 as i32,

    // Insufficient gas in call frame to supply requested gas limit for child
    // invocation
    kInsufficientGasForInvocation = 7 as i32,

    // Invalid input to a syscall, such as subtracting below 0 on an nnint64
    kInvalidSyscallInput = 8 as i32,

    // Accessing a storage key that does not exist in the current view
    kStorageKeyNotExist = 9 as i32,

    // No entry in the retrocausal queue to consume
    kNoRetrocausalQueueEntry = 10 as i32,

    // Failed to parse calldata
    kCalldataParseError = 100 as i32,

    // Failed to parse returned data
    kReturnParseError = 101 as i32,

    // Deterministic error within called wasm (i.e. trap)
    kDeterministicWasm = -1 as i32,

    // Child call frame manually reverted
    kFrameReverted = -2 as i32,
}
pub use SyscallErrorCode::*;

impl TryFrom<i32> for SyscallErrorCode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kNone),
            1 => Ok(Self::kFault),
            2 => Ok(Self::kNoSyscall),
            3 => Ok(Self::kIsRootTransaction),
            4 => Ok(Self::kContractLookupFailed),
            5 => Ok(Self::kContractAlreadyDeployed),
            6 => Ok(Self::kNoReturnBufferAvailable),
            7 => Ok(Self::kInsufficientGasForInvocation),
            8 => Ok(Self::kInvalidSyscallInput),
            9 => Ok(Self::kStorageKeyNotExist),
            10 => Ok(Self::kNoRetrocausalQueueEntry),
            100 => Ok(Self::kCalldataParseError),
            101 => Ok(Self::kReturnParseError),
            -1 => Ok(Self::kDeterministicWasm),
            -2 => Ok(Self::kFrameReverted),
            _ => Err(()),
        }
    }
}



//
// XDR internals below this point
//


