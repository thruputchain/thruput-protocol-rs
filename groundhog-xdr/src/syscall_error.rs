// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 443aa4e97f3a0684c47b4f27fa9b4cd32b74f7fb

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
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SyscallErrorCode(pub i32);

impl SyscallErrorCode {
    /// No error
    pub const kSyscallErrorCodeNone: Self = Self(0);

    /// Out of bounds pointer to wasm memory passed to syscall
    pub const kSyscallErrorCodeFault: Self = Self(1);

    /// System call for supplied number is not supported
    pub const kSyscallErrorCodeNoSyscall: Self = Self(2);

    /// Cannot call get_msg_sender on root invocation
    pub const kSyscallErrorCodeIsRootTransaction: Self = Self(3);

    /// Attempt made to deploy a contract twice (same nonce and contract hash, from
    /// same address)
    pub const kSyscallErrorCodeContractAlreadyDeployed: Self = Self(5);

    /// No return buffer is available.  Either the most recent subcall failed, or
    /// there have been no subcalls within this call frame.
    pub const kSyscallErrorCodeNoReturnBufferAvailable: Self = Self(6);

    /// Invalid input to a syscall, such as subtracting below 0 on an nnint64
    /// or a bytestring that is too long
    pub const kSyscallErrorCodeInvalidSyscallInput: Self = Self(7);

    /// Accessing a storage key that does not exist in the current view
    pub const kSyscallErrorCodeStorageKeyNotExist: Self = Self(8);

    /// No entry in the retrocausal queue to consume
    pub const kSyscallErrorCodeNoRetrocausalQueueEntry: Self = Self(9);

    // ===== Error codes that signal the called frame reverted =====
    
    /// Deterministic error within called wasm (i.e. trap)
    pub const kSyscallErrorCodeDeterministicWasm: Self = Self(-1);

    /// Child call frame manually reverted
    pub const kSyscallErrorCodeFrameReverted: Self = Self(-2);
}
impl From<i32> for SyscallErrorCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}



//
// XDR internals below this point
//


