// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: c385cdc8b9f636baa910ab0814dbbb7f0ff55057

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

pub type BristleconeQuorumSetHash = [u8; 32 as usize];

pub type BristleconeCount = u32;

// A slot number (aka height). Voting starts at slot 0. Slot -1 is the
// implicit genesis slot: it is never voted on, but slot 0 names it as its
// parent, so slots are signed.
pub type BristleconeSlot = i64;

pub type BristleconeValue = BytesM;

pub type BristleconeValues = VecM::<BristleconeValue>;

pub type BristleconeOptValue = Option::<BristleconeValue>;

// A leader proposal for the note's slot: the proposed value and the slot
// whose value it hash-chains onto.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeProposal {
    pub parent_slot: BristleconeSlot,
    pub value: BristleconeValue,
}

// The statement of a SAFE vote: the note's slot holds this value, with this
// parent slot.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeSafeStatement {
    pub parent_slot: BristleconeSlot,
    pub value: BristleconeValue,
}

// The statement of a TIMELY vote: whether the note's slot is timely.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeTimelyStatement {
    pub timely: bool,
}

// The federated-voting phase of a vote: the sender votes for the statement,
// or accepts it.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct BristleconeVoteType(pub i32);

impl BristleconeVoteType {
    pub const BRISTLECONE_VOTE: Self = Self(0);
    pub const BRISTLECONE_ACCEPT: Self = Self(1);
}
impl From<i32> for BristleconeVoteType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for BristleconeVoteType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::BRISTLECONE_VOTE.0 => {
                write!(f, "BristleconeVoteType::BRISTLECONE_VOTE")
            },
            x if x == Self::BRISTLECONE_ACCEPT.0 => {
                write!(f, "BristleconeVoteType::BRISTLECONE_ACCEPT")
            },
            _ => {
                write!(f, "BristleconeVoteType({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeSafeVote {
    // The hash of the sender's quorum set (BristleconeQuorumSet) for this vote.
    pub qset_hash: BristleconeQuorumSetHash,
    pub statement: BristleconeSafeStatement,
    pub vote: BristleconeVoteType,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeTimelyVote {
    // The hash of the sender's quorum set (BristleconeQuorumSet) for this vote.
    pub qset_hash: BristleconeQuorumSetHash,
    pub statement: BristleconeTimelyStatement,
    pub vote: BristleconeVoteType,
}

// The three kinds of envelope: a proposal, a SAFE vote, and a TIMELY vote.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct BristleconeContentType(pub i32);

impl BristleconeContentType {
    pub const BRISTLECONE_PROPOSAL: Self = Self(0);
    pub const BRISTLECONE_SAFE_VOTE: Self = Self(1);
    pub const BRISTLECONE_TIMELY_VOTE: Self = Self(2);
}
impl From<i32> for BristleconeContentType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for BristleconeContentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::BRISTLECONE_PROPOSAL.0 => {
                write!(f, "BristleconeContentType::BRISTLECONE_PROPOSAL")
            },
            x if x == Self::BRISTLECONE_SAFE_VOTE.0 => {
                write!(f, "BristleconeContentType::BRISTLECONE_SAFE_VOTE")
            },
            x if x == Self::BRISTLECONE_TIMELY_VOTE.0 => {
                write!(f, "BristleconeContentType::BRISTLECONE_TIMELY_VOTE")
            },
            _ => {
                write!(f, "BristleconeContentType({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum BristleconeContent {
    // case BRISTLECONE_PROPOSAL:
    //   BristleconeProposal proposal;
    BRISTLECONE_PROPOSAL { proposal: BristleconeProposal } = BristleconeContentType::BRISTLECONE_PROPOSAL.0 as i32,
    // case BRISTLECONE_SAFE_VOTE:
    //   BristleconeSafeVote safe_vote;
    BRISTLECONE_SAFE_VOTE { safe_vote: BristleconeSafeVote } = BristleconeContentType::BRISTLECONE_SAFE_VOTE.0 as i32,
    // case BRISTLECONE_TIMELY_VOTE:
    //   BristleconeTimelyVote timely_vote;
    BRISTLECONE_TIMELY_VOTE { timely_vote: BristleconeTimelyVote } = BristleconeContentType::BRISTLECONE_TIMELY_VOTE.0 as i32,
    
}

// The signed payload of an envelope.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeNote {
    // The sending party.
    pub party: ValidatorID,
    // The slot the content applies to.
    pub slot: BristleconeSlot,
    pub content: BristleconeContent,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeEnvelope {
    pub note: BristleconeNote,
    // The sending party's signature over the XDR encoding of the note.
    pub signature: Signature,
}

pub type BristleconeEnvelopes = VecM::<BristleconeEnvelope>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct BristleconeQuorumSet {
    pub threshold: BristleconeCount,
    pub parties: VecM<ValidatorID>,
    pub inner_sets: VecM<BristleconeQuorumSet>,
}

pub type BristleconeOptQuorumSet = Option::<BristleconeQuorumSet>;


//
// XDR internals below this point
//

impl BristleconeContent {
    pub fn content_type(&self) -> BristleconeContentType {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


