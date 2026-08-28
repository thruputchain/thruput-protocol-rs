// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 14959ad52a564640d44d7079849fb9f146d330c0

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

/// kCoordinatorMaxBytesTxsMessage is the maximum size of a dissemination
/// message containing transactions. The transaction batches inside these
/// messages will be submitted to the mempool as vetted txs.
/// Thus the mempool-to-coordinator RPC must have a slightly higher
/// maximum message size to accommodate for RPC framing.
// 2GB
pub const kCoordinatorMaxBytesTxsMessage : u32 = 2_000_000_000;

// 2GB + 10KB
pub const kMempoolCoordinatorRpcMaxBytes : u32 = 2_000_010_000;

/// kCoordinatorMaxBytesCVMessage is the maximum size of a dissemination
/// message containing consensus values.
// 70MB
pub const kCoordinatorMaxBytesCVMessage : u32 = 70_000_000;

/// kCoordinatorMaxBytesSCPEnvelopeMessage is the maximum size of a dissemination
/// message containing an SCP envelope. These messages are passed on to SCPD verbatim.
/// Thus the SCPD controller RPC (used by the coordinator) must have a slightly higher
/// maximum message size to accommodate for RPC framing.
// 5MB
pub const kCoordinatorMaxBytesSCPEnvelopeMessage : u32 = 5_000_000;

/// kCoordinatorMaxBytesSCPQuorumSet is the maximum acceptable encoded quorum set size,
/// enforced at the coordinator.
// 1MB
pub const kCoordinatorMaxBytesSCPQuorumSet : u32 = 1_000_000;

/// kMaxBytesSignedTransaction is a currently assumed (but not enforced) bound on the
/// maximum size of a signed transaction submitted through the wallet.
/// This bound is used by the coordinator to calculate how many unvetted transactions it can buffer
/// before submitting them to the mempool without violating kMempoolCoordinatorRpcMaxBytes.
// 2MB
pub const kMaxBytesSignedTransaction : u32 = 2_000_000;

/// kRPCFrameMaxBytes is an over-estimate of the maximum RPC frame size, used by the coordinator
/// to make sure it does not overflow the mempool's max RPC message size.
// 10KB
pub const kRPCFrameMaxBytes : u32 = 10_000;


//
// XDR internals below this point
//


