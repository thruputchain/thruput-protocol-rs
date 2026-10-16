// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 6274c208f1d444c1450f0a2111b8b1918e067237

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

/// Reason a storage delta could not be applied during two-phase commit.
/// These are the nondeterministic failures of try_push_new_subtransaction:
/// any of them causes ExecResultTxNonDeterministicError, and the transaction
/// is excluded from the block. Codes are intentionally coarse; finer-grained
/// detail may be added later behind a debug-only extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DbConflictCode(pub i32);

impl DbConflictCode {
    /// Exclusive-lock collision: the key is exclusively locked by another
    /// transaction (committed or concurrent), an exclusive lock could not be
    /// acquired because other transactions hold the key, or an exclusive-lock
    /// flag was used on a key type that does not support it.
    pub const ExclusiveLockDenied: Self = Self(0);

    /// Two transactions created different item types at the same key.
    pub const TypeMismatch: Self = Self(1);

    /// Two transactions wrote irreconcilable values of the same type at the
    /// same key: different bytestrings, different nnint base values,
    /// different auth-data bases, or different contract / deployment hashes.
    pub const ValueMismatch: Self = Self(2);

    /// A nonnegative int would go below zero given the deltas in this block.
    /// A native HOG balance that cannot cover its debits reports
    /// InsufficientFunds instead.
    pub const BoundsExceeded: Self = Self(3);

    /// Integer overflow combining deltas on a nonnegative int or balance.
    pub const Overflow: Self = Self(4);

    /// Replay-protection conflict: uniquifier expired, already consumed (in
    /// a prior block or by a concurrent transaction), or consumed twice by
    /// this transaction.
    pub const ReplayProtection: Self = Self(5);

    /// A retrocausal queue already has a consumer in this block.
    pub const QueueConsumer: Self = Self(6);

    /// A per-item capacity limit was hit: replay-cache active heights or
    /// nonces, retrocausal-queue block entries, or auth signer slots.
    pub const CapacityExceeded: Self = Self(7);

    /// The native HOG balance at the diagnostic's address cannot cover the
    /// debits in this block, or a newly created balance item cannot cover
    /// its own storage cost. Storage written under a contract's address is
    /// charged to that contract's balance, so a deployment reports this at
    /// the new contract's address when it holds no HOG.
    pub const InsufficientFunds: Self = Self(8);

    /// An auth-data change would register a public key in a slot while
    /// another slot of the same address already holds that key, whether
    /// committed or set by the same change. The table would count one
    /// key's weight twice, so the change is refused.
    pub const DuplicateSigner: Self = Self(9);
}
impl From<i32> for DbConflictCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for DbConflictCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::ExclusiveLockDenied.0 => {
                write!(f, "DbConflictCode::ExclusiveLockDenied")
            },
            x if x == Self::TypeMismatch.0 => {
                write!(f, "DbConflictCode::TypeMismatch")
            },
            x if x == Self::ValueMismatch.0 => {
                write!(f, "DbConflictCode::ValueMismatch")
            },
            x if x == Self::BoundsExceeded.0 => {
                write!(f, "DbConflictCode::BoundsExceeded")
            },
            x if x == Self::Overflow.0 => {
                write!(f, "DbConflictCode::Overflow")
            },
            x if x == Self::ReplayProtection.0 => {
                write!(f, "DbConflictCode::ReplayProtection")
            },
            x if x == Self::QueueConsumer.0 => {
                write!(f, "DbConflictCode::QueueConsumer")
            },
            x if x == Self::CapacityExceeded.0 => {
                write!(f, "DbConflictCode::CapacityExceeded")
            },
            x if x == Self::InsufficientFunds.0 => {
                write!(f, "DbConflictCode::InsufficientFunds")
            },
            x if x == Self::DuplicateSigner.0 => {
                write!(f, "DbConflictCode::DuplicateSigner")
            },
            _ => {
                write!(f, "DbConflictCode({})", self.0)
            }
        }
    }
}


/// How much conflict diagnostic detail try_push_new_subtransaction reports
/// on failure. Selected by the database from the block's execution mode:
/// proposal blocks report First (release builds) or All (debug builds);
/// validation and viewing blocks report None.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct DbDiagnosticLevel(pub i32);

impl DbDiagnosticLevel {
    /// No diagnostics (validation / viewing; previous behavior).
    pub const kDbDiagnosticNone: Self = Self(0);

    /// Report only the first conflict encountered (proposal, release builds).
    pub const kDbDiagnosticFirst: Self = Self(1);

