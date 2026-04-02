// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: bab641a16d398d48b16086772f51d186ac0522cb

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


/// Flags that modify the behavior of a contract invocation.
///
/// These flags are used with the Invoke syscall to control how a subinvocation
/// executes. They are combined using bitwise OR.
///
/// Note: InvokeFlagSet is NOT part of the Invocation struct itself, but is
/// instead a parameter to the Invoke syscall. The root transaction invocation
/// always has no flags set (equivalent to None).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct InvokeFlag(pub i32);

impl InvokeFlag {
    /// No special behavior (default).
    pub const None: Self = Self(0);

    /// Enable early conflict detection for parallel execution.
    ///
    /// Groundhog executes transactions in parallel without observing other
    /// transactions' writes. After execution, a two-phase commit process commits writes
    /// to the database. If there's a conflict, the entire transaction aborts.
    ///
    /// TryInvoke pulls this conflict detection earlier: when a try_invoke frame
    /// finishes, it performs two-phase commit lock acquisition immediately. If any lock fails,
    /// the subinvocation frame reverts (but the caller can handle this gracefully).
    ///
    /// This is nondeterministic, so the two-phase commit result (a boolean) is appended to
    /// the nondeterministic_bits in [`NondeterministicProposalData`] for deterministic
    /// re-execution by validators.
    pub const TryInvoke: Self = Self(0x00000001);

    /// Prevent all state modifications in the subinvocation.
    ///
    /// The subinvocation can read storage but cannot write, deploy contracts,
    /// or perform any other state-modifying operations.
    pub const ReadOnly: Self = Self(0x00000002);

    /// Execute the target contract's code in the caller's context.
    ///
    /// Similar to Ethereum's DELEGATECALL: the invoked code runs with the
    /// caller's storage and address context, allowing library-like code reuse.
    ///
    /// TODO(geoff): This flag is not yet implemented. When implementing, provide
    /// a comprehensive specification of what changes and what doesn't change in
    /// a delegatecall frame. For example, GetMsgSender and GetCurrentAddress
    /// should NOT change (they return the original caller and current address),
    /// but storage operations should operate on the caller's storage. Document
    /// the behavior of all syscalls in a delegatecall context.
    pub const DelegateCall: Self = Self(0x00000004);
}
impl From<i32> for InvokeFlag {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for InvokeFlag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::None.0 => {
                write!(f, "InvokeFlag::None")
            },
            x if x == Self::TryInvoke.0 => {
                write!(f, "InvokeFlag::TryInvoke")
            },
            x if x == Self::ReadOnly.0 => {
                write!(f, "InvokeFlag::ReadOnly")
            },
            x if x == Self::DelegateCall.0 => {
                write!(f, "InvokeFlag::DelegateCall")
            },
            _ => {
                write!(f, "InvokeFlag({})", self.0)
            }
        }
    }
}


/// A bitset of InvokeFlag values.
///
/// Multiple flags can be combined using bitwise OR. For example:
/// `ReadOnly | TryInvoke`
pub type InvokeFlagSet = i32;


//
// XDR internals below this point
//


