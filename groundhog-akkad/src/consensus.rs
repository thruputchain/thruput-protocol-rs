// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: c0e2b46d8abfb48cd8fb90d8f06af8d2a99acf27

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

// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct ConsensusBasisHash {
    inner: SHA256Hash,
}
impl From<SHA256Hash> for ConsensusBasisHash {
    fn from(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}
impl ConsensusBasisHash {
    pub fn into_inner(self) -> SHA256Hash {
        self.inner
    }
    pub const fn new(_inner: SHA256Hash) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for ConsensusBasisHash {
    type Target = SHA256Hash;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for ConsensusBasisHash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConsensusBasis {
    pub timestamp: TimePoint,
    pub proposer: ValidatorID,
    pub nomination: Nomination,
    pub upgrades: VecM<Upgrade, 6>,
    pub prev_value_hash: ConsensusValueHash,
    pub prev_basis_hash: ConsensusBasisHash,
}

pub type OptConsensusBasis = Option::<ConsensusBasis>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ConsensusValue {
    pub timestamp: TimePoint,
    pub proposer: ValidatorID,
    pub upgrades: VecM<Upgrade, 6>,
    pub block_height: BlockHeight,
    pub prev_value_hash: ConsensusValueHash,
    pub basis_hash: ConsensusBasisHash,
    pub prev_basis_hash: ConsensusBasisHash,
    pub block_hash: BlockHash,
    pub prev_block_hash: BlockHash,
}


//
// XDR internals below this point
//


