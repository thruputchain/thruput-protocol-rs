// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: c385cdc8b9f636baa910ab0814dbbb7f0ff55057

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



// clang-format on

// triple slash comments are added to the generated rust docs

/// Error codes returned by system calls.
/// Each syscall can return a specific subset of these error codes.
/// Check the syscalls.yaml documentation for which errors each syscall can return.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SyscallErrorCode(pub i32);

impl SyscallErrorCode {
    /// No error
    pub const kSyscallErrorCodeNone: Self = Self(0);

    /// Out of bounds pointer to contract memory passed to syscall
    pub const kSyscallErrorCodeFault: Self = Self(1);

    /// System call for supplied number is not supported
    pub const kSyscallErrorCodeNoSyscall: Self = Self(2);

    /// Cannot call get_msg_sender on root invocation
    pub const kSyscallErrorCodeIsRootTransaction: Self = Self(3);

    /// Reentrance guard violation
    pub const kSyscallErrorCodeReentrance: Self = Self(4);

    /// Attempt made to deploy a contract twice (same nonce and contract hash, from
    /// same address)
    pub const kSyscallErrorCodeContractAlreadyDeployed: Self = Self(5);

    /// No return buffer is available.  Either the most recent subcall faulted, or
    /// there have been no subcalls within this call frame.
    pub const kSyscallErrorCodeNoReturnBufferAvailable: Self = Self(6);

    /// Invalid input to a syscall, such as subtracting below 0 on an nnint
    /// or a bytestring that is too long
    pub const kSyscallErrorCodeInvalidSyscallInput: Self = Self(7);

    /// Accessing a storage key that does not exist in the current view
    pub const kSyscallErrorCodeStorageKeyNotExist: Self = Self(8);

    /// No entry in the retrocausal queue to consume
    pub const kSyscallErrorCodeNoRetrocausalQueueEntry: Self = Self(9);

    /// The transaction has reached its per-transaction limit on concurrent
    /// retrocausal-queue reads (kMaxConcurrentRcqReadsPerTx). This is a resource
    /// limit, not a malformed call, and does not by itself fail the transaction;
    /// a contract may use it as a stop condition (e.g. drain a queue until it
    /// returns this).
    pub const kSyscallErrorCodeRetrocausalQueueReadLimitReached: Self = Self(10);

    /// The invocation would exceed the network's cross-invoke depth bound
    /// (kConfigParamMaxCrossInvokeDepth), which limits how many invocation
    /// frames one transaction may hold open at once. Like the retrocausal-queue
    /// read limit this is a resource limit, not a malformed call: the same
    /// invocation succeeds from a shallower frame. It is reported before any
    /// child frame runs, so the syscall is a complete no-op and the caller's
    /// invoke outputs are left untouched.
    pub const kSyscallErrorCodeInvokeDepthExceeded: Self = Self(11);
}
impl From<i32> for SyscallErrorCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SyscallErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kSyscallErrorCodeNone.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeNone")
            },
            x if x == Self::kSyscallErrorCodeFault.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeFault")
            },
            x if x == Self::kSyscallErrorCodeNoSyscall.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeNoSyscall")
            },
            x if x == Self::kSyscallErrorCodeIsRootTransaction.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeIsRootTransaction")
            },
            x if x == Self::kSyscallErrorCodeReentrance.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeReentrance")
            },
            x if x == Self::kSyscallErrorCodeContractAlreadyDeployed.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeContractAlreadyDeployed")
            },
            x if x == Self::kSyscallErrorCodeNoReturnBufferAvailable.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable")
            },
            x if x == Self::kSyscallErrorCodeInvalidSyscallInput.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput")
            },
            x if x == Self::kSyscallErrorCodeStorageKeyNotExist.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist")
            },
            x if x == Self::kSyscallErrorCodeNoRetrocausalQueueEntry.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeNoRetrocausalQueueEntry")
            },
            x if x == Self::kSyscallErrorCodeRetrocausalQueueReadLimitReached.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeRetrocausalQueueReadLimitReached")
            },
            x if x == Self::kSyscallErrorCodeInvokeDepthExceeded.0 => {
                write!(f, "SyscallErrorCode::kSyscallErrorCodeInvokeDepthExceeded")
            },
            _ => {
                write!(f, "SyscallErrorCode({})", self.0)
            }
        }
    }
}



//
// XDR internals below this point
//


