// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 2ff2d08a54a36835f1f9a22aa693ba3205b0217e

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
use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};



// clang-format on

/// The inputs to the genesis ceremony: the genesis configuration, the
/// proposal context of the genesis block, and the transactions the genesis
/// block executes.
///
/// A coordinator reads it from the file named by `coordinator start
/// --genesis`, and runs the ceremony through its scheduler (see
/// SchedulerCoord1 in sched_coord.x) when the scheduler has no genesis
/// block yet. Every validator of a network must run the ceremony with the
/// same prior, since the genesis value it produces is the one the whole
/// network's consensus starts from.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct GenesisPrior {
    pub config: GenesisConfiguration,
    pub ctx: ProposalContext,
    pub txs: VecM<SignedTransaction>,
}


//
// XDR internals below this point
//


