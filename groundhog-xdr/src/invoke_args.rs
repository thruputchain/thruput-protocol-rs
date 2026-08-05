// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 2bb863900e285102907f789666495140afed7392

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



#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CopyableInvokeInputs {
    pub entrypoint_id: u64,
    pub gas_limit: GasAmount,
    pub flags: InvokeFlagSet,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "spec", derive(XDR))]
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


