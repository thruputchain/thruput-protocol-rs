// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 708ebf9bf535633eea730b810b2fb07494dca14e

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
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

// triple slash comments are added to the generated rust docs

/// Error codes returned by system calls.
/// Each syscall can return a specific subset of these error codes.
/// Check the syscalls.yaml documentation for which errors each syscall can return.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum SyscallErrorCode {
    /// No error
    kSyscallErrorCodeNone = 0 as i32,

    /// Out of bounds pointer to wasm memory passed to syscall
    kSyscallErrorCodeFault = 1 as i32,

    /// System call for supplied number is not supported
    kSyscallErrorCodeNoSyscall = 2 as i32,

    /// Cannot call get_msg_sender on root invocation
    kSyscallErrorCodeIsRootTransaction = 3 as i32,

    /// Attempt made to deploy a contract twice (same nonce and contract hash, from
    /// same address)
    kSyscallErrorCodeContractAlreadyDeployed = 5 as i32,

    /// No return buffer is available.  Either the most recent subcall failed, or
    /// there have been no subcalls within this call frame.
    kSyscallErrorCodeNoReturnBufferAvailable = 6 as i32,

    /// Invalid input to a syscall, such as subtracting below 0 on an nnint64
    /// or a bytestring that is too long
    kSyscallErrorCodeInvalidSyscallInput = 7 as i32,

    /// Accessing a storage key that does not exist in the current view
    kSyscallErrorCodeStorageKeyNotExist = 8 as i32,

    /// No entry in the retrocausal queue to consume
    kSyscallErrorCodeNoRetrocausalQueueEntry = 9 as i32,

    // ===== Error codes that signal the called frame reverted =====
    
    /// Deterministic error within called wasm (i.e. trap)
    kSyscallErrorCodeDeterministicWasm = -1 as i32,

    /// Child call frame manually reverted
    kSyscallErrorCodeFrameReverted = -2 as i32,
}
impl TryFrom<i32> for SyscallErrorCode {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::kSyscallErrorCodeNone),
            1 => Ok(Self::kSyscallErrorCodeFault),
            2 => Ok(Self::kSyscallErrorCodeNoSyscall),
            3 => Ok(Self::kSyscallErrorCodeIsRootTransaction),
            5 => Ok(Self::kSyscallErrorCodeContractAlreadyDeployed),
            6 => Ok(Self::kSyscallErrorCodeNoReturnBufferAvailable),
            7 => Ok(Self::kSyscallErrorCodeInvalidSyscallInput),
            8 => Ok(Self::kSyscallErrorCodeStorageKeyNotExist),
            9 => Ok(Self::kSyscallErrorCodeNoRetrocausalQueueEntry),
            -1 => Ok(Self::kSyscallErrorCodeDeterministicWasm),
            -2 => Ok(Self::kSyscallErrorCodeFrameReverted),
            _ => Err(()),
        }
    }
}



//
// XDR internals below this point
//


