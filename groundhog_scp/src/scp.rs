// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 7c6215466921ff17dc1f31705e23af0f52ce2c08

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

pub type SCPHash = [u8; 32 as usize];

pub type SCPNodeID = PublicKey;

pub type SCPCount = u32;

pub type SCPSlotIndex = u64;

pub type SCPValue = BytesM;

pub type SCPValues = VecM::<SCPValue>;

pub type SCPOptValue = Option::<SCPValue>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SCPBallot {
    // n
    pub counter: SCPCount,
    // x
    pub value: SCPValue,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SCPStatementType(pub i32);

impl SCPStatementType {
    pub const SCP_ST_PREPARE: Self = Self(0);
    pub const SCP_ST_CONFIRM: Self = Self(1);
    pub const SCP_ST_EXTERNALIZE: Self = Self(2);
    pub const SCP_ST_NOMINATE: Self = Self(3);
}
impl From<i32> for SCPStatementType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SCPStatementType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SCP_ST_PREPARE.0 => {
                write!(f, "SCPStatementType::SCP_ST_PREPARE")
            },
            x if x == Self::SCP_ST_CONFIRM.0 => {
                write!(f, "SCPStatementType::SCP_ST_CONFIRM")
            },
            x if x == Self::SCP_ST_EXTERNALIZE.0 => {
                write!(f, "SCPStatementType::SCP_ST_EXTERNALIZE")
            },
            x if x == Self::SCP_ST_NOMINATE.0 => {
                write!(f, "SCPStatementType::SCP_ST_NOMINATE")
            },
            _ => {
                write!(f, "SCPStatementType({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SCPStatementNominate {
    // D
    pub quorum_set_hash: SCPHash,
    // X
    pub votes: VecM<SCPValue>,
    // Y
    pub accepted: VecM<SCPValue>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SCPStatementPrepare {
    // D
    pub quorum_set_hash: SCPHash,
    // b
    pub ballot: SCPBallot,
    // p
    pub prepared: Option<SCPBallot>,
    // p'
    pub prepared_prime: Option<SCPBallot>,
    // c.n
    pub n_c: SCPCount,
    // h.n
    pub n_h: SCPCount,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SCPStatementConfirm {
    // b
    pub ballot: SCPBallot,
    // p.n
    pub n_prepared: SCPCount,
    // c.n
    pub n_commit: SCPCount,
    // h.n
    pub n_h: SCPCount,
    // D
    pub quorum_set_hash: SCPHash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SCPStatementExternalize {
    // c
    pub commit: SCPBallot,
    // h.n
    pub n_h: SCPCount,
    // D used before EXTERNALIZE
    pub commit_quorum_set_hash: SCPHash,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum SCPStatement_pledges {
    // case SCP_ST_PREPARE:
    //   SCPStatementPrepare prepare;
    SCP_ST_PREPARE { prepare: SCPStatementPrepare } = SCPStatementType::SCP_ST_PREPARE.0 as i32,
    // case SCP_ST_CONFIRM:
    //   SCPStatementConfirm confirm;
    SCP_ST_CONFIRM { confirm: SCPStatementConfirm } = SCPStatementType::SCP_ST_CONFIRM.0 as i32,
    // case SCP_ST_EXTERNALIZE:
    //   SCPStatementExternalize externalize;
    SCP_ST_EXTERNALIZE { externalize: SCPStatementExternalize } = SCPStatementType::SCP_ST_EXTERNALIZE.0 as i32,
    // case SCP_ST_NOMINATE:
    //   SCPStatementNominate nominate;
    SCP_ST_NOMINATE { nominate: SCPStatementNominate } = SCPStatementType::SCP_ST_NOMINATE.0 as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SCPStatement {
    // v
    pub node_id: SCPNodeID,
    // i
    pub slot_index: SCPSlotIndex,
    pub pledges: SCPStatement_pledges,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SCPEnvelope {
    pub statement: SCPStatement,
    pub signature: Signature,
}

pub type SCPEnvelopes = VecM::<SCPEnvelope>;

// supports things like: A,B,C,(D,E,F),(G,H,(I,J,K,L))
// only allows 2 levels of nesting
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SCPQuorumSet {
    pub threshold: SCPCount,
    pub validators: VecM<SCPNodeID>,
    pub inner_sets: VecM<SCPQuorumSet>,
}

pub type SCPOptQuorumSet = Option::<SCPQuorumSet>;


//
// XDR internals below this point
//

impl SCPStatement_pledges {
    pub fn stmt_type(&self) -> SCPStatementType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


