// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 011eecf37b9bb7cf7f9b0e6ae267ec8935a484b3

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

#[cfg(feature="std")]
use std::io::{Read, Write};

#[cfg(feature="xdrs")]
use xdrs::*;
#[cfg(feature="spec")]
use xdrs::xdr_repr::*;



#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum ExecResult {
    ExecResultOK = 0 as i32,

    // Transaction execution (during proposal or validation) failed;
    // the error is deterministic and can be replayed in validation.
    // Transactions with this error code can be included in the block.
    ExecResultTxDeterministicError = 1 as i32,

    // Transaction execution (during proposal or validation) failed,
    // due to nondeterminism during the post-execution 2-phase commit.
    // Transactions with this error code should not be included in a proposal,
    // and indicate a block validation failure.
    ExecResultTxNonDeterministicError = 2 as i32,

    // One or more [GasClaim]s in the transaction were not validated with a
    // corresponding [sponsor_gas()] system call in a contract.
    ExecResultFailedGasSponsorship = 3 as i32,

    // Validation on the transaction has failed because the attached validation
    // data is bad. E.g. missing data or unused data. This error code is only
    // returned during validation. This is determininistic, but signals a bad
    // block from a malicious proposer.
    ExecResultTxValidationFailure = 4 as i32,

    // System error; network connection broke, etc.
    // Executor can recover from this error to continue execution of other
    // transactions.
    ExecResultSystemError = 5 as i32,

    // Critical failure in the executor, which cannot be recovered from.
    // E.g. OOM or critical logic error.
    ExecResultUnrecoverable = 6 as i32,
}
pub use ExecResult::*;

impl TryFrom<i32> for ExecResult {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ExecResultOK),
            1 => Ok(Self::ExecResultTxDeterministicError),
            2 => Ok(Self::ExecResultTxNonDeterministicError),
            3 => Ok(Self::ExecResultFailedGasSponsorship),
            4 => Ok(Self::ExecResultTxValidationFailure),
            5 => Ok(Self::ExecResultSystemError),
            6 => Ok(Self::ExecResultUnrecoverable),
            _ => Err(()),
        }
    }
}



//
// XDR internals below this point
//


