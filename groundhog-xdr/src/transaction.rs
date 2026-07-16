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
#[cfg(feature = "xdrs")]
use xdrs::*;
#[cfg(feature = "spec")]
use xdrs::xdr_repr::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;



// clang-format on

//==============================================================================
// Transactions and Invocations
//
// This module defines the transaction model for Groundhog. Transactions are
// the fundamental unit of state modification in the system.
//
// Key concepts:
// - Each transaction invokes a contract method with specified parameters
// - Transactions include authorization claims and signatures
// - Transactions can deploy new contracts
// - Groundhog executes transactions in parallel, using two-phase commit for conflict resolution
// - Proposers attach nondeterministic data to ensure deterministic re-execution
//
// Transaction lifecycle:
// 1. User constructs a Transaction with invocation details, auth claims, etc.
// 2. Signatures are added to create a SignedTransaction
// 3. Transaction is submitted to the network
// 4. A proposer executes the transaction and captures nondeterministic data
// 5. Proposer creates a ProposedTransaction and includes it in a block proposal
// 6. Validators re-execute using the nondeterministic data to verify correctness
//==============================================================================

//------------------------------------------------------------------------------
// Call Data and Return Values
//------------------------------------------------------------------------------

/// Opaque encoded call arguments passed to a contract invocation.
///
/// Contracts can encode calldata arbitrarily, but the SDK default uses
/// XDR serialization per RFC 4506 (<https://datatracker.ietf.org/doc/html/rfc4506>).
/// The SDK typically encodes a tuple or struct of the method's parameters.
///
/// Note: "TXS" refers to "traditional XDR serialization". A more compact
/// encoding is planned but not yet enabled.
pub type Calldata = BytesM;

/// Maximum size of data that can be returned from a contract invocation.
///
/// Return values larger than this will cause the invocation to revert
/// with a deterministic error (equivalent to wasm `unreachable`).
// 
pub const kMaxReturnBufferLen : u32 = 1024;

/// Opaque encoded return value from a contract invocation.
///
/// Like Calldata, contracts can encode return values arbitrarily, but the
/// SDK default uses XDR serialization per RFC 4506 (<https://datatracker.ietf.org/doc/html/rfc4506>).
pub type ReturnBuffer = BytesM::<kMaxReturnBufferLen>;

//------------------------------------------------------------------------------
// Invocations
//------------------------------------------------------------------------------

/// Specifies a contract method invocation with its parameters.
///
/// This struct is used both for the root transaction invocation and for
/// subinvocations made via the Invoke syscall.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Invocation {
    /// Address of the contract to invoke.
    pub invoked_address: ContractAddress,
    /// 8-byte identifier of the contract method to call.
    ///
    /// See [`ContractEntrypointID`] in contract.x for how these are derived.
    pub entrypoint: ContractEntrypointID,
    /// Maximum gas units available for this invocation.
    ///
    /// For the root transaction invocation, this is the total gas budget.
    /// For subinvocations via the Invoke syscall, this is capped at the
    /// remaining gas in the caller's frame. The SDK default is UINT64_MAX,
    /// meaning "use all remaining gas".
    pub gas_limit: GasAmount,
    /// Encoded arguments to pass to the contract method.
    pub calldata: Calldata,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct InvocationFrameStatusCode(pub i32);

impl InvocationFrameStatusCode {
    pub const Return: Self = Self(0);
    pub const Fault: Self = Self(1);
    pub const Abort: Self = Self(2);
}
impl From<i32> for InvocationFrameStatusCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for InvocationFrameStatusCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::Return.0 => {
                write!(f, "InvocationFrameStatusCode::Return")
            },
            x if x == Self::Fault.0 => {
                write!(f, "InvocationFrameStatusCode::Fault")
            },
            x if x == Self::Abort.0 => {
                write!(f, "InvocationFrameStatusCode::Abort")
            },
            _ => {
                write!(f, "InvocationFrameStatusCode({})", self.0)
            }
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(i32)]
#[non_exhaustive]
pub enum InvocationFrameOutput {
    // case Return:
    //   ReturnBuffer return_buffer;
    Return { return_buffer: ReturnBuffer } = InvocationFrameStatusCode::Return.0 as i32,
    // case Abort:
    //   ReturnBuffer abort_buffer;
    Abort { abort_buffer: ReturnBuffer } = InvocationFrameStatusCode::Abort.0 as i32,
    // case Fault:
    //   void;
    Fault = InvocationFrameStatusCode::Fault.0 as i32,
    
}

