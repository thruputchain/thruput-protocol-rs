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
use xdrs::*;



#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

pub type Upgrade = BytesM::<128>;

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ConsensusValueHash {
    inner: SHA256Hash,
}
impl From<SHA256Hash> for ConsensusValueHash {
    fn from(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}
impl ConsensusValueHash {
    pub fn into_inner(self) -> SHA256Hash {
        self.inner
    }
    pub const fn new(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ConsensusValueHash {
    type Target = SHA256Hash;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ConsensusValueHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConsensusValue {
    pub timestamp: TimePoint,
    pub nomination: Nomination,
    pub upgrades: VecM<Upgrade, 6>,
    pub prev_cv_hash: ConsensusValueHash,
}

pub type OptConsensusValue = Option::<ConsensusValue>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConsensusValueCue {
    pub proposer: ValidatorPublicInfo,
    pub cv_hash: ConsensusValueHash,
    pub prev_cv_hash: ConsensusValueHash,
    pub block_height: BlockHeight,
    pub block_hash: BlockHash,
    pub prev_block_hash: BlockHash,
}


//
// XDR internals below this point
//


