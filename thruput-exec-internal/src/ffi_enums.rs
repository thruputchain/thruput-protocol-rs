// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: d205b2a2427fe9af85bb4fcdcf7e85ac770d5961

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


// Error codes returned from a runtime (wasmi) invocation.
// This is the external interface to a smart contract. Normally wrapped in an
// InvokeStatus<int32_t> (a std::expected).
//
// When InvokeStatus<int32_t> has an int32_t value (and an
// InvokeErrorCode::None), the int32_t value represents a handleable
// (deterministic) result from the invocation. (Specifically, the invoke()
// syscall in a nested smart contract invocation passes an
// {InvokeErrorCode::None, code} back as a
// {HostFnErrorCode::NoneOrRecoverable, code}).
//
// This value is generated as either the returned value from the selected
// WebAssembly exported function that was run, the code caused by a host
// function returning HostFnErrorCode::Exit, or 0, if invocation ends because a
// host function returns HostFnErrorCode::Return.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct InvokeErrorCode(pub i32);

impl InvokeErrorCode {
    // A call frame can only succeed if this code is returned.
    // All others revert the call frame.
    pub const None: Self = Self(0);

    // An unspecified deterministic error.
    pub const Deterministic: Self = Self(1);

    // Runtime exceeded its gas budget.
    pub const OutOfGas: Self = Self(2);

    // Nondeterministic error during invocation (e.g. malloc error)
    // Signals a serious system issue or a serious bug in our code and should be
    // diagnosed ASAP.
    pub const Unrecoverable: Self = Self(3);

    // Discovered during validation that proposer submitted bad data.
    // If a block contains a transaction with this error, the block does not need
    // to be discarded.  The transaction that causes this error is discarded,
    // and the block proposer should be penalized (via whatever out of band
    // mechanism),
    // but the rest of the block can still be processed.
    // Note that this is different from the case of a block proposal with
    // conflicting transactions;
    // those errors are discovered _after_ executing the transactions in a block,
    // and require
    // discarding the entire block.
    pub const ValidationFailure: Self = Self(4);

    // The transaction was aborted due to an external reason (such as the current
    // block being discarded),
    // but this is not the result of a totally unrecoverable error. This is to be
    // expected during normal system operation.
    pub const Abandon: Self = Self(5);
}
impl From<i32> for InvokeErrorCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for InvokeErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::None.0 => {
                write!(f, "InvokeErrorCode::None")
            },
            x if x == Self::Deterministic.0 => {
                write!(f, "InvokeErrorCode::Deterministic")
            },
            x if x == Self::OutOfGas.0 => {
                write!(f, "InvokeErrorCode::OutOfGas")
            },
            x if x == Self::Unrecoverable.0 => {
                write!(f, "InvokeErrorCode::Unrecoverable")
            },
            x if x == Self::ValidationFailure.0 => {
                write!(f, "InvokeErrorCode::ValidationFailure")
            },
            x if x == Self::Abandon.0 => {
                write!(f, "InvokeErrorCode::Abandon")
            },
            _ => {
                write!(f, "InvokeErrorCode({})", self.0)
            }
        }
    }
}


// HostFunctionErrorCode categorizes errors returned from system calls. The
// normal usage pattern is to wrap them within HostFnStatus<int32_t>, where the
// int32 is the more detailed error code.
//
// In WebAssembly, a host function is defined as a function that can be
// called outside of WebAssembly. We import one of these functions to provide
// access to our table of system call handlers.
//
// When a HostFnStatus<int32_t> (a wrapper around std::expected) has an int32_t
// value, (and a HostFnErrorCode::NoneOrRecoverable), that int32_t is passed to
// WebAssembly as a handleable error code, as the return result from the host
// function. The WebAssembly bytecode (the smart contract) itself never sees the
// HostFnErrorCode. Other HostFnErrorCodes terminate the WebAssembly runtime
// (returning to the caller), and are handled by the WebAssembly runtime.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct HostFnErrorCode(pub i32);

impl HostFnErrorCode {
    pub const NoneOrRecoverable: Self = Self(0);

    // Insufficient gas to continue WebAssembly execution.
    //
    // Result: the WebAssembly runtime immediately returns an
    // InvokeErrorCode::OutOfGas.
    pub const OutOfGas: Self = Self(1);

