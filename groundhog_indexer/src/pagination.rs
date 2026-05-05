// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: b516a2309c47d23e003572fcd5df7834a31d42e9

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

/// Direction to walk a paginated stream.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct IterationOrder(pub i32);

impl IterationOrder {
    pub const IterationOrderAscending: Self = Self(0);
    pub const IterationOrderDescending: Self = Self(1);
}
impl From<i32> for IterationOrder {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for IterationOrder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::IterationOrderAscending.0 => {
                write!(f, "IterationOrder::IterationOrderAscending")
            },
            x if x == Self::IterationOrderDescending.0 => {
                write!(f, "IterationOrder::IterationOrderDescending")
            },
            _ => {
                write!(f, "IterationOrder({})", self.0)
            }
        }
    }
}


/// Where to begin iteration on the initial call.  Independent of
/// `IterationOrder`; combinations that run off the end of the range
/// (AnchorAtEnd with IterationOrderAscending, AnchorAtBeginning with
/// IterationOrderDescending) return empty, because there is nothing
/// past that end to iterate toward.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct BlockAnchorKind(pub i32);

impl BlockAnchorKind {
    // oldest block the indexer still retains
    pub const AnchorAtBeginning: Self = Self(0);

    // highest block the indexer has processed
    pub const AnchorAtEnd: Self = Self(1);

    // caller-specified
    pub const AnchorAtHeight: Self = Self(2);
}
impl From<i32> for BlockAnchorKind {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for BlockAnchorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::AnchorAtBeginning.0 => {
                write!(f, "BlockAnchorKind::AnchorAtBeginning")
            },
            x if x == Self::AnchorAtEnd.0 => {
                write!(f, "BlockAnchorKind::AnchorAtEnd")
            },
            x if x == Self::AnchorAtHeight.0 => {
                write!(f, "BlockAnchorKind::AnchorAtHeight")
            },
            _ => {
                write!(f, "BlockAnchorKind({})", self.0)
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
pub enum BlockAnchor {
    // case AnchorAtBeginning:
    // case AnchorAtEnd:
    //   void;
    AnchorAtBeginning = BlockAnchorKind::AnchorAtBeginning.0 as i32,
    AnchorAtEnd = BlockAnchorKind::AnchorAtEnd.0 as i32,
    // case AnchorAtHeight:
    //   BlockHeight height;
    AnchorAtHeight { height: BlockHeight } = BlockAnchorKind::AnchorAtHeight.0 as i32,
    
}

/// Opaque pagination cursor authored by the server.  Clients pass
/// back whatever they received in the previous response's `cursor`
/// field to continue pagination.  The encoding is a server-side
/// implementation detail and may change between versions; the first
/// byte is a format-version tag so stale cursors fail cleanly rather
/// than being silently misinterpreted.
pub type PaginationCursor = BytesM;


//
// XDR internals below this point
//

impl BlockAnchor {
    pub fn kind(&self) -> BlockAnchorKind {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


