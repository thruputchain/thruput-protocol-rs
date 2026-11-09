// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 1a0af366169a99b67ad0a6ed7a8eb58f45da5b7c

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
impl xdrs::XDRDiscriminant for MultiplexMessageType {
    const ENCODING: xdrs::DiscriminantEncoding = xdrs::DiscriminantEncoding::Signed;
    fn discriminant(&self) -> i32 {
        self.0
    }
    fn from_discriminant(d: i32) -> Self {
        Self(d)
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
#[xdr(discriminant = "MultiplexMessageType")]
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
    #[xdr(default_variant)] Default { r#type: MultiplexMessageType } = {
        const __XDR_CASES: &[i32] = &[
            MultiplexMessageType::MultiplexMessageSubmittedTxs.0 as i32,
            MultiplexMessageType::MultiplexMessageConsensusBasis.0 as i32,
            MultiplexMessageType::MultiplexMessageEnvelope.0 as i32,
        ];
        let mut tag = i32::MIN;
        let mut i = 0;
        while i < __XDR_CASES.len() {
            if __XDR_CASES[i] == tag {
                tag += 1;
                i = 0;
            } else {
                i += 1;
            }
        }
        tag
    },
}


//
// XDR internals below this point
//

impl MultiplexMessage {
    #[allow(clippy::match_single_binding)]
    pub fn r#type(&self) -> MultiplexMessageType {
        match self {
            MultiplexMessage::MultiplexMessageSubmittedTxs { .. } => MultiplexMessageType::MultiplexMessageSubmittedTxs,
            MultiplexMessage::MultiplexMessageConsensusBasis { .. } => MultiplexMessageType::MultiplexMessageConsensusBasis,
            MultiplexMessage::MultiplexMessageEnvelope { .. } => MultiplexMessageType::MultiplexMessageEnvelope,
            MultiplexMessage::Default{ r#type } => *r#type,

        }
    }
}