    /// Report a best-effort list of all conflicting deltas in the batch
    /// (proposal, debug builds — SDK testing).
    pub const kDbDiagnosticAll: Self = Self(2);
}
impl From<i32> for DbDiagnosticLevel {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for DbDiagnosticLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::kDbDiagnosticNone.0 => {
                write!(f, "DbDiagnosticLevel::kDbDiagnosticNone")
            },
            x if x == Self::kDbDiagnosticFirst.0 => {
                write!(f, "DbDiagnosticLevel::kDbDiagnosticFirst")
            },
            x if x == Self::kDbDiagnosticAll.0 => {
                write!(f, "DbDiagnosticLevel::kDbDiagnosticAll")
            },
            _ => {
                write!(f, "DbDiagnosticLevel({})", self.0)
            }
        }
    }
}


/// One conflicting storage op. Fixed-size: `ContractAddress[32]` plus a
/// fixed-size StorageKey plus the code — no heap allocation.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct DbConflictDiagnostic {
    pub addrkey: AddressAndKey,
    pub code: DbConflictCode,
}

/// Conflict diagnostics attached to a failed try_push_new_subtransaction.
/// The discriminant is the DbDiagnosticLevel the database selected from
/// the block's execution mode, so the variable-length arm is only ever
/// constructed in kDbDiagnosticAll mode (debug-build proposals); prod
/// levels are allocation-free by construction.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum DbConflictResult {
    // case kDbDiagnosticNone:
    //   void;
    kDbDiagnosticNone = DbDiagnosticLevel::kDbDiagnosticNone.0 as i32,
    // case kDbDiagnosticFirst:
    //   DbConflictDiagnostic first;
    kDbDiagnosticFirst { first: DbConflictDiagnostic } = DbDiagnosticLevel::kDbDiagnosticFirst.0 as i32,
    // case kDbDiagnosticAll:
    //   VecM<DbConflictDiagnostic> conflicts;
    kDbDiagnosticAll { conflicts: VecM<DbConflictDiagnostic> } = DbDiagnosticLevel::kDbDiagnosticAll.0 as i32,
    
}

/// Reason the executor refused a transaction's contract attachments
/// (Transaction.contracts_to_deploy) at proposal time. Any of them drops the
/// transaction from the proposal; none is a contract fault. The RV32 codes
/// name which check of the artifact failed, so an author can tell a
/// toolchain/node mismatch from an oversized or corrupt build.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct ContractRejectCode(pub i32);

impl ContractRejectCode {
    /// The transaction attaches more contracts than the chain's
    /// max_new_contracts_per_tx permits.
    pub const ContractRejectTooMany: Self = Self(0);

    /// The contract blob exceeds the chain's max_contract_size_bytes.
    pub const ContractRejectBlobTooLarge: Self = Self(1);

    /// The attachment is not an RV32 contract. Native contracts are registered
    /// at genesis; RV32 is the only deployable type.
    pub const ContractRejectNotDeployable: Self = Self(2);

    /// The same contract is attached more than once.
    pub const ContractRejectDuplicate: Self = Self(3);

    /// The blob is not a readable RV32 artifact: too short, wrong magic, wrong
    /// header size, or its length disagrees with the section sizes the header
    /// declares.
    pub const ContractRejectMalformedArtifact: Self = Self(4);

    /// The artifact's format version is not the one this node reads.
    pub const ContractRejectUnsupportedFormat: Self = Self(5);

    /// The artifact's VM profile, flags, entry ABI, or syscall ABI is not one
    /// this node runs.
    pub const ContractRejectUnsupportedAbi: Self = Self(6);

    /// The gas schedule (version, digest) stamped into the artifact is not one
    /// the node accepts: the artifact was lowered by a toolchain whose gas
    /// schedule differs from the node's.
    pub const ContractRejectGasScheduleMismatch: Self = Self(7);

    /// The declared guest memory exceeds the chain's max_rv32_memory_bytes.
    pub const ContractRejectMemoryTooLarge: Self = Self(8);

    /// The code section is empty or exceeds the chain's max_rv32_code_bytes.
    pub const ContractRejectCodeSizeInvalid: Self = Self(9);

    /// The initial memory image does not fit in the declared guest memory, or
    /// an empty image declares a nonzero load address.
    pub const ContractRejectInitialMemoryInvalid: Self = Self(10);

    /// A section's bytes do not match the SHA-256 digest the header records.
    pub const ContractRejectDigestMismatch: Self = Self(11);

