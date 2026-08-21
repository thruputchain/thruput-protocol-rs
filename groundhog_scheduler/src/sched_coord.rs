// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 003a0d33894e4c7c48474454948073bd6d10da45

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
    // fails, or tx_hashes/tx_nondeterministic counts do not match).
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
    // case SchedulerValidationMissingTxs:
    //   VecM<TransactionHash> missing_tx_hashes;
    SchedulerValidationMissingTxs { missing_tx_hashes: VecM<TransactionHash> } = SchedulerValidationOutcome::SchedulerValidationMissingTxs.0 as i32,
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


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SchedulerCoord_SchedulerCoord1: Send + Sync {    
    async fn schedulercoord_null(&self) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn schedulercoord_intro_coord(&self, arg0 : CoordCard) -> Result<CoordinatorRegistrationOutcome, ::rpc_driver::ProgramError>;
        // Coordinator calls schedulercoord_submit_unvetted to submit new/unvetted
    // transactions coming from the wallet. Asynchronously, the scheduler will
    // call the coordinator's coordscheduler_notify_vetted to notify the
    // coordinator when transactions have become vetted and can be broadcast
    // to the network.

    async fn schedulercoord_submit_unvetted(&self, arg0 : ParallelSignedTransactionBatch) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn schedulercoord_submit_vetted(&self, arg0 : ValidatorID, arg1 : ParallelSignedTransactionBatch) -> Result<(), ::rpc_driver::ProgramError>;
        // schedulercoord_propose asks the scheduler to prepare a proposal (a
    // block proposal) on top of the given db block.

    async fn schedulercoord_propose_block(&self, arg0 : BlockID, arg1 : ProposalContext) -> Result<SchedulerProposalResult, ::rpc_driver::ProgramError>;
        // schedulercoord_validate asks the scheduler to validate a proposal,
    // and if successful, return the block ID in the db of the validated
    // block. BlockID is the block onto which the proposal is done.

    async fn schedulercoord_validate_block(&self, arg0 : BlockID, arg1 : Proposal) -> Result<SchedulerValidationResult, ::rpc_driver::ProgramError>;
        // Commit the block with block ID. This block must have been previously
    // proposed or validated by the mempool.
    // On commit the block is associated with the ConsensusValue that confirmed it.
    // This value is saved in the database and retrievable via schedcoord_get_tip.

    async fn schedulercoord_commit_block(&self, arg0 : BlockID, arg1 : ConsensusValue) -> Result<SchedulerCommitOutcome, ::rpc_driver::ProgramError>;
        // Returns information about the current highest committed block.
    // If there is no committed block, returns SchedulerGetTipNoGenesisBlock.

    async fn schedcoord_get_tip(&self) -> Result<SchedulerGetTipResult, ::rpc_driver::ProgramError>;
        // TODO(jhumphri): Remove this once we integrate the coordinator into the
    // loadgen experiment.

    async fn schedcoord_load_genesis_configuration(&self, arg0 : GenesisConfiguration) -> Result<DbSchedResult, ::rpc_driver::ProgramError>;
    
    async fn schedcoord_load_genesis_configuration_v2(&self, arg0 : GenesisConfigurationV2) -> Result<DbSchedResult, ::rpc_driver::ProgramError>;
    
    async fn schedulercoord_commit_genesis_ceremony(&self, arg0 : GenesisCeremonyRequest) -> Result<GenesisCeremonyResult, ::rpc_driver::ProgramError>;
        // Returns transactions from the mempool that have not yet been committed in a block.

    async fn schedcoord_lookup_transactions(&self, arg0 : TransactionHashBatch) -> Result<OptSignedTransactionBatch, ::rpc_driver::ProgramError>;
}


#[allow(non_camel_case_types)]
pub struct SchedulerCoord_SchedulerCoord1_Dispatcher {
  inner: Box<dyn SchedulerCoord_SchedulerCoord1>,
}