    // Unrecoverable (nondeterministic) error, such as a malloc error.
    // The only course of action is to abort the entire transaction.
    //
    // Result: the WebAssembly runtime immediately returns an
    // InvokeErrorCode::Unrecoverable
    // Signals a serious system issue or a serious bug in our code and should be
    // diagnosed ASAP.
    pub const Unrecoverable: Self = Self(2);

    // Return from a WebAssembly invocation to the caller.  Registers a byte
    // buffer to be passed back to the caller.
    //
    // Result: Runtime immediately returns an {InvokeErrorCode::None, 0}.
    pub const Return: Self = Self(3);

    // Return from a WebAssembly invocation to the caller, with some (int32_t)
    // error code.
    //
    // Result: Runtime immediately returns an {InvokeErrorCode::None, code}.
    //
    // On a root smart contract invocation, a nonzero code is converted to a
    // InvokeErrorCode::Deterministic.
    pub const Exit: Self = Self(4);

    // Validation fails because a proposer submitted bad nondeterministic
    // proposal data.
    //
    // Result: the WebAssembly runtime immediately returns an
    // InvokeErrorCode::ValidationFailure
    pub const ValidationFailure: Self = Self(5);

    // The transaction was aborted due to an external reason (such as the current
    // block being discarded), but this is not the result of a totally
    // unrecoverable error. This is to be expected during normal system operation.
    //
    // Result: the WebAssembly runtime immediately returns an
    // InvokeErrorCode::Abandon
    pub const Abandon: Self = Self(6);
}
impl From<i32> for HostFnErrorCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for HostFnErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::NoneOrRecoverable.0 => {
                write!(f, "HostFnErrorCode::NoneOrRecoverable")
            },
            x if x == Self::OutOfGas.0 => {
                write!(f, "HostFnErrorCode::OutOfGas")
            },
            x if x == Self::Unrecoverable.0 => {
                write!(f, "HostFnErrorCode::Unrecoverable")
            },
            x if x == Self::Return.0 => {
                write!(f, "HostFnErrorCode::Return")
            },
            x if x == Self::Exit.0 => {
                write!(f, "HostFnErrorCode::Exit")
            },
            x if x == Self::ValidationFailure.0 => {
                write!(f, "HostFnErrorCode::ValidationFailure")
            },
            x if x == Self::Abandon.0 => {
                write!(f, "HostFnErrorCode::Abandon")
            },
            _ => {
                write!(f, "HostFnErrorCode({})", self.0)
            }
        }
    }
}


// General-purpose system error codes returned from within various executor operations.
// These can be mapped into either InvokeErrorCode or HostFnErrorCode, depending on the context in which the
// error occurs.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SystemErrorCode(pub i32);

impl SystemErrorCode {
    // Unrecoverable error (e.g. out of memory, or critical logic error).
    // Only recourse is to abort entire transaction and possibly crash binary.
    // Signals serious error, which we should investigate.
    // This converts to InvokeErrorCode::Unrecoverable or HostFnErrorCode::Unrecoverable.
    pub const Unrecoverable: Self = Self(1);

    // Nondeterministic error that occurs during normal course of operation.
    // E.g. a transaction is being executed concurrently with the whole block
    // getting cancelled.
    // Only recourse is to abort the entire transaction.
    // This converts to InvokeErrorCode::Abandon or HostFnErrorCode::Abandon.
    pub const Abandon: Self = Self(2);

    // Failure during validation of a transaction.  Signals that the whole
    // block must be aborted.
    // This converts to InvokeErrorCode::ValidationFailure or HostFnErrorCode::ValidationFailure.
    pub const ValidationFailure: Self = Self(3);
}
impl From<i32> for SystemErrorCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SystemErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::Unrecoverable.0 => {
                write!(f, "SystemErrorCode::Unrecoverable")
            },
            x if x == Self::Abandon.0 => {
                write!(f, "SystemErrorCode::Abandon")
            },
            x if x == Self::ValidationFailure.0 => {
                write!(f, "SystemErrorCode::ValidationFailure")
            },
            _ => {
                write!(f, "SystemErrorCode({})", self.0)
            }
        }
    }
}



//
// XDR internals below this point
//


