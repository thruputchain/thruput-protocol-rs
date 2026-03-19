// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0b35b4bc56dc32f40c6de155f35b76f18a1da5f2

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
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ExecResult(pub i32);

impl ExecResult {
    pub const ExecResultOK: Self = Self(0);

    // Transaction execution (during proposal or validation) failed.
    // Specifically, the root frame returned Deterministic.
    // the error is deterministic and can be replayed in validation.
    // Transactions with this error code can be included in the block.
    pub const ExecResultTxDeterministicError: Self = Self(1);

    // The root frame of the transaction chose to revert.
    // This error is deterministic and can be included in a block.
    // However, any changes within contract state are reverted.
    pub const ExecResultTxRootFrameReverted: Self = Self(2);

    // Transaction execution (during proposal or validation) failed,
    // due to nondeterminism during the post-execution 2-phase commit.
    // Transactions with this error code should not be included in a proposal,
    // and indicate a block validation failure.
    pub const ExecResultTxNonDeterministicError: Self = Self(3);

    // One or more [GasClaim]s in the transaction were not validated with a
    // corresponding [sponsor_gas()] system call in a contract.
    pub const ExecResultFailedGasSponsorship: Self = Self(100);

    // Validation on the transaction has failed because the attached validation
    // data is bad. E.g. missing data or unused data. This error code is only
    // returned during validation. This is determininistic, but signals a bad
    // block from a malicious proposer.
    pub const ExecResultTxValidationFailure: Self = Self(200);

    // System error; network connection broke, etc.
    // Executor can recover from this error to continue execution of other
    // transactions.
    pub const ExecResultSystemError: Self = Self(300);

    // Critical failure in the executor, which cannot be recovered from.
    // E.g. OOM or critical logic error.
    pub const ExecResultUnrecoverable: Self = Self(400);
}
impl From<i32> for ExecResult {
    fn from(v: i32) -> Self {
        Self(v)
    }
}



//
// XDR internals below this point
//