impl SchedulerCoord_SchedulerCoord1_Dispatcher {
  pub fn new(inner: Box<dyn SchedulerCoord_SchedulerCoord1>) -> Self {
    Self { inner }
  }
}

#[async_trait::async_trait]
impl ::rpc_driver::handler::RpcHandler for SchedulerCoord_SchedulerCoord1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x76671111, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8]) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // schedulercoord_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_null().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_intro_coord
            1 => {
                let args: (CoordCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_intro_coord(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_submit_unvetted
            2 => {
                let args: (ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_submit_unvetted(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_submit_vetted
            3 => {
                let args: (ValidatorID, ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_submit_vetted(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_propose_block
            4 => {
                let args: (BlockID, ProposalContext, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_propose_block(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_validate_block
            5 => {
                let args: (BlockID, Proposal, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_validate_block(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_commit_block
            6 => {
                let args: (BlockID, ConsensusValue, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_commit_block(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedcoord_get_tip
            7 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_get_tip().await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedcoord_load_genesis_configuration
            8 => {
                let args: (GenesisConfiguration, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_load_genesis_configuration(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedcoord_load_genesis_configuration_v2
            9 => {
                let args: (GenesisConfigurationV2, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_load_genesis_configuration_v2(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_commit_genesis_ceremony
            10 => {
                let args: (GenesisCeremonyRequest, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_commit_genesis_ceremony(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedcoord_lookup_transactions
            100 => {
                let args: (TransactionHashBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_lookup_transactions(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone)]
pub struct SchedulerCoord_SchedulerCoord1_Client {
    handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>,
}

impl SchedulerCoord_SchedulerCoord1_Client {
    pub fn new(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self { handle }
    }

    fn prog_vers() -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x76671111, 1)
    }
    pub async fn schedulercoord_null(&self) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 0, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_intro_coord(&self, arg0 : &CoordCard) -> Result<CoordinatorRegistrationOutcome, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 1, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_submit_unvetted(&self, arg0 : &ParallelSignedTransactionBatch) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 2, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_submit_vetted(&self, arg0 : &ValidatorID, arg1 : &ParallelSignedTransactionBatch) -> Result<(), ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 3, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_propose_block(&self, arg0 : &BlockID, arg1 : &ProposalContext) -> Result<SchedulerProposalResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 4, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_validate_block(&self, arg0 : &BlockID, arg1 : &Proposal) -> Result<SchedulerValidationResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 5, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_commit_block(&self, arg0 : &BlockID, arg1 : &ConsensusValue) -> Result<SchedulerCommitOutcome, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 6, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedcoord_get_tip(&self) -> Result<SchedulerGetTipResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&()).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 7, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedcoord_load_genesis_configuration(&self, arg0 : &GenesisConfiguration) -> Result<DbSchedResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 8, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedcoord_load_genesis_configuration_v2(&self, arg0 : &GenesisConfigurationV2) -> Result<DbSchedResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 9, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_commit_genesis_ceremony(&self, arg0 : &GenesisCeremonyRequest) -> Result<GenesisCeremonyResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 10, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedcoord_lookup_transactions(&self, arg0 : &TransactionHashBatch) -> Result<OptSignedTransactionBatch, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}

impl From<::std::sync::Arc<dyn ::rpc_driver::RpcSender>> for SchedulerCoord_SchedulerCoord1_Client {
    fn from(handle: ::std::sync::Arc<dyn ::rpc_driver::RpcSender>) -> Self {
        Self::new(handle)
    }
}



//
// XDR internals below this point
//

impl SchedulerProposalResult {
    pub fn outcome(&self) -> SchedulerProposalOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl SchedulerValidationResult {
    pub fn outcome(&self) -> SchedulerValidationOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl SchedulerGetTipResult {
    pub fn outcome(&self) -> SchedulerGetTipOutcome {
        match self {
            
            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl GenesisCeremonyResult {
    pub fn outcome(&self) -> GenesisCeremonyOutcome {
        match self {
            GenesisCeremonyResult::Default{ outcome } => (*outcome).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


