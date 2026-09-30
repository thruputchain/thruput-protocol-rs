// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 667c466d8f5daf4d16a68920704d5ae7fc1660b0

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
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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

    // No longer used. Previously indicated that gas sponsorship claims were
    // not validated. Retained to avoid renumbering enum values.
    pub const ExecResultFailedGasSponsorship: Self = Self(100);

    // Validation on the transaction has failed because the attached validation
    // data is bad. E.g. missing data or unused data. This error code is only
    // returned during validation. This is determininistic, but signals a bad
    // block from a malicious proposer.
    pub const ExecResultTxValidationFailure: Self = Self(200);

    // The transaction is malformed: a structural check rejected it before
    // execution, on its contents alone and independent of ledger state.
    // This error code is only returned during proposal, where the scheduler
    // drops the transaction instead of including it in the block. The same
    // defect during validation is ExecResultTxValidationFailure, since a
    // malformed transaction inside a proposed block is the proposer's fault.
    pub const ExecResultMalformedTransaction: Self = Self(201);

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
#[cfg(feature = "std")]
impl core::fmt::Display for ExecResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::ExecResultOK.0 => {
                write!(f, "ExecResult::ExecResultOK")
            },
            x if x == Self::ExecResultTxDeterministicError.0 => {
                write!(f, "ExecResult::ExecResultTxDeterministicError")
            },
            x if x == Self::ExecResultTxRootFrameReverted.0 => {
                write!(f, "ExecResult::ExecResultTxRootFrameReverted")
            },
            x if x == Self::ExecResultTxNonDeterministicError.0 => {
                write!(f, "ExecResult::ExecResultTxNonDeterministicError")
            },
            x if x == Self::ExecResultFailedGasSponsorship.0 => {
                write!(f, "ExecResult::ExecResultFailedGasSponsorship")
            },
            x if x == Self::ExecResultTxValidationFailure.0 => {
                write!(f, "ExecResult::ExecResultTxValidationFailure")
            },
            x if x == Self::ExecResultMalformedTransaction.0 => {
                write!(f, "ExecResult::ExecResultMalformedTransaction")
            },
            x if x == Self::ExecResultSystemError.0 => {
                write!(f, "ExecResult::ExecResultSystemError")
            },
            x if x == Self::ExecResultUnrecoverable.0 => {
                write!(f, "ExecResult::ExecResultUnrecoverable")
            },
            _ => {
                write!(f, "ExecResult({})", self.0)
            }
        }
    }
}



//
// XDR internals below this point
//


