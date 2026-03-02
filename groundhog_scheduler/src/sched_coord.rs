// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 0045f3f59b643b1d543cb618c1c14185fab73435

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
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum CoordinatorRegistrationOutcome {
    CoordinatorRegistrationOK = 0 as i32,
    CoordinatorRegistrationInvalidProtocol = 1 as i32,
}
impl TryFrom<i32> for CoordinatorRegistrationOutcome {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::CoordinatorRegistrationOK),
            1 => Ok(Self::CoordinatorRegistrationInvalidProtocol),
            _ => Err(()),
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct CoordCard {
    pub protocol: Protocol,
    pub address_for_sched: ServerAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum SchedulerNominationOutcome {
    SchedulerNominationOK = 0 as i32,
    SchedulerNominationError = 1 as i32,
}
impl TryFrom<i32> for SchedulerNominationOutcome {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SchedulerNominationOK),
            1 => Ok(Self::SchedulerNominationError),
            _ => Err(()),
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

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

#[repr(i32)]
pub enum SchedulerNominationResult {
    // case SchedulerNominationOK:
    //   SchedulerNominatedBlock nominated_block;
    SchedulerNominationOK { nominated_block: SchedulerNominatedBlock } = SchedulerNominationOutcome::SchedulerNominationOK as i32,
    // default:
    //   void;
    Default { outcome: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum SchedulerValidationOutcome {
    SchedulerValidationOK = 0 as i32,
    SchedulerValidationInvalid = 1 as i32,
    SchedulerValidationMissingTxs = 2 as i32,
    SchedulerValidationDbFailed = 3 as i32,
    SchedulerValidationExecutorFailed = 4 as i32,
}
impl TryFrom<i32> for SchedulerValidationOutcome {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SchedulerValidationOK),
            1 => Ok(Self::SchedulerValidationInvalid),
            2 => Ok(Self::SchedulerValidationMissingTxs),
            3 => Ok(Self::SchedulerValidationDbFailed),
            4 => Ok(Self::SchedulerValidationExecutorFailed),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum SchedulerValidationResult {
    // case SchedulerValidationOK:
    //   DbTip db_tip;
    SchedulerValidationOK { db_tip: DbTip } = SchedulerValidationOutcome::SchedulerValidationOK as i32,
    // case SchedulerValidationMissingTxs:
    //   VecM<SignedTransactionHash> missing_tx_hashes;
    SchedulerValidationMissingTxs { missing_tx_hashes: VecM<SignedTransactionHash> } = SchedulerValidationOutcome::SchedulerValidationMissingTxs as i32,
    // default:
    //   void;
    Default { outcome: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum SchedulerCommitOutcome {
    SchedulerCommitOK = 0 as i32,
    SchedulerCommitBlockNotFound = 1 as i32,
    SchedulerCommitRace = 2 as i32,
}
impl TryFrom<i32> for SchedulerCommitOutcome {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SchedulerCommitOK),
            1 => Ok(Self::SchedulerCommitBlockNotFound),
            2 => Ok(Self::SchedulerCommitRace),
            _ => Err(()),
        }
    }
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[derive(Copy)]
#[repr(i32)]
pub enum SchedulerGetTipOutcome {
    SchedulerGetTipOK = 0 as i32,
    SchedulerGetTipNoGenesisBlock = 1 as i32,
}
impl TryFrom<i32> for SchedulerGetTipOutcome {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::SchedulerGetTipOK),
            1 => Ok(Self::SchedulerGetTipNoGenesisBlock),
            _ => Err(()),
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR)]

#[repr(i32)]
pub enum SchedulerGetTipResult {
    // case SchedulerGetTipOK:
    //   DbTip db_tip;
    SchedulerGetTipOK { db_tip: DbTip } = SchedulerGetTipOutcome::SchedulerGetTipOK as i32,
    // case SchedulerGetTipNoGenesisBlock:
    //   void;
    SchedulerGetTipNoGenesisBlock = SchedulerGetTipOutcome::SchedulerGetTipNoGenesisBlock as i32,
    // default:
    //   void;
    Default { outcome: i32 },
}


#[allow(non_camel_case_types)]
#[async_trait::async_trait]
pub trait SchedulerCoord_SchedulerCoord1: Send + Sync {    
    async fn schedulercoord_null(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn schedulercoord_intro_coord(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : CoordCard) -> Result<CoordinatorRegistrationOutcome, ::rpc_driver::ProgramError>;
        // Coordinator calls schedulercoord_submit_unvetted to submit new/unvetted
    // transactions coming from the wallet. Asynchronously, the scheduler will
    // call the coordinator's coordscheduler_notify_vetted to notify the
    // coordinator when transactions have become vetted and can be broadcast
    // to the network.

    async fn schedulercoord_submit_unvetted(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : ParallelSignedTransactionBatch) -> Result<(), ::rpc_driver::ProgramError>;
    
    async fn schedulercoord_submit_vetted(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : ValidatorID, arg1 : ParallelSignedTransactionBatch) -> Result<(), ::rpc_driver::ProgramError>;
        // schedulercoord_nominate asks the scheduler to prepare a nomination (a
    // block proposal) on top of the given db block.

    async fn schedulercoord_nominate(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : DbTip) -> Result<SchedulerNominationResult, ::rpc_driver::ProgramError>;
        // schedulercoord_validate asks the scheduler to validate a nomination,
    // and if successful, return the block ID in the db of the validated
    // block. DbTip is the block onto which the nomination is done.

    async fn schedulercoord_validate(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : DbTip, arg1 : Nomination) -> Result<SchedulerValidationResult, ::rpc_driver::ProgramError>;
        // Commit the block with block ID. This block must have been previously
    // nominated or validated by the mempool.

    async fn schedulercoord_commit_block(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : BlockID) -> Result<SchedulerCommitOutcome, ::rpc_driver::ProgramError>;
        // Returns information about the current highest committed block.
    // If there is no committed block, returns SchedulerGetTipNoGenesisBlock.

    async fn schedcoord_get_tip(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>) -> Result<SchedulerGetTipResult, ::rpc_driver::ProgramError>;
        // TODO(jhumphri): Remove this once we integrate the coordinator into the
    // loadgen experiment.

    async fn schedcoord_load_genesis_configuration(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : GenesisConfiguration) -> Result<DbSchedResult, ::rpc_driver::ProgramError>;
        // Returns transactions from the mempool that have not yet been committed in a block.

    async fn schedcoord_lookup_transactions(&self, client: &::std::sync::Arc<::rpc_driver::ClientHandle>, arg0 : SignedTransactionHashBatch) -> Result<OptSignedTransactionBatch, ::rpc_driver::ProgramError>;
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
impl ::rpc_driver::ServerDispatch for SchedulerCoord_SchedulerCoord1_Dispatcher {
    fn prog_vers(&self) -> ::rpc_driver::ProgVers {
        ::rpc_driver::ProgVers::new(0x76671111, 1)
    }
    async fn dispatch(&self, proc: u32, data: &[u8], client: &::std::sync::Arc<::rpc_driver::ClientHandle>) -> Result<Vec<u8>, ::rpc_driver::RpcError> {
        match proc {
            // schedulercoord_null
            0 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_null(client).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_intro_coord
            1 => {
                let args: (CoordCard, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_intro_coord(client, args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_submit_unvetted
            2 => {
                let args: (ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_submit_unvetted(client, args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_submit_vetted
            3 => {
                let args: (ValidatorID, ParallelSignedTransactionBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_submit_vetted(client, args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_nominate
            4 => {
                let args: (DbTip, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_nominate(client, args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_validate
            5 => {
                let args: (DbTip, Nomination, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_validate(client, args.0, args.1).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedulercoord_commit_block
            6 => {
                let args: (BlockID, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedulercoord_commit_block(client, args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedcoord_get_tip
            7 => {
                let args: () =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_get_tip(client).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedcoord_load_genesis_configuration
            8 => {
                let args: (GenesisConfiguration, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_load_genesis_configuration(client, args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            // schedcoord_lookup_transactions
            100 => {
                let args: (SignedTransactionHashBatch, ) =
                    xdrs::txs::decode(data).map_err(|_| ::rpc_driver::RpcError::GarbageArguments)?;
                let res = self.inner.schedcoord_lookup_transactions(client, args.0).await.map_err(|_| ::rpc_driver::RpcError::SystemError)?;
                xdrs::txs::encode(&res).map_err(|_| ::rpc_driver::RpcError::SystemError)
            }
            _ => Err(::rpc_driver::RpcError::InvalidProcedure),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SchedulerCoord_SchedulerCoord1_Client {
    handle: ::std::sync::Arc<::rpc_driver::ClientHandle>,
}

impl SchedulerCoord_SchedulerCoord1_Client {
    pub fn new(handle: ::std::sync::Arc<::rpc_driver::ClientHandle>) -> Self {
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
    pub fn outcome(&self) -> i32 {
        match self {
            SchedulerNominationResult::Default{ outcome } => *outcome ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl SchedulerValidationResult {
    pub fn outcome(&self) -> i32 {
        match self {
            SchedulerValidationResult::Default{ outcome } => *outcome ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}

impl SchedulerGetTipResult {
    pub fn outcome(&self) -> i32 {
        match self {
            SchedulerGetTipResult::Default{ outcome } => *outcome ,

            _ => unsafe { *(self as *const Self as *const i32) },
        }
    }
}


