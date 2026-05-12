// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: bca4204390034cf31ecbfd4ca04619eab347032d

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::ops::{Deref, DerefMut};
use core::option::Option;

#[cfg(feature = "std")]
use std::io::{Read, Write};
use xdrs::*;

#[cfg(feature = "serde")]
use ::serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use ::serde_arrays;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]
#[derive(
    schemars::JsonSchema,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Copy,
)]
#[repr(transparent)]
pub struct MultiplexMessageType(pub i32);

impl MultiplexMessageType {
    pub const MultiplexMessageInvalid: Self = Self(0);
    pub const MultiplexMessageSubmittedTxs: Self = Self(1);
    pub const MultiplexMessageConsensusValue: Self = Self(2);
    pub const MultiplexMessageSCP: Self = Self(3);
}
impl From<i32> for MultiplexMessageType {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for MultiplexMessageType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::MultiplexMessageInvalid.0 => {
                write!(f, "MultiplexMessageType::MultiplexMessageInvalid")
            }
            x if x == Self::MultiplexMessageSubmittedTxs.0 => {
                write!(f, "MultiplexMessageType::MultiplexMessageSubmittedTxs")
            }
            x if x == Self::MultiplexMessageConsensusValue.0 => {
                write!(
                    f,
                    "MultiplexMessageType::MultiplexMessageConsensusValue"
                )
            }
            x if x == Self::MultiplexMessageSCP.0 => {
                write!(f, "MultiplexMessageType::MultiplexMessageSCP")
            }
            _ => {
                write!(f, "MultiplexMessageType({})", self.0)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum MultiplexMessage {
    // case MultiplexMessageSubmittedTxs:
    //   SubmittedTransactions submitted;
    MultiplexMessageSubmittedTxs {
        submitted: SubmittedTransactions,
    } = MultiplexMessageType::MultiplexMessageSubmittedTxs.0 as i32,
    // case MultiplexMessageConsensusValue:
    //   ConsensusValue consensus_value;
    MultiplexMessageConsensusValue {
        consensus_value: ConsensusValue,
    } = MultiplexMessageType::MultiplexMessageConsensusValue.0 as i32,
    // case MultiplexMessageSCP:
    //   SCPEnvelope envelope;
    MultiplexMessageSCP {
        envelope: SCPEnvelope,
    } = MultiplexMessageType::MultiplexMessageSCP.0 as i32,
    // default:
    //   void;
    Default {
        r#type: i32,
    },
}

//
// XDR internals below this point
//

impl MultiplexMessage {
    pub fn type_(&self) -> MultiplexMessageType {
        match self {
            MultiplexMessage::Default { r#type } => (*r#type).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}
