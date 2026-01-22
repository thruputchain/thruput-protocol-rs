// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};
use core::fmt::{self, Debug};

#[cfg(feature="std")]
use std::io::{Read, Write};

#[cfg(feature="xdrs")]
use xdrs::*;
#[cfg(feature="spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{Serialize, Deserialize};
#[cfg(feature = "serde")]
use ::serde_arrays;


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum ExecResult {
    ExecResultOK = 0 as i32,

    // Transaction execution (during proposal or validation) failed.
    // Specifically, the root frame returned kInvokeDeterministic.
    // the error is deterministic and can be replayed in validation.
    // Transactions with this error code can be included in the block.
    ExecResultTxDeterministicError = 1 as i32,

    // The root frame of the transaction chose to revert.
    // This error is deterministic and can be included in a block.
    // However, any changes within contract state are reverted.
    ExecResultTxRootFrameReverted = 2 as i32,

    // Transaction execution (during proposal or validation) failed,
    // due to nondeterminism during the post-execution 2-phase commit.
    // Transactions with this error code should not be included in a proposal,
    // and indicate a block validation failure.
    ExecResultTxNonDeterministicError = 3 as i32,

    // One or more [GasClaim]s in the transaction were not validated with a
    // corresponding [sponsor_gas()] system call in a contract.
    ExecResultFailedGasSponsorship = 100 as i32,

    // Validation on the transaction has failed because the attached validation
    // data is bad. E.g. missing data or unused data. This error code is only
    // returned during validation. This is determininistic, but signals a bad
    // block from a malicious proposer.
    ExecResultTxValidationFailure = 200 as i32,

    // System error; network connection broke, etc.
    // Executor can recover from this error to continue execution of other
    // transactions.
    ExecResultSystemError = 300 as i32,

    // Critical failure in the executor, which cannot be recovered from.
    // E.g. OOM or critical logic error.
    ExecResultUnrecoverable = 400 as i32,
}
impl TryFrom<i32> for ExecResult {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ExecResultOK),
            1 => Ok(Self::ExecResultTxDeterministicError),
            2 => Ok(Self::ExecResultTxRootFrameReverted),
            3 => Ok(Self::ExecResultTxNonDeterministicError),
            100 => Ok(Self::ExecResultFailedGasSponsorship),
            200 => Ok(Self::ExecResultTxValidationFailure),
            300 => Ok(Self::ExecResultSystemError),
            400 => Ok(Self::ExecResultUnrecoverable),
            _ => Err(()),
        }
    }
}



//
// XDR internals below this point
//


