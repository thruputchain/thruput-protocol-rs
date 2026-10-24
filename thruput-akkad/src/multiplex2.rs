// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: d205b2a2427fe9af85bb4fcdcf7e85ac770d5961

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


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct MultiplexMessageType(pub i32);

impl MultiplexMessageType {
    pub const MultiplexMessageInvalid: Self = Self(0);
    pub const MultiplexMessageSubmittedTxs: Self = Self(1);
    pub const MultiplexMessageConsensusBasis: Self = Self(2);
    pub const MultiplexMessageEnvelope: Self = Self(3);
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
            },
            x if x == Self::MultiplexMessageSubmittedTxs.0 => {
                write!(f, "MultiplexMessageType::MultiplexMessageSubmittedTxs")
            },
            x if x == Self::MultiplexMessageConsensusBasis.0 => {
                write!(f, "MultiplexMessageType::MultiplexMessageConsensusBasis")
            },
            x if x == Self::MultiplexMessageEnvelope.0 => {
                write!(f, "MultiplexMessageType::MultiplexMessageEnvelope")
            },
            _ => {
                write!(f, "MultiplexMessageType({})", self.0)
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
pub enum MultiplexMessage {
    // case MultiplexMessageSubmittedTxs:
    //   SubmittedTransactions submitted;
    MultiplexMessageSubmittedTxs { submitted: SubmittedTransactions } = MultiplexMessageType::MultiplexMessageSubmittedTxs.0 as i32,
    // case MultiplexMessageConsensusBasis:
    //   ConsensusBasis basis;
    MultiplexMessageConsensusBasis { basis: ConsensusBasis } = MultiplexMessageType::MultiplexMessageConsensusBasis.0 as i32,
    // case MultiplexMessageEnvelope:
    //   BristleconeEnvelope envelope;
    MultiplexMessageEnvelope { envelope: BristleconeEnvelope } = MultiplexMessageType::MultiplexMessageEnvelope.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { r#type: i32 },
}


//
// XDR internals below this point
//

impl MultiplexMessage {
    #[allow(clippy::match_single_binding)]
    pub fn r#type(&self) -> MultiplexMessageType {
        match self {
            MultiplexMessage::Default{ r#type } => (*r#type).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


