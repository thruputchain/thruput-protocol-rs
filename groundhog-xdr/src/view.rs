// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: d77f1087b6802e206902605689e2d4f92618a753

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
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TransactionViewQuery {
    // The invocation to be queried
    pub invocation: Invocation,
    // Context around the invocation
    pub parent_address: Option<ContractAddress>,
    pub auth_claim: AuthClaim,
    pub forwarded_tokens: VecM<AuthToken>,
    pub invoke_flags: InvokeFlagSet,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TransactionViewResult {
    // The result of the invocation
    pub output: InvocationFrameOutput,
    pub events: VecM<TxEmittedEvent>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct TransactionViewStatus(pub i32);

impl TransactionViewStatus {
    pub const Ok: Self = Self(0);

    // Block was abandoned (sibling block committed) during view execution.
    // View operation should be retried later.
    pub const Abandoned: Self = Self(1);

    // Internal error; should not happen.
    pub const InternalError: Self = Self(2);

    // Invalid input: i.e. reentrance guard violation.  Deterministic, but the frame does not start,
    // so no frame output is produced.
    pub const InvalidInput: Self = Self(3);

    // Validation failure: supplied nondeterministic data is somehow invalid,
    // such as requesting a queue entry that is not present.
    pub const NondeterministicDataValidationFailure: Self = Self(4);
}
impl From<i32> for TransactionViewStatus {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for TransactionViewStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::Ok.0 => {
                write!(f, "TransactionViewStatus::Ok")
            },
            x if x == Self::Abandoned.0 => {
                write!(f, "TransactionViewStatus::Abandoned")
            },
            x if x == Self::InternalError.0 => {
                write!(f, "TransactionViewStatus::InternalError")
            },
            x if x == Self::InvalidInput.0 => {
                write!(f, "TransactionViewStatus::InvalidInput")
            },
            x if x == Self::NondeterministicDataValidationFailure.0 => {
                write!(f, "TransactionViewStatus::NondeterministicDataValidationFailure")
            },
            _ => {
                write!(f, "TransactionViewStatus({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum ExpectedViewResult {
    // case Ok:
    //   TransactionViewResult result;
    Ok { result: TransactionViewResult } = TransactionViewStatus::Ok.0 as i32,
    // case InvalidInput:
    //   SyscallErrorCode error_code;
    InvalidInput { error_code: SyscallErrorCode } = TransactionViewStatus::InvalidInput.0 as i32,
    // case Abandoned:
    // case InternalError:
    // case NondeterministicDataValidationFailure:
    //   void;
    Abandoned = TransactionViewStatus::Abandoned.0 as i32,
    InternalError = TransactionViewStatus::InternalError.0 as i32,
    NondeterministicDataValidationFailure = TransactionViewStatus::NondeterministicDataValidationFailure.0 as i32,
    
}


//
// XDR internals below this point
//

impl ExpectedViewResult {
    pub fn status(&self) -> TransactionViewStatus {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


