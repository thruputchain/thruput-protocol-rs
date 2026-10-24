// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 855ee2b6f1b64eea7b421ca083525da4e6371c5a

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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct CoordinatorRegistrationOutcome(pub i32);

impl CoordinatorRegistrationOutcome {
    pub const CoordinatorRegistrationOK: Self = Self(0);
    pub const CoordinatorRegistrationInvalidProtocol: Self = Self(1);
    pub const CoordinatorRegistrationAlreadyRegistered: Self = Self(2);
}
impl From<i32> for CoordinatorRegistrationOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for CoordinatorRegistrationOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::CoordinatorRegistrationOK.0 => {
                write!(f, "CoordinatorRegistrationOutcome::CoordinatorRegistrationOK")
            },
            x if x == Self::CoordinatorRegistrationInvalidProtocol.0 => {
                write!(f, "CoordinatorRegistrationOutcome::CoordinatorRegistrationInvalidProtocol")
            },
            x if x == Self::CoordinatorRegistrationAlreadyRegistered.0 => {
                write!(f, "CoordinatorRegistrationOutcome::CoordinatorRegistrationAlreadyRegistered")
            },
            _ => {
                write!(f, "CoordinatorRegistrationOutcome({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CoordCard {
    pub protocol: ProtocolCommit,
    pub address_for_sched: ServerAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerProposalOutcome(pub i32);

impl SchedulerProposalOutcome {
    pub const SchedulerProposalOK: Self = Self(0);
    pub const SchedulerProposalError: Self = Self(1);
}
impl From<i32> for SchedulerProposalOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedulerProposalOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedulerProposalOK.0 => {
                write!(f, "SchedulerProposalOutcome::SchedulerProposalOK")
            },
            x if x == Self::SchedulerProposalError.0 => {
                write!(f, "SchedulerProposalOutcome::SchedulerProposalError")
            },
            _ => {
                write!(f, "SchedulerProposalOutcome({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SchedulerProposedBlock {
    pub block_id: BlockID,
    pub txs: ParallelSignedTransactionBatch,
    pub proposal: Proposal,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum SchedulerProposalResult {
    // case SchedulerProposalOK:
    //   SchedulerProposedBlock proposed_block;
    SchedulerProposalOK { proposed_block: SchedulerProposedBlock } = SchedulerProposalOutcome::SchedulerProposalOK.0 as i32,
    // case SchedulerProposalError:
    //   void;
    SchedulerProposalError = SchedulerProposalOutcome::SchedulerProposalError.0 as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerValidationOutcome(pub i32);

impl SchedulerValidationOutcome {
    pub const SchedulerValidationOK: Self = Self(0);
    pub const SchedulerValidationInvalid: Self = Self(1);
    pub const SchedulerValidationMissingTxs: Self = Self(2);
    pub const SchedulerValidationDbFailed: Self = Self(3);
    pub const SchedulerValidationExecutorFailed: Self = Self(4);
    pub const SchedulerValidationTooLateToValidate: Self = Self(5);
    pub const SchedulerValidationUnknownParentBlock: Self = Self(6);

    // Proposal batch metadata is malformed or fails to decode (e.g.
    // segment token/byte counts are inconsistent, XDR deserialization
    // fails, or signed_hash/nondeterministic_data counts do not match).
    pub const SchedulerValidationMalformed: Self = Self(7);
}
impl From<i32> for SchedulerValidationOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedulerValidationOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedulerValidationOK.0 => {
                write!(f, "SchedulerValidationOutcome::SchedulerValidationOK")
            },
            x if x == Self::SchedulerValidationInvalid.0 => {
                write!(f, "SchedulerValidationOutcome::SchedulerValidationInvalid")
            },
            x if x == Self::SchedulerValidationMissingTxs.0 => {
                write!(f, "SchedulerValidationOutcome::SchedulerValidationMissingTxs")
            },
            x if x == Self::SchedulerValidationDbFailed.0 => {
                write!(f, "SchedulerValidationOutcome::SchedulerValidationDbFailed")
            },
            x if x == Self::SchedulerValidationExecutorFailed.0 => {
                write!(f, "SchedulerValidationOutcome::SchedulerValidationExecutorFailed")
            },
            x if x == Self::SchedulerValidationTooLateToValidate.0 => {
                write!(f, "SchedulerValidationOutcome::SchedulerValidationTooLateToValidate")
            },
            x if x == Self::SchedulerValidationUnknownParentBlock.0 => {
                write!(f, "SchedulerValidationOutcome::SchedulerValidationUnknownParentBlock")
            },
            x if x == Self::SchedulerValidationMalformed.0 => {
                write!(f, "SchedulerValidationOutcome::SchedulerValidationMalformed")
            },
            _ => {
                write!(f, "SchedulerValidationOutcome({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SchedulerValidationBlock {
    pub block_id: BlockID,
    pub signed_transactions: ParallelSignedTransactionBatch,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum SchedulerValidationResult {
    // case SchedulerValidationOK:
    //   SchedulerValidationBlock block;
    SchedulerValidationOK { block: SchedulerValidationBlock } = SchedulerValidationOutcome::SchedulerValidationOK.0 as i32,
    // The proposal's transactions this mempool does not hold in the proposed
    // encoding, including bodies it holds under a different signing.
    // case SchedulerValidationMissingTxs:
    //   VecM<SignedTransactionHash> missing_signed_tx_hashes;
    SchedulerValidationMissingTxs { missing_signed_tx_hashes: VecM<SignedTransactionHash> } = SchedulerValidationOutcome::SchedulerValidationMissingTxs.0 as i32,
    // case SchedulerValidationDbFailed:
    // case SchedulerValidationExecutorFailed:
    // case SchedulerValidationInvalid:
    // case SchedulerValidationTooLateToValidate:
    // case SchedulerValidationUnknownParentBlock:
    // case SchedulerValidationMalformed:
    //   void;
    SchedulerValidationDbFailed = SchedulerValidationOutcome::SchedulerValidationDbFailed.0 as i32,
    SchedulerValidationExecutorFailed = SchedulerValidationOutcome::SchedulerValidationExecutorFailed.0 as i32,
    SchedulerValidationInvalid = SchedulerValidationOutcome::SchedulerValidationInvalid.0 as i32,
    SchedulerValidationTooLateToValidate = SchedulerValidationOutcome::SchedulerValidationTooLateToValidate.0 as i32,
    SchedulerValidationUnknownParentBlock = SchedulerValidationOutcome::SchedulerValidationUnknownParentBlock.0 as i32,
    SchedulerValidationMalformed = SchedulerValidationOutcome::SchedulerValidationMalformed.0 as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerCommitOutcome(pub i32);

impl SchedulerCommitOutcome {
    pub const SchedulerCommitOK: Self = Self(0);
    pub const SchedulerCommitBlockNotFound: Self = Self(1);
    pub const SchedulerCommitInvalidConsensusValue: Self = Self(2);
    pub const SchedulerConcurrentCommit: Self = Self(3);
}
impl From<i32> for SchedulerCommitOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedulerCommitOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedulerCommitOK.0 => {
                write!(f, "SchedulerCommitOutcome::SchedulerCommitOK")
            },
            x if x == Self::SchedulerCommitBlockNotFound.0 => {
                write!(f, "SchedulerCommitOutcome::SchedulerCommitBlockNotFound")
            },
            x if x == Self::SchedulerCommitInvalidConsensusValue.0 => {
                write!(f, "SchedulerCommitOutcome::SchedulerCommitInvalidConsensusValue")
            },
            x if x == Self::SchedulerConcurrentCommit.0 => {
                write!(f, "SchedulerCommitOutcome::SchedulerConcurrentCommit")
            },
            _ => {
                write!(f, "SchedulerCommitOutcome({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerGetTipOutcome(pub i32);

impl SchedulerGetTipOutcome {
    pub const SchedulerGetTipOK: Self = Self(0);
    pub const SchedulerGetTipNoGenesisBlock: Self = Self(1);
}
impl From<i32> for SchedulerGetTipOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedulerGetTipOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedulerGetTipOK.0 => {
                write!(f, "SchedulerGetTipOutcome::SchedulerGetTipOK")
            },
            x if x == Self::SchedulerGetTipNoGenesisBlock.0 => {
                write!(f, "SchedulerGetTipOutcome::SchedulerGetTipNoGenesisBlock")
            },
            _ => {
                write!(f, "SchedulerGetTipOutcome({})", self.0)
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
pub enum SchedulerGetTipResult {
    // case SchedulerGetTipOK:
    //   DbTip db_tip;
    SchedulerGetTipOK { db_tip: DbTip } = SchedulerGetTipOutcome::SchedulerGetTipOK.0 as i32,
    // case SchedulerGetTipNoGenesisBlock:
    //   void;
    SchedulerGetTipNoGenesisBlock = SchedulerGetTipOutcome::SchedulerGetTipNoGenesisBlock.0 as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct GenesisCeremonyRequest {
    pub expected_parent_height: BlockHeight,
    pub expected_parent_value_hash: ConsensusValueHash,
    pub context: ProposalContext,
    pub transactions: ParallelSignedTransactionBatch,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct GenesisCeremonyOutcome(pub i32);

impl GenesisCeremonyOutcome {
    pub const GenesisCeremonyCommitted: Self = Self(0);
    pub const GenesisCeremonyParentMismatch: Self = Self(2);
    pub const GenesisCeremonyBatchMismatch: Self = Self(3);
    pub const GenesisCeremonyInvalidTransaction: Self = Self(4);
    pub const GenesisCeremonyExecutionFailed: Self = Self(5);
}
impl From<i32> for GenesisCeremonyOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for GenesisCeremonyOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::GenesisCeremonyCommitted.0 => {
                write!(f, "GenesisCeremonyOutcome::GenesisCeremonyCommitted")
            },
            x if x == Self::GenesisCeremonyParentMismatch.0 => {
                write!(f, "GenesisCeremonyOutcome::GenesisCeremonyParentMismatch")
            },
            x if x == Self::GenesisCeremonyBatchMismatch.0 => {
                write!(f, "GenesisCeremonyOutcome::GenesisCeremonyBatchMismatch")
            },
            x if x == Self::GenesisCeremonyInvalidTransaction.0 => {
                write!(f, "GenesisCeremonyOutcome::GenesisCeremonyInvalidTransaction")
            },
            x if x == Self::GenesisCeremonyExecutionFailed.0 => {
                write!(f, "GenesisCeremonyOutcome::GenesisCeremonyExecutionFailed")
            },
            _ => {
                write!(f, "GenesisCeremonyOutcome({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct GenesisCeremonyCommit {
    pub db_tip: DbTip,
    pub proposal_txs: ProposalTxs,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerTransactionStatus(pub i32);

impl SchedulerTransactionStatus {
    // The transaction is not pending and has no retained outcome. Note that the
    // tx *may* have existed at some point in the past, but has since been dropped
    // altogether from the mempool since the bounded queue filled. This not-found
    // status does not necessarily mean the tx has never existed at all.
    pub const SchedulerTransactionNotFound: Self = Self(0);

    // The transaction is currently pending in the mempool.
    pub const SchedulerTransactionPending: Self = Self(1);

    // The transaction was successfully committed in a block. This status does
    // *not* mean that the tx was successfully included in a proposed block or a
    // verified block--the block needs to go on to actually be committed.
    pub const SchedulerTransactionSucceeded: Self = Self(2);

    // Processing failed for a retryable reason; the transaction may be
    // resubmitted. Note that it is the responsibility of the wallet owner to
    // resubmit the tx--the mempool will not retry the tx on its own.
    pub const SchedulerTransactionRetry: Self = Self(3);

    // Processing failed permanently; the transaction should not be resubmitted
    // because the error is irrecoverable (e.g., failed authentication). The tx
    // needs to be modified in some way before it can be successfully processed.
    pub const SchedulerTransactionDropped: Self = Self(4);
}
impl From<i32> for SchedulerTransactionStatus {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedulerTransactionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedulerTransactionNotFound.0 => {
                write!(f, "SchedulerTransactionStatus::SchedulerTransactionNotFound")
            },
            x if x == Self::SchedulerTransactionPending.0 => {
                write!(f, "SchedulerTransactionStatus::SchedulerTransactionPending")
            },
            x if x == Self::SchedulerTransactionSucceeded.0 => {
                write!(f, "SchedulerTransactionStatus::SchedulerTransactionSucceeded")
            },
            x if x == Self::SchedulerTransactionRetry.0 => {
                write!(f, "SchedulerTransactionStatus::SchedulerTransactionRetry")
            },
            x if x == Self::SchedulerTransactionDropped.0 => {
                write!(f, "SchedulerTransactionStatus::SchedulerTransactionDropped")
            },
            _ => {
                write!(f, "SchedulerTransactionStatus({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerTransactionErrorSource(pub i32);

impl SchedulerTransactionErrorSource {
    // There is no error to report.
    pub const SchedulerTransactionErrorNone: Self = Self(0);

    // The error was triggered by the authentication subsystem (hog-auth).
    pub const SchedulerTransactionErrorAuth: Self = Self(1);

    // The error was triggered by the mempool (hog-mempool).
    pub const SchedulerTransactionErrorScheduler: Self = Self(2);

    // The error was triggered by an executor (hog-exec).
    pub const SchedulerTransactionErrorExecutor: Self = Self(3);
}
impl From<i32> for SchedulerTransactionErrorSource {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedulerTransactionErrorSource {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedulerTransactionErrorNone.0 => {
                write!(f, "SchedulerTransactionErrorSource::SchedulerTransactionErrorNone")
            },
            x if x == Self::SchedulerTransactionErrorAuth.0 => {
                write!(f, "SchedulerTransactionErrorSource::SchedulerTransactionErrorAuth")
            },
            x if x == Self::SchedulerTransactionErrorScheduler.0 => {
                write!(f, "SchedulerTransactionErrorSource::SchedulerTransactionErrorScheduler")
            },
            x if x == Self::SchedulerTransactionErrorExecutor.0 => {
                write!(f, "SchedulerTransactionErrorSource::SchedulerTransactionErrorExecutor")
            },
            _ => {
                write!(f, "SchedulerTransactionErrorSource({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerErrorCode(pub i32);

impl SchedulerErrorCode {
    // The mempool had no capacity to admit the transaction.
    pub const SchedulerErrorCapacity: Self = Self(1);

    // The scheduler could not obtain the account's average native-token age.
    pub const SchedulerErrorNativeTokenAgeUnavailable: Self = Self(2);

    // The transaction's expiry block height is earlier than the current block.
    pub const SchedulerErrorExpired: Self = Self(3);

    // The transaction's gas limit exceeds the account's available native tokens.
    pub const SchedulerErrorInsufficientNativeTokens: Self = Self(4);
}
impl From<i32> for SchedulerErrorCode {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedulerErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedulerErrorCapacity.0 => {
                write!(f, "SchedulerErrorCode::SchedulerErrorCapacity")
            },
            x if x == Self::SchedulerErrorNativeTokenAgeUnavailable.0 => {
                write!(f, "SchedulerErrorCode::SchedulerErrorNativeTokenAgeUnavailable")
            },
            x if x == Self::SchedulerErrorExpired.0 => {
                write!(f, "SchedulerErrorCode::SchedulerErrorExpired")
            },
            x if x == Self::SchedulerErrorInsufficientNativeTokens.0 => {
                write!(f, "SchedulerErrorCode::SchedulerErrorInsufficientNativeTokens")
            },
            _ => {
                write!(f, "SchedulerErrorCode({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SchedulerTransactionOutcome {
    // The tx status. If the status is an error, then [error_source] and
    // [error_code] are filled in below. If the status is not an error, then
    // [error_source] and [error_code] have unspecified values.
    pub status: SchedulerTransactionStatus,
    // Identifies the enum represented by [error_code] below.
    //
    // - None ([SchedulerTransactionErrorNone]): there was no error and
    //   [error_code] has an unspecified value.
    //
    // - Auth, Scheduler, and Executor: [error_code] is an [AuthErrorCode],
    //   [SchedulerErrorCode], or [ExecResult], respectively.
    pub error_source: SchedulerTransactionErrorSource,
    // The actual error.
    pub error_code: i32,
    // For [SchedulerTransactionSucceeded], the committed block height.
    // For [SchedulerTransactionRetry] or [SchedulerTransactionDropped], the
    // proposal height for an executor failure, or the current chain height for
    // an admission/scheduler failure.
    // For [SchedulerTransactionPending], the current chain height at lookup.
    // Unspecified for [SchedulerTransactionNotFound].
    // Zero is a valid height.
    pub height: BlockHeight,
}

pub type SchedulerTransactionOutcomeBatch = VecM::<SchedulerTransactionOutcome>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum GenesisCeremonyResult {
    // case GenesisCeremonyCommitted:
    //   GenesisCeremonyCommit commit;
    GenesisCeremonyCommitted { commit: GenesisCeremonyCommit } = GenesisCeremonyOutcome::GenesisCeremonyCommitted.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { outcome: i32 },
}


/// Each handler replies with anything that encodes like the reply type.
/// Implementations can be plain `async fn`s returning a concrete reply type
/// under `#[allow(refining_impl_trait)]`.
#[allow(non_camel_case_types)]
pub trait SchedulerCoord_SchedulerCoord1: Send + Sync {    
    fn schedulercoord_null(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn schedulercoord_intro_coord(&self, arg0 : CoordCard) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<CoordinatorRegistrationOutcome>, ::thruput_rpc_api::ProgramError>> + Send;
        // Coordinator calls schedulercoord_submit_unvetted to submit new/unvetted
    // transactions coming from the wallet. Asynchronously, the scheduler will
    // call the coordinator's coordscheduler_notify_vetted to notify the
    // coordinator when transactions have become vetted and can be broadcast
    // to the network.

    fn schedulercoord_submit_unvetted(&self, arg0 : ParallelSignedTransactionBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn schedulercoord_submit_vetted(&self, arg0 : ValidatorID, arg1 : ParallelSignedTransactionBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<()>, ::thruput_rpc_api::ProgramError>> + Send;
        // schedulercoord_propose asks the scheduler to prepare a proposal (a
    // block proposal) on top of the given db block.

    fn schedulercoord_propose_block(&self, arg0 : BlockID, arg1 : ProposalContext) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SchedulerProposalResult>, ::thruput_rpc_api::ProgramError>> + Send;
        // schedulercoord_validate asks the scheduler to validate a proposal,
    // and if successful, return the block ID in the db of the validated
    // block. BlockID is the block onto which the proposal is done.

    fn schedulercoord_validate_block(&self, arg0 : BlockID, arg1 : Proposal) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SchedulerValidationResult>, ::thruput_rpc_api::ProgramError>> + Send;
        // Commit the block with block ID. This block must have been previously
    // proposed or validated by the mempool.
    // On commit the block is associated with the ConsensusValue that confirmed it.
    // This value is saved in the database and retrievable via schedcoord_get_tip.

    fn schedulercoord_commit_block(&self, arg0 : BlockID, arg1 : ConsensusValue) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SchedulerCommitOutcome>, ::thruput_rpc_api::ProgramError>> + Send;
        // Returns information about the current highest committed block.
    // If there is no committed block, returns SchedulerGetTipNoGenesisBlock.

    fn schedcoord_get_tip(&self) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SchedulerGetTipResult>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn schedcoord_load_genesis_configuration(&self, arg0 : GenesisConfiguration) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<DbSchedResult>, ::thruput_rpc_api::ProgramError>> + Send;
    
    fn schedulercoord_commit_genesis_ceremony(&self, arg0 : GenesisCeremonyRequest) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<GenesisCeremonyResult>, ::thruput_rpc_api::ProgramError>> + Send;
        // Returns transactions from the mempool that have not yet been committed in a block,
    // by their exact signed encoding.

    fn schedcoord_lookup_transactions(&self, arg0 : SignedTransactionHashBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<OptSignedTransactionBatch>, ::thruput_rpc_api::ProgramError>> + Send;
        // Returns one pending or recent outcome per input hash (same order).

    fn schedcoord_lookup_transaction_outcomes(&self, arg0 : TransactionHashBatch) -> impl ::core::future::Future<Output = Result<impl xdrs::EncodesAs<SchedulerTransactionOutcomeBatch>, ::thruput_rpc_api::ProgramError>> + Send;
}


#[allow(non_camel_case_types)]
pub struct SchedulerCoord_SchedulerCoord1_Dispatcher<__Handler> {
  inner: Box<__Handler>,
}

impl<__Handler: SchedulerCoord_SchedulerCoord1> SchedulerCoord_SchedulerCoord1_Dispatcher<__Handler> {
  pub fn new(inner: Box<__Handler>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl<__Handler: SchedulerCoord_SchedulerCoord1> ::thruput_rpc_api::RpcHandler for SchedulerCoord_SchedulerCoord1_Dispatcher<__Handler> {
    fn prog_vers(&self) -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x76671111, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::thruput_rpc_api::RpcError> {
        match proc {
            // schedulercoord_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_null().await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedulercoord_intro_coord
            1 => {
                let args: (CoordCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_intro_coord(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedulercoord_submit_unvetted
            2 => {
                let args: (ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_submit_unvetted(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedulercoord_submit_vetted
            3 => {
                let args: (ValidatorID, ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_submit_vetted(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedulercoord_propose_block
            4 => {
                let args: (BlockID, ProposalContext, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_propose_block(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedulercoord_validate_block
            5 => {
                let args: (BlockID, Proposal, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_validate_block(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedulercoord_commit_block
            6 => {
                let args: (BlockID, ConsensusValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_commit_block(args.0, args.1).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedcoord_get_tip
            7 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_get_tip().await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedcoord_load_genesis_configuration
            8 => {
                let args: (GenesisConfiguration, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_load_genesis_configuration(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedulercoord_commit_genesis_ceremony
            10 => {
                let args: (GenesisCeremonyRequest, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_commit_genesis_ceremony(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedcoord_lookup_transactions
            100 => {
                let args: (SignedTransactionHashBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_lookup_transactions(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            // schedcoord_lookup_transaction_outcomes
            101 => {
                let args: (TransactionHashBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_lookup_transaction_outcomes(args.0).await.map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)
            }
            _ => Err(::thruput_rpc_api::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SchedulerCoord_SchedulerCoord1_Client {
    handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>,
}

impl SchedulerCoord_SchedulerCoord1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::thruput_rpc_api::ProgVers {
        ::thruput_rpc_api::ProgVers::new(0x76671111, 1)
    }
    pub async fn schedulercoord_null(&self) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 0, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedulercoord_intro_coord(&self, arg0 : &(impl xdrs::EncodesAs<CoordCard> + ?Sized)) -> Result<CoordinatorRegistrationOutcome, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 1, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedulercoord_submit_unvetted(&self, arg0 : &(impl xdrs::EncodesAs<ParallelSignedTransactionBatch> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 2, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedulercoord_submit_vetted(&self, arg0 : &(impl xdrs::EncodesAs<ValidatorID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ParallelSignedTransactionBatch> + ?Sized)) -> Result<(), ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 3, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedulercoord_propose_block(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ProposalContext> + ?Sized)) -> Result<SchedulerProposalResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 4, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedulercoord_validate_block(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<Proposal> + ?Sized)) -> Result<SchedulerValidationResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 5, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedulercoord_commit_block(&self, arg0 : &(impl xdrs::EncodesAs<BlockID> + ?Sized), arg1 : &(impl xdrs::EncodesAs<ConsensusValue> + ?Sized)) -> Result<SchedulerCommitOutcome, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0, arg1), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 6, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedcoord_get_tip(&self) -> Result<SchedulerGetTipResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 7, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedcoord_load_genesis_configuration(&self, arg0 : &(impl xdrs::EncodesAs<GenesisConfiguration> + ?Sized)) -> Result<DbSchedResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 8, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedulercoord_commit_genesis_ceremony(&self, arg0 : &(impl xdrs::EncodesAs<GenesisCeremonyRequest> + ?Sized)) -> Result<GenesisCeremonyResult, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 10, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedcoord_lookup_transactions(&self, arg0 : &(impl xdrs::EncodesAs<SignedTransactionHashBatch> + ?Sized)) -> Result<OptSignedTransactionBatch, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 100, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
    pub async fn schedcoord_lookup_transaction_outcomes(&self, arg0 : &(impl xdrs::EncodesAs<TransactionHashBatch> + ?Sized)) -> Result<SchedulerTransactionOutcomeBatch, ::thruput_rpc_api::RpcError> {
         let mut call = self.handle.call_buf();
         call.append_with(|buf| {
             let mut w = xdrs::txs::buf_writer::BufWriter::from(::core::mem::take(buf));
             let r = xdrs::write_xdr(&(arg0), &mut w);
             *buf = w.into();
             r
         }).map_err(|_| ::thruput_rpc_api::RpcError::GarbageArguments)?;
         let mut out = ::core::option::Option::None;
         let mut decode = |reply: &[u8]| -> ::core::result::Result<(), ::thruput_rpc_api::RpcError> {
             out = ::core::option::Option::Some(xdrs::txs::decode(reply).map_err(|_| ::thruput_rpc_api::RpcError::SystemError)?);
             Ok(())
         };
         // SAFETY: the call future is awaited right here, so it lives in
         // this method's future along with `out` and `decode`. Once that
         // is polled it is pinned, so neither can be freed without the
         // call future being dropped first. Decoding never blocks.
         unsafe { self.handle.send_call_decoded(Self::prog_vers(), 101, call, &mut decode) }.await?;
         out.ok_or(::thruput_rpc_api::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>> for SchedulerCoord_SchedulerCoord1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::thruput_rpc_api::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl SchedulerProposalResult {
    #[allow(clippy::match_single_binding)]
    pub fn outcome(&self) -> SchedulerProposalOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl SchedulerValidationResult {
    #[allow(clippy::match_single_binding)]
    pub fn outcome(&self) -> SchedulerValidationOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl SchedulerGetTipResult {
    #[allow(clippy::match_single_binding)]
    pub fn outcome(&self) -> SchedulerGetTipOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl GenesisCeremonyResult {
    #[allow(clippy::match_single_binding)]
    pub fn outcome(&self) -> GenesisCeremonyOutcome {
        match self {
            GenesisCeremonyResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