    /// The code does not translate: it contains an unsupported or malformed
    /// instruction.
    pub const ContractRejectInvalidProgram: Self = Self(12);
}
impl From<i32> for ContractRejectCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for ContractRejectCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::ContractRejectTooMany.0 => {
                write!(f, "ContractRejectCode::ContractRejectTooMany")
            },
            x if x == Self::ContractRejectBlobTooLarge.0 => {
                write!(f, "ContractRejectCode::ContractRejectBlobTooLarge")
            },
            x if x == Self::ContractRejectNotDeployable.0 => {
                write!(f, "ContractRejectCode::ContractRejectNotDeployable")
            },
            x if x == Self::ContractRejectDuplicate.0 => {
                write!(f, "ContractRejectCode::ContractRejectDuplicate")
            },
            x if x == Self::ContractRejectMalformedArtifact.0 => {
                write!(f, "ContractRejectCode::ContractRejectMalformedArtifact")
            },
            x if x == Self::ContractRejectUnsupportedFormat.0 => {
                write!(f, "ContractRejectCode::ContractRejectUnsupportedFormat")
            },
            x if x == Self::ContractRejectUnsupportedAbi.0 => {
                write!(f, "ContractRejectCode::ContractRejectUnsupportedAbi")
            },
            x if x == Self::ContractRejectGasScheduleMismatch.0 => {
                write!(f, "ContractRejectCode::ContractRejectGasScheduleMismatch")
            },
            x if x == Self::ContractRejectMemoryTooLarge.0 => {
                write!(f, "ContractRejectCode::ContractRejectMemoryTooLarge")
            },
            x if x == Self::ContractRejectCodeSizeInvalid.0 => {
                write!(f, "ContractRejectCode::ContractRejectCodeSizeInvalid")
            },
            x if x == Self::ContractRejectInitialMemoryInvalid.0 => {
                write!(f, "ContractRejectCode::ContractRejectInitialMemoryInvalid")
            },
            x if x == Self::ContractRejectDigestMismatch.0 => {
                write!(f, "ContractRejectCode::ContractRejectDigestMismatch")
            },
            x if x == Self::ContractRejectInvalidProgram.0 => {
                write!(f, "ContractRejectCode::ContractRejectInvalidProgram")
            },
            _ => {
                write!(f, "ContractRejectCode({})", self.0)
            }
        }
    }
}


/// The first contract attachment the executor refused.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct ContractRejectDiagnostic {
    /// Position in Transaction.contracts_to_deploy of the refused contract.
    /// For ContractRejectTooMany, the first position past the limit.
    pub contract_index: u32,
    pub code: ContractRejectCode,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct TxFailureKind(pub i32);

impl TxFailureKind {
    /// Two-phase commit could not apply the transaction's storage deltas.
    pub const TxFailureDbConflict: Self = Self(0);

    /// The executor refused a contract attachment before execution.
    pub const TxFailureContractRejected: Self = Self(1);
}
impl From<i32> for TxFailureKind {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for TxFailureKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::TxFailureDbConflict.0 => {
                write!(f, "TxFailureKind::TxFailureDbConflict")
            },
            x if x == Self::TxFailureContractRejected.0 => {
                write!(f, "TxFailureKind::TxFailureContractRejected")
            },
            _ => {
                write!(f, "TxFailureKind({})", self.0)
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
pub enum TxFailureCause {
    // case TxFailureDbConflict:
    //   DbConflictDiagnostic conflict;
    TxFailureDbConflict { conflict: DbConflictDiagnostic } = TxFailureKind::TxFailureDbConflict.0 as i32,
    // case TxFailureContractRejected:
    //   ContractRejectDiagnostic contract;
    TxFailureContractRejected { contract: ContractRejectDiagnostic } = TxFailureKind::TxFailureContractRejected.0 as i32,
    
}

/// Best-effort diagnostic for a transaction the proposer excluded from a
/// proposal: the executing block height of the failed attempt plus why it
/// failed -- the first conflicting storage op hog_db reported during
/// two-phase commit, or the first contract attachment the executor refused.
/// Fixed-size by construction; the wire and caches carry exactly one
/// diagnostic per failure to bound memory end to end.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TxFailureDiagnostic {
    pub attempt_height: BlockHeight,
    pub cause: TxFailureCause,
}


//
// XDR internals below this point
//

impl DbConflictResult {
    pub fn level(&self) -> DbDiagnosticLevel {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl TxFailureCause {
    pub fn kind(&self) -> TxFailureKind {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


