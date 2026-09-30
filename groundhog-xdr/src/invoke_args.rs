// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: dcb5d7af8f39be558b0bfe56c0d0bb9163cce9f2

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



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CopyableInvokeInputs {
    pub entrypoint_id: u64,
    pub gas_limit: GasAmount,
    pub flags: InvokeFlagSet,
    // Explicit tail padding so the aggregate has no implementation-defined
    // padding bytes. The RV32 syscall ABI decodes this struct by range-checking
    // and copying sizeof() bytes of guest memory, which is only a
    // host-layout-independent decode when every byte is a named field. Naming
    // the tail keeps std::has_unique_object_representations_v true, which the
    // by-reference syscall input path in hogd static_asserts. Ignored by the
    // host; not part of the invoke semantics (issue #3854).
    pub reserved: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CopyableInvokeOutputs {
    pub return_buffer_len: RuntimeMemoryLength,
    pub returned_auth_token_len: RuntimeMemoryLength,
    // The outcome of the child frame. Only meaningful when the Invoke syscall
    // returns kSyscallErrorCodeNone (i.e. a child frame actually ran):
    //   Return -> child returned normally; return_buffer_len bytes available
    //   Abort  -> child manually reverted; abort buffer available
    //   Fault  -> child trapped / ran out of gas; no return buffer
    // Left untouched when the syscall returns a nonzero error code (the call
    // was rejected before any child frame ran, and is a complete no-op).
    pub frame_status: InvocationFrameStatusCode,
}


//
// XDR internals below this point
//