//------------------------------------------------------------------------------
// Transaction Uniqueness and Replay Protection
//------------------------------------------------------------------------------

/// Ensures transaction uniqueness and prevents replay attacks.
///
/// Each transaction must include a uniquifier with two components:
/// 1. An expiry height - the transaction becomes invalid after this block
/// 2. A nonce - ensures uniqueness even if the same operation is repeated
///
/// The uniquifier is stored in the replay cache (see [`StorageKeyType::ReplayCache`]
/// in types.x) at the invoked contract's address to prevent replays.
///
/// Important: The replay cache considers the (nonce, block_expiry_height) pair as the
/// unique identifier. Two transactions with the same nonce but different expiry heights
/// do NOT conflict with each other. For example, (nonce=1, expiry=100) and (nonce=1, expiry=200)
/// can both be valid transactions.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Uniquifier {
    /// The block height after which this transaction becomes invalid.
    ///
    /// Transactions submitted after this height will be rejected. This prevents
    /// old transactions from being replayed indefinitely.
    ///
    /// Note: This is part of the uniqueness check - the pair (nonce, block_expiry_height)
    /// must be unique, not just the nonce alone.
    pub block_expiry_height: BlockHeight,
    /// A 64-bit value chosen by the transaction creator.
    ///
    /// Combined with block_expiry_height and the invoked address, this ensures each
    /// transaction is globally unique. The nonce can be chosen arbitrarily (random,
    /// sequential, timestamp-based, etc.) as long as the (nonce, block_expiry_height)
    /// pair hasn't been used before for this address.
    pub nonce: u64,
}

//------------------------------------------------------------------------------
// Transactions
//------------------------------------------------------------------------------

/// A complete transaction that can be executed on the Groundhog blockchain.
///
/// Transactions are the fundamental unit of state modification. Each transaction:
/// - Invokes a single contract method (which may make subinvocations)
/// - Includes claims about attached signatures, which must be verified before execution 
///   can begin.  A transaction is invalid if any attached signatures are invalid
///   or if the attached signatures do not match the auth claims.
/// - Can deploy new contracts that will be available during execution
/// - Must have a [`Uniquifier`] that has never before been used for this address
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct Transaction {
    /// The root contract invocation for this transaction.
    pub invocation: Invocation,
    /// Uniqueness and replay protection data.
    pub uniquifier: Uniquifier,
    /// New contracts to deploy before execution begins.
    ///
    /// These contracts become available on-chain before the root invocation
    /// executes, allowing the transaction to deploy and immediately use contracts.
    /// Contracts already deployed on-chain do not need to be included here.
    ///
    /// Deployed via the DeployContract syscall during transaction execution.
    pub contracts_to_deploy: VecM<Contract>,
    /// Authorization claim specifying required signatures.
    ///
    /// See [`AuthClaim`] in auth_claim.x for details.
    pub auth_claim: AuthClaim,
}

//------------------------------------------------------------------------------
// Signatures and Signed Transactions
//------------------------------------------------------------------------------

/// A cryptographic signature paired with its key identifier.
///
/// The public_key_id indicates which key from an address's key set produced
/// this signature, allowing the signature to be matched against the transaction's
/// auth claims.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DecoratedSignature {
    /// The KeyID of the public key that created this signature.
    ///
    /// This is matched against the auth claims to determine if the signature
    /// satisfies the transaction's authorization requirements. See [`AuthClaim`]
    /// in auth_claim.x for details on how keys are registered and weighted.
    // uint32_t
    pub public_key_id: u32,
    /// The cryptographic signature itself.
    ///
    /// See [`Signature`] in types.x for the union that supports different schemes.
    pub signature: Signature,
}

/// Hash of a signed transaction, used as a unique transaction identifier.
///
/// This is the SHA-256 hash of the XDR-encoded SignedTransaction.
pub type SignedTransactionHash = SHA256Hash;

/// A transaction with cryptographic signatures.
///
/// This is the form in which transactions are transmitted and stored.
/// Before execution, the signatures are verified against the auth claims.
///
/// The signatures are over the [`Transaction`] object.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SignedTransaction {
    /// The transaction to execute.
    pub tx: Transaction,
    /// Signatures that authorize this transaction.
    ///
    /// These signatures must satisfy the auth_claim in the transaction.
    /// The signatures are over the [`Transaction`] object.
    pub signatures: VecM<DecoratedSignature>,
}

