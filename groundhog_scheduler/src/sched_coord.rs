// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: a3b921b9de2770b9d2cf4663589132ec222ce8f2

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
#[derive(ReadXDR, WriteXDR)]

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
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CoordCard {
    pub protocol: Protocol,
    pub address_for_sched: ServerAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerNominationOutcome(pub i32);

impl SchedulerNominationOutcome {
    pub const SchedulerNominationOK: Self = Self(0);
    pub const SchedulerNominationError: Self = Self(1);
}
impl From<i32> for SchedulerNominationOutcome {
    fn from(v: i32) -> Self {
        Self(v)
    }
}
#[cfg(feature = "std")]
impl core::fmt::Display for SchedulerNominationOutcome {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            x if x == Self::SchedulerNominationOK.0 => {
                write!(f, "SchedulerNominationOutcome::SchedulerNominationOK")
            },
            x if x == Self::SchedulerNominationError.0 => {
                write!(f, "SchedulerNominationOutcome::SchedulerNominationError")
            },
            _ => {
                write!(f, "SchedulerNominationOutcome({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SchedulerNominatedBlock {
    pub block_id: BlockID,
    pub txs: ParallelSignedTransactionBatch,
    pub nomination: Nomination,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum SchedulerNominationResult {
    // case SchedulerNominationOK:
    //   SchedulerNominatedBlock nominated_block;
    SchedulerNominationOK { nominated_block: SchedulerNominatedBlock } = SchedulerNominationOutcome::SchedulerNominationOK.0 as i32,
    // case SchedulerNominationError:
    //   void;
    SchedulerNominationError = SchedulerNominationOutcome::SchedulerNominationError.0 as i32,
    
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

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

    // Nomination batch metadata is malformed or fails to decode (e.g.
    // segment token/byte counts are inconsistent, XDR deserialization
    // fails, or tx_hashes/tx_nominations counts do not match).
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
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SchedulerValidationBlock {
    pub db_tip: DbTip,
    pub signed_transactions: ParallelSignedTransactionBatch,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum SchedulerValidationResult {
    // case SchedulerValidationOK:
    //   SchedulerValidationBlock block;
    SchedulerValidationOK { block: SchedulerValidationBlock } = SchedulerValidationOutcome::SchedulerValidationOK.0 as i32,
    // case SchedulerValidationMissingTxs:
    //   VecM<SignedTransactionHash> missing_tx_hashes;
    SchedulerValidationMissingTxs { missing_tx_hashes: VecM<SignedTransactionHash> } = SchedulerValidationOutcome::SchedulerValidationMissingTxs.0 as i32,
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
#[derive(ReadXDR, WriteXDR)]

#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(transparent)]
pub struct SchedulerCommitOutcome(pub i32);

impl SchedulerCommitOutcome {
    pub const SchedulerCommitOK: Self = Self(0);
    pub const SchedulerCommitBlockNotFound: Self = Self(1);
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
            _ => {
                write!(f, "SchedulerCommitOutcome({})", self.0)
            }
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(ReadXDR, WriteXDR)]

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
#[derive(ReadXDR, WriteXDR)]

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
        // schedulercoord_nominate asks the scheduler to prepare a nomination (a
    // block proposal) on top of the given db block.

    async fn schedulercoord_nominate(&self, arg0 : DbTip) -> Result<SchedulerNominationResult, ::rpc_driver::ProgramError>;
        // schedulercoord_validate asks the scheduler to validate a nomination,
    // and if successful, return the block ID in the db of the validated
    // block. DbTip is the block onto which the nomination is done.

    async fn schedulercoord_validate(&self, arg0 : DbTip, arg1 : Nomination) -> Result<SchedulerValidationResult, ::rpc_driver::ProgramError>;
        // Commit the block with block ID. This block must have been previously
    // nominated or validated by the mempool.

    async fn schedulercoord_commit_block(&self, arg0 : BlockID) -> Result<SchedulerCommitOutcome, ::rpc_driver::ProgramError>;
        // Returns information about the current highest committed block.
    // If there is no committed block, returns SchedulerGetTipNoGenesisBlock.

    async fn schedcoord_get_tip(&self) -> Result<SchedulerGetTipResult, ::rpc_driver::ProgramError>;
        // TODO(jhumphri): Remove this once we integrate the coordinator into the
    // loadgen experiment.

    async fn schedcoord_load_genesis_configuration(&self, arg0 : GenesisConfiguration) -> Result<DbSchedResult, ::rpc_driver::ProgramError>;
        // Returns transactions from the mempool that have not yet been committed in a block.

    async fn schedcoord_lookup_transactions(&self, arg0 : SignedTransactionHashBatch) -> Result<OptSignedTransactionBatch, ::rpc_driver::ProgramError>;
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
            // schedulercoord_nominate
            4 => {
                let args: (DbTip, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_nominate(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_validate
            5 => {
                let args: (DbTip, Nomination, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_validate(args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_commit_block
            6 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_commit_block(args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
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
            // schedcoord_lookup_transactions
            100 => {
                let args: (SignedTransactionHashBatch, ) =
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
    pub async fn schedulercoord_nominate(&self, arg0 : &DbTip) -> Result<SchedulerNominationResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 4, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_validate(&self, arg0 : &DbTip, arg1 : &Nomination) -> Result<SchedulerValidationResult, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0, &arg1)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 5, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
    pub async fn schedulercoord_commit_block(&self, arg0 : &BlockID) -> Result<SchedulerCommitOutcome, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
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
    pub async fn schedcoord_lookup_transactions(&self, arg0 : &SignedTransactionHashBatch) -> Result<OptSignedTransactionBatch, ::rpc_driver::RpcError> {
         let data = xdrs::txs::encode(&(&arg0)).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
         let buf = self.handle.send_call(Self::prog_vers(), 100, &data).await?;
         xdrs::txs::decode(&buf).map_err(|_| ::rpc_driver::RpcError::SystemError)
    }
}



//
// XDR internals below this point
//

impl SchedulerNominationResult {
    pub fn outcome(&self) -> SchedulerNominationOutcome {
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


