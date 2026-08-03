// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 

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

/// Reason a storage delta could not be applied during two-phase commit.
/// These are the nondeterministic failures of try_push_new_subtransaction:
/// any of them causes ExecResultTxNonDeterministicError, and the transaction
/// is excluded from the block. Codes are intentionally coarse; finer-grained
/// detail may be added later behind a debug-only extension.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

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
    pub const kDbConflictExclusiveLock: Self = Self(0);

    /// Two transactions created different item types at the same key.
    pub const kDbConflictTypeMismatch: Self = Self(1);

    /// Two transactions wrote irreconcilable values of the same type at the
    /// same key: different bytestrings, different nnint base values,
    /// different auth-data bases, or different contract / deployment hashes.
    pub const kDbConflictValueMismatch: Self = Self(2);

    /// A nonnegative int (or balance) would go below zero given the deltas
    /// in this block, or a newly created balance item cannot cover its own
    /// storage cost.
    pub const kDbConflictBoundsExceeded: Self = Self(3);

    /// Integer overflow combining deltas on a nonnegative int or balance.
    pub const kDbConflictOverflow: Self = Self(4);

    /// Replay-protection conflict: uniquifier expired, already consumed (in
    /// a prior block or by a concurrent transaction), or consumed twice by
    /// this transaction.
    pub const kDbConflictReplayProtection: Self = Self(5);

    /// A retrocausal queue already has a consumer in this block.
    pub const kDbConflictQueueConsumer: Self = Self(6);

    /// A per-item capacity limit was hit: replay-cache active heights or
    /// nonces, retrocausal-queue block entries, or auth signer slots.
    pub const kDbConflictCapacityExceeded: Self = Self(7);
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
            x if x == Self::kDbConflictExclusiveLock.0 => {
                write!(f, "DbConflictCode::kDbConflictExclusiveLock")
            },
            x if x == Self::kDbConflictTypeMismatch.0 => {
                write!(f, "DbConflictCode::kDbConflictTypeMismatch")
            },
            x if x == Self::kDbConflictValueMismatch.0 => {
                write!(f, "DbConflictCode::kDbConflictValueMismatch")
            },
            x if x == Self::kDbConflictBoundsExceeded.0 => {
                write!(f, "DbConflictCode::kDbConflictBoundsExceeded")
            },
            x if x == Self::kDbConflictOverflow.0 => {
                write!(f, "DbConflictCode::kDbConflictOverflow")
            },
            x if x == Self::kDbConflictReplayProtection.0 => {
                write!(f, "DbConflictCode::kDbConflictReplayProtection")
            },
            x if x == Self::kDbConflictQueueConsumer.0 => {
                write!(f, "DbConflictCode::kDbConflictQueueConsumer")
            },
            x if x == Self::kDbConflictCapacityExceeded.0 => {
                write!(f, "DbConflictCode::kDbConflictCapacityExceeded")
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
#[derive(ReadXDR, WriteXDR)]

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
#[derive(ReadXDR, WriteXDR)]

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
#[derive(ReadXDR, WriteXDR)]

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

/// Best-effort diagnostic for a transaction that failed nondeterministically
/// during two-phase commit at proposal time: the executing block height of
/// the failed attempt plus the first conflicting storage op hogdb reported.
/// Fixed-size by construction; the wire and caches carry exactly one
/// diagnostic per failure to bound memory end to end.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct TxFailureDiagnostic {
    pub attempt_height: BlockHeight,
    pub conflict: DbConflictDiagnostic,
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