//------------------------------------------------------------------------------
// Batch Types
//------------------------------------------------------------------------------

/// A batch of transactions (without signatures).
pub type TransactionBatch = VecM::<Transaction>;

/// A batch of signed transactions.
pub type SignedTransactionBatch = VecM::<SignedTransaction>;

/// An optional signed transaction (pointer may be null).
pub type OptSignedTransaction = Option::<SignedTransaction>;

/// A batch of optional signed transactions.
pub type OptSignedTransactionBatch = VecM::<OptSignedTransaction>;

/// A batch of signed transaction hashes.
pub type SignedTransactionHashBatch = VecM::<SignedTransactionHash>;

//------------------------------------------------------------------------------
// Proposed Transactions and Nondeterministic Data
//------------------------------------------------------------------------------

/// Nondeterministic data captured during transaction execution by the proposer.
///
/// Groundhog executes transactions in parallel, which introduces nondeterminism
/// (e.g., from two-phase commit conflict resolution). To ensure validators can deterministically
/// re-execute transactions, the proposer captures all nondeterministic decisions
/// and includes them in the block proposal.
///
/// This allows validators to replay the exact same execution path as the proposer,
/// ensuring consensus on the transaction result.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct NondeterministicProposalData {
    /// A bitstring of nondeterministic decisions made during execution.
    ///
    /// This includes:
    /// - Two-phase commit conflict resolution results from try_invoke frames (one boolean per frame)
    /// - Other nondeterministic outcomes that must be consistent across validators
    ///
    /// Each bit represents a binary decision point encountered during execution.
    /// The bits are consumed sequentially as the transaction re-executes.
    pub nondeterministic_bits: BytesM,
    /// The length of nondeterministic_bits in bits (not bytes).
    ///
    /// Since nondeterministic_bits.size() returns the rounded-up length in bytes,
    /// this field tracks the exact bit count to avoid trailing garbage bits.
    pub nondeterministic_bit_count: u32,
    /// Sequence of retrocausal queue entries observed during execution.
    ///
    /// TODO(geoff): Update this documentation once retrocausal queue handling
    /// is finalized. This field captures the queue entries consumed during
    /// execution to ensure deterministic replay.
    pub observed_queue_entry_sequence: VecM<QueueEntry, kMaxConcurrentRcqReadsPerTx>,
}

/// A signed transaction with nondeterministic execution data attached.
///
/// This is the form in which transactions are included in block proposals.
/// The proposer:
/// 1. Executes the signed transaction
/// 2. Captures all nondeterministic decisions (two-phase commit results, queue entries, etc.)
/// 3. Packages both into a ProposedTransaction
/// 4. Includes it in the proposed block
///
/// Validators then re-execute using the nondeterministic data to verify
/// the proposer's execution was correct.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ProposedTransaction {
    /// The signed transaction to execute.
    pub signed_tx: SignedTransaction,
    /// Nondeterministic data captured during the proposer's execution.
    ///
    /// Validators use this to deterministically replay the same execution path.
    pub nondeterministic_proposal_data: NondeterministicProposalData,
}

pub type ProposedTransactionBatch = VecM::<ProposedTransaction>;

//------------------------------------------------------------------------------
// Transaction Statistics
//------------------------------------------------------------------------------

/// Transaction execution statistics available via the GetTransactionStatistics syscall.
///
/// These statistics provide runtime information about transaction execution,
/// primarily for debugging and monitoring. Some statistics may be disabled
/// in production builds.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[cfg_attr(feature = "xdrs", derive(ReadXDR, WriteXDR))]
#[cfg_attr(feature = "spec", derive(XDR))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct TransactionStatistic(pub i32);

impl TransactionStatistic {
    /// The amount of gas remaining in the current call frame.
    ///
    /// This can be queried to monitor gas consumption during execution.
    /// Note that this statistic may be disabled or return nondeterministic
    /// values in test configurations (see syscalls.yaml for details).
    pub const GasRemaining: Self = Self(0);
}
impl From<i32> for TransactionStatistic {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for TransactionStatistic {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::GasRemaining.0 => {
                write!(f, "TransactionStatistic::GasRemaining")
            },
            _ => {
                write!(f, "TransactionStatistic({})", self.0)
            }
        }
    }
}



//
// XDR internals below this point
//

impl InvocationFrameOutput {
    pub fn status_code(&self) -> InvocationFrameStatusCode {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


