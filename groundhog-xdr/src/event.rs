// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 667c466d8f5daf4d16a68920704d5ae7fc1660b0

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
#[cfg(feature = "xdrs")]
use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};



// clang-format on

// The event format puts the first N * 32 bytes of the data
// as the topics.

/// Maximum length in bytes for a serialized event.
/// Events exceeding this length will be rejected by the LogEvent syscall.
// 
pub const kMaxEventLength : u32 = 1024;

/// Maximum number of events a single transaction may emit.
/// This keeps a single TxEventLog well below the RPC transport message limit.
// 
pub const kMaxEventsPerTransaction : u32 = 1000;

/// Maximum number of topics that can be attached to an event.
/// Topics enable efficient indexing and filtering of events.
pub const kMaxTopics : u32 = 3;

/// A 32-byte topic used for event indexing.
/// Topics are prepended to the serialized event data. The num_topics field
/// indicates how many of the leading 32-byte chunks are topics (up to kMaxTopics).
/// Downstream indexers use topics to efficiently filter events without deserializing.
// 
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Default, Copy)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Topic {
    #[cfg_attr(feature = "serde", serde(with = "xdr_common::serde_hex"))]
    inner: [u8; 32 as usize],
}
impl From<[u8; 32 as usize]> for Topic {
    fn from(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}
impl Topic {
    pub fn into_inner(self) -> [u8; 32 as usize] {
        self.inner
    }
    pub const fn new(_inner: [u8; 32 as usize]) -> Self {
        Self { inner: _inner }
    }
}

impl Deref for Topic {
    type Target = [u8; 32 as usize];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for Topic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<[u8]> for Topic {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_slice()
    }
}
impl<'a> From<&'a [u8; 32]> for &'a Topic {
    fn from(arr: &[u8; 32]) -> Self {
        let ptr = arr as *const [u8; 32] as *const Topic;
        unsafe { &*ptr }
    }
}
#[cfg(feature = "xdrs")]
impl<'r> ReadXDR<'r> for &'r Topic {
  fn read_xdr<R: xdrs::Reader<'r>>(r: &mut R) -> xdrs::ReadResult<Self> {
      let arr: &[u8; 32] = xdrs::read_xdr(r)?;
      let val = Self::from(arr);
      Ok(val)
  }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for Topic {
    fn schema_name() -> ::std::borrow::Cow<'static, str> {
        "Topic".into()
    }
    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "type": "string",
            "contentEncoding": "hex",
            "contentMediaType": "application/binary",
            "minLength": 64,
            "maxLength": 64
        })
    }
}



/// An event emitted by a contract during transaction execution.
/// Events are logged via the LogEvent syscall and stored in the transaction's event log.
///
/// The serialized_event field contains topic0 || topic1 || topic2 || data, where
/// the number of topic fields depends on num_topics.
/// This structure is the minimum unit forwarded to downstream indexing systems.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TxEmittedEvent {
    pub contract_address: ContractAddress,
    // <= kMaxTopics
    pub num_topics: u32,
    pub serialized_event: BytesM<kMaxEventLength>,
}

/// Event log for a single transaction, stored in the merkle-patricia trie.
/// The trie leaf is encoded canonically as follows:
///
/// - key: the leading TransactionHash from XDR(TxEventLog);
/// - value: the remainder of XDR(TxEventLog), followed by
///   XDR(NondeterministicProposalData).
///
/// The appended proposal data binds the nondeterministic inputs used during
/// execution to the events root. Signature-wrapper bytes are deliberately not
/// included: TransactionHash identifies the inner Transaction independently
/// of its signatures, so validators with different valid wrappers compute the
/// same root.
///
/// That key is the identity of the transaction the submitter signed, not of
/// the [`ProposedTransaction`] the proposer assembled from it, so a
/// submitter can compute it before submission and use it to find its own
/// events. See [`TransactionHash`].
///
/// `root_output` carries the InvocationFrameOutput of the transaction's
/// root frame: the Return buffer on success, the Abort buffer on a
/// deterministic revert, or the Fault variant when no root output was
/// produced (e.g., non-deterministic failure).  Downstream indexers and
/// the receipt-read paths surface this as `return_data`/`abort_data`.
///
/// `gas_consumed` is the gas the whole transaction consumed: the root frame,
/// every frame it invoked, and the pre-execution validation of attached
/// contracts. On an out-of-gas fault it is the transaction's gas limit. It
/// is part of the leaf, so the events root commits to it and every
/// validator's re-execution must reproduce it.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TxEventLog {
    // sha256(Transaction)
    pub tx_hash: TransactionHash,
    pub exec_result: ExecResult,
    pub root_output: InvocationFrameOutput,
    pub gas_consumed: GasAmount,
    pub events: VecM<TxEmittedEvent, kMaxEventsPerTransaction>,
}

/// Complete identification of a unique event within the blockchain.
/// This structure is not constructed during the main transaction workflow; instead,
/// it may be produced by downstream systems (indexers, APIs) that cross-reference
/// events across transactions and blocks.
/// The combination of tx_hash and sequence_number uniquely identifies
/// any event. The sequence_number distinguishes multiple events emitted by
/// the same transaction.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR, ConstXDRSize))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct IdentifiedEvent {
    // sha256(Transaction)
    pub tx_hash: TransactionHash,
    pub block_height: BlockHeight,
    // uniquifies the event
    pub sequence_number: u32,
    pub emitted_event: TxEmittedEvent,
}


//
// XDR internals below this point
//


