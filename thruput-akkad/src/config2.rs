// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0c2c59fb8ebeb0d6d37a1dad9cf88c4f22187ce6

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

/// The router of a coordinator's boundary, which chooses the dissemination
/// trees over the validators the boundary knows.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CoordinatorRouterConfig {
    /// The largest number of children of a node of a dissemination tree.
    pub max_fanout: u64,
}

/// The baton, which drives a coordinator's proposals and validations
/// through its scheduler.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CoordinatorBatonConfig {
    /// The expected maximum local clock skew between validators.
    pub clock_skew_msec: Milliseconds,
}

/// The consensus protocol a coordinator's broker runs.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CoordinatorConsensusConfig {
    /// The number of slots in an epoch: the leader elected for an epoch
    /// proposes for all of its slots. Must be positive.
    pub epoch_length: u64,
    /// The pipeline width: how many slots ahead of the current slot the
    /// leader may propose. Must be positive.
    pub pipeline_length: u64,
    /// The time a slot is given to resolve before it is deemed late.
    pub timeout_msec: Milliseconds,
    /// How early each epoch's leader election starts: the election for epoch
    /// e starts once the first slot of epoch e - election_lookahead is
    /// confirmed (or its timer expires). 1 starts it while the previous epoch
    /// runs; larger values give the election vote more time, from an older
    /// view of which parties are live. Must be positive.
    pub election_lookahead: u64,
}

/// The basis and block pools of a coordinator. Each keeps the values of
/// the latest heights in memory and evicts older ones to disk after a
/// commit.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CoordinatorPoolConfig {
    /// How many heights below the committed tip the basis pool keeps in
    /// memory.
    pub basis_pool_hotcold_window: u64,
    /// How many heights below the committed tip the block pool keeps in
    /// memory.
    pub block_pool_hotcold_window: u64,
}

/// The configuration of a coordinator: what its command line reads from
/// the file named by `coordinator start --config`.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CoordinatorConfig {
    pub router: CoordinatorRouterConfig,
    pub dogwood: DogwoodConfig,
    pub baton: CoordinatorBatonConfig,
    pub consensus: CoordinatorConsensusConfig,
    pub pool: CoordinatorPoolConfig,
}


//
// XDR internals below this point
//


