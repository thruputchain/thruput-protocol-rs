//! Signature pins for `groundhog_akkad` (generated: src/coord_sched.rs, src/discovery.rs,
//! src/catchup.rs, src/db_sched.rs, src/repair2.rs, src/sched_coord.rs, src/loadgen.rs,
//! src/dogwood.rs).

use groundhog_akkad::{
    BlockExecutionMode, BlockHeight, BlockID, BristleconeKnownQuorumSets,
    Catchup_Catchup1, Catchup_Catchup1_Client, ConsensusValue,
    ContractAddressBatch, CoordCard, CoordScheduler_CoordScheduler1,
    CoordScheduler_CoordScheduler1_Client, CoordinatorRegistrationOutcome,
    DbAddressAuthDataBatchResult, DbBlockResult, DbSched_DbSched1,
    DbSched_DbSched1_Client, DbSchedResult, DbTipResult, Discovery_Discovery2,
    Discovery_Discovery2_Client, Dogwood_Dogwood1, Dogwood_Dogwood1_Client,
    DogwoodDeliveryEnvelope, ExperimentConfig, GenesisCeremonyRequest,
    GenesisCeremonyResult, GenesisConfiguration, HogAgeBatch, HogAgeResults,
    LoadgenContextID, LoadgenExtensionResult,
    LoadgenInterface_LoadgenInterfaceV1,
    LoadgenInterface_LoadgenInterfaceV1_Client, LoadgenResult,
    OptSignedTransactionBatch, ParallelSignedTransactionBatch, Proposal,
    ProposalContext, Repair_Repair2, Repair_Repair2_Client, RepairedBlocks,
    RepairedProposal, RepairedSlot, RepairedTxs, SchedulerCommitOutcome,
    SchedulerCoord_SchedulerCoord1, SchedulerCoord_SchedulerCoord1_Client,
    SchedulerGetTipResult, SchedulerProposalResult, SchedulerValidationResult,
    SignedCatchupBlocksRequest, SignedCatchupBlocksResult,
    SignedMissingBlocks, SignedMissingProposal, SignedMissingSlot,
    SignedMissingTxs, TransactionHashBatch, ValidatorID,
};
use rpc_api::{ProgramError, RpcError};

pub mod coord_scheduler {
    use super::*;
    type S = CoordScheduler_CoordScheduler1_Client;

    pub async fn client_coordscheduler_null(c: &S) -> Result<(), RpcError> {
        c.coordscheduler_null().await
    }
    pub async fn client_coordscheduler_notify_vetted(
        c: &S,
        arg0: &ParallelSignedTransactionBatch,
    ) -> Result<(), RpcError> {
        c.coordscheduler_notify_vetted(arg0).await
    }

    pub async fn trait_coordscheduler_null<
        T: CoordScheduler_CoordScheduler1,
    >(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.coordscheduler_null().await
    }
    pub async fn trait_coordscheduler_notify_vetted<
        T: CoordScheduler_CoordScheduler1,
    >(
        t: &T,
        arg0: ParallelSignedTransactionBatch,
    ) -> Result<(), ProgramError> {
        t.coordscheduler_notify_vetted(arg0).await
    }
}

pub mod discovery {
    use super::*;
    type S = Discovery_Discovery2_Client;

    pub async fn client_discovery_null(c: &S) -> Result<(), RpcError> {
        c.discovery_null().await
    }
    pub async fn client_discovery_update_quorums(
        c: &S,
        arg0: &BristleconeKnownQuorumSets,
    ) -> Result<BristleconeKnownQuorumSets, RpcError> {
        c.discovery_update_quorums(arg0).await
    }

    pub async fn trait_discovery_null<T: Discovery_Discovery2>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.discovery_null().await
    }
    pub async fn trait_discovery_update_quorums<T: Discovery_Discovery2>(
        t: &T,
        arg0: BristleconeKnownQuorumSets,
    ) -> Result<BristleconeKnownQuorumSets, ProgramError> {
        t.discovery_update_quorums(arg0).await
    }
}

pub mod catchup {
    use super::*;
    type S = Catchup_Catchup1_Client;

    pub async fn client_catchup_null(c: &S) -> Result<(), RpcError> {
        c.catchup_null().await
    }
    pub async fn client_catchup_solicit(
        c: &S,
        arg0: &SignedCatchupBlocksRequest,
    ) -> Result<SignedCatchupBlocksResult, RpcError> {
        c.catchup_solicit(arg0).await
    }

    pub async fn trait_catchup_null<T: Catchup_Catchup1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.catchup_null().await
    }
    pub async fn trait_catchup_solicit<T: Catchup_Catchup1>(
        t: &T,
        arg0: SignedCatchupBlocksRequest,
    ) -> Result<SignedCatchupBlocksResult, ProgramError> {
        t.catchup_solicit(arg0).await
    }
}

pub mod db_sched {
    use super::*;
    type S = DbSched_DbSched1_Client;

    pub async fn client_dbsched_null(c: &S) -> Result<(), RpcError> {
        c.dbsched_null().await
    }
    pub async fn client_dbsched_get_address_auth_data(
        c: &S,
        arg0: &BlockID,
        arg1: &ContractAddressBatch,
    ) -> Result<DbAddressAuthDataBatchResult, RpcError> {
        c.dbsched_get_address_auth_data(arg0, arg1).await
    }
    pub async fn client_dbsched_get_hog_ages(
        c: &S,
        arg0: &ContractAddressBatch,
    ) -> Result<HogAgeResults, RpcError> {
        c.dbsched_get_hog_ages(arg0).await
    }
    pub async fn client_dbsched_write_hog_ages(
        c: &S,
        arg0: &HogAgeBatch,
    ) -> Result<DbSchedResult, RpcError> {
        c.dbsched_write_hog_ages(arg0).await
    }
    pub async fn client_dbsched_create_snapshot(
        c: &S,
        arg0: &BlockID,
        arg1: &BlockID,
        arg2: &BlockExecutionMode,
    ) -> Result<DbSchedResult, RpcError> {
        c.dbsched_create_snapshot(arg0, arg1, arg2).await
    }
    pub async fn client_dbsched_commit_snapshot(
        c: &S,
        arg0: &BlockID,
        arg1: &ConsensusValue,
    ) -> Result<DbBlockResult, RpcError> {
        c.dbsched_commit_snapshot(arg0, arg1).await
    }
    pub async fn client_dbsched_close_snapshot(
        c: &S,
        arg0: &BlockID,
    ) -> Result<DbBlockResult, RpcError> {
        c.dbsched_close_snapshot(arg0).await
    }
    pub async fn client_dbsched_abandon_snapshot(
        c: &S,
        arg0: &BlockID,
    ) -> Result<DbSchedResult, RpcError> {
        c.dbsched_abandon_snapshot(arg0).await
    }
    pub async fn client_dbsched_load_genesis_configuration(
        c: &S,
        arg0: &GenesisConfiguration,
        arg1: &ConsensusValue,
    ) -> Result<DbSchedResult, RpcError> {
        c.dbsched_load_genesis_configuration(arg0, arg1).await
    }
    pub async fn client_dbsched_get_tip(
        c: &S,
    ) -> Result<DbTipResult, RpcError> {
        c.dbsched_get_tip().await
    }

    pub async fn trait_dbsched_null<T: DbSched_DbSched1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.dbsched_null().await
    }
    pub async fn trait_dbsched_get_address_auth_data<T: DbSched_DbSched1>(
        t: &T,
        arg0: BlockID,
        arg1: ContractAddressBatch,
    ) -> Result<DbAddressAuthDataBatchResult, ProgramError> {
        t.dbsched_get_address_auth_data(arg0, arg1).await
    }
    pub async fn trait_dbsched_get_hog_ages<T: DbSched_DbSched1>(
        t: &T,
        arg0: ContractAddressBatch,
    ) -> Result<HogAgeResults, ProgramError> {
        t.dbsched_get_hog_ages(arg0).await
    }
    pub async fn trait_dbsched_write_hog_ages<T: DbSched_DbSched1>(
        t: &T,
        arg0: HogAgeBatch,
    ) -> Result<DbSchedResult, ProgramError> {
        t.dbsched_write_hog_ages(arg0).await
    }
    pub async fn trait_dbsched_create_snapshot<T: DbSched_DbSched1>(
        t: &T,
        arg0: BlockID,
        arg1: BlockID,
        arg2: BlockExecutionMode,
    ) -> Result<DbSchedResult, ProgramError> {
        t.dbsched_create_snapshot(arg0, arg1, arg2).await
    }
    pub async fn trait_dbsched_commit_snapshot<T: DbSched_DbSched1>(
        t: &T,
        arg0: BlockID,
        arg1: ConsensusValue,
    ) -> Result<DbBlockResult, ProgramError> {
        t.dbsched_commit_snapshot(arg0, arg1).await
    }
    pub async fn trait_dbsched_close_snapshot<T: DbSched_DbSched1>(
        t: &T,
        arg0: BlockID,
    ) -> Result<DbBlockResult, ProgramError> {
        t.dbsched_close_snapshot(arg0).await
    }
    pub async fn trait_dbsched_abandon_snapshot<T: DbSched_DbSched1>(
        t: &T,
        arg0: BlockID,
    ) -> Result<DbSchedResult, ProgramError> {
        t.dbsched_abandon_snapshot(arg0).await
    }
    pub async fn trait_dbsched_load_genesis_configuration<
        T: DbSched_DbSched1,
    >(
        t: &T,
        arg0: GenesisConfiguration,
        arg1: ConsensusValue,
    ) -> Result<DbSchedResult, ProgramError> {
        t.dbsched_load_genesis_configuration(arg0, arg1).await
    }
    pub async fn trait_dbsched_get_tip<T: DbSched_DbSched1>(
        t: &T,
    ) -> Result<DbTipResult, ProgramError> {
        t.dbsched_get_tip().await
    }
}

pub mod repair {
    use super::*;
    type S = Repair_Repair2_Client;

    pub async fn client_repair_null(c: &S) -> Result<(), RpcError> {
        c.repair_null().await
    }
    pub async fn client_repair_missing_txs(
        c: &S,
        arg0: &SignedMissingTxs,
    ) -> Result<RepairedTxs, RpcError> {
        c.repair_missing_txs(arg0).await
    }
    pub async fn client_repair_missing_proposal(
        c: &S,
        arg0: &SignedMissingProposal,
    ) -> Result<RepairedProposal, RpcError> {
        c.repair_missing_proposal(arg0).await
    }
    pub async fn client_repair_missing_slot(
        c: &S,
        arg0: &SignedMissingSlot,
    ) -> Result<RepairedSlot, RpcError> {
        c.repair_missing_slot(arg0).await
    }
    pub async fn client_repair_missing_blocks(
        c: &S,
        arg0: &SignedMissingBlocks,
    ) -> Result<RepairedBlocks, RpcError> {
        c.repair_missing_blocks(arg0).await
    }

    pub async fn trait_repair_null<T: Repair_Repair2>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.repair_null().await
    }
    pub async fn trait_repair_missing_txs<T: Repair_Repair2>(
        t: &T,
        arg0: SignedMissingTxs,
    ) -> Result<RepairedTxs, ProgramError> {
        t.repair_missing_txs(arg0).await
    }
    pub async fn trait_repair_missing_proposal<T: Repair_Repair2>(
        t: &T,
        arg0: SignedMissingProposal,
    ) -> Result<RepairedProposal, ProgramError> {
        t.repair_missing_proposal(arg0).await
    }
    pub async fn trait_repair_missing_slot<T: Repair_Repair2>(
        t: &T,
        arg0: SignedMissingSlot,
    ) -> Result<RepairedSlot, ProgramError> {
        t.repair_missing_slot(arg0).await
    }
    pub async fn trait_repair_missing_blocks<T: Repair_Repair2>(
        t: &T,
        arg0: SignedMissingBlocks,
    ) -> Result<RepairedBlocks, ProgramError> {
        t.repair_missing_blocks(arg0).await
    }
}

pub mod scheduler_coord {
    use super::*;
    type S = SchedulerCoord_SchedulerCoord1_Client;

    pub async fn client_schedulercoord_null(c: &S) -> Result<(), RpcError> {
        c.schedulercoord_null().await
    }
    pub async fn client_schedulercoord_intro_coord(
        c: &S,
        arg0: &CoordCard,
    ) -> Result<CoordinatorRegistrationOutcome, RpcError> {
        c.schedulercoord_intro_coord(arg0).await
    }
    pub async fn client_schedulercoord_submit_unvetted(
        c: &S,
        arg0: &ParallelSignedTransactionBatch,
    ) -> Result<(), RpcError> {
        c.schedulercoord_submit_unvetted(arg0).await
    }
    pub async fn client_schedulercoord_submit_vetted(
        c: &S,
        arg0: &ValidatorID,
        arg1: &ParallelSignedTransactionBatch,
    ) -> Result<(), RpcError> {
        c.schedulercoord_submit_vetted(arg0, arg1).await
    }
    pub async fn client_schedulercoord_propose_block(
        c: &S,
        arg0: &BlockID,
        arg1: &ProposalContext,
    ) -> Result<SchedulerProposalResult, RpcError> {
        c.schedulercoord_propose_block(arg0, arg1).await
    }
    pub async fn client_schedulercoord_validate_block(
        c: &S,
        arg0: &BlockID,
        arg1: &Proposal,
    ) -> Result<SchedulerValidationResult, RpcError> {
        c.schedulercoord_validate_block(arg0, arg1).await
    }
    pub async fn client_schedulercoord_commit_block(
        c: &S,
        arg0: &BlockID,
        arg1: &ConsensusValue,
    ) -> Result<SchedulerCommitOutcome, RpcError> {
        c.schedulercoord_commit_block(arg0, arg1).await
    }
    pub async fn client_schedcoord_get_tip(
        c: &S,
    ) -> Result<SchedulerGetTipResult, RpcError> {
        c.schedcoord_get_tip().await
    }
    pub async fn client_schedcoord_load_genesis_configuration(
        c: &S,
        arg0: &GenesisConfiguration,
    ) -> Result<DbSchedResult, RpcError> {
        c.schedcoord_load_genesis_configuration(arg0).await
    }
    pub async fn client_schedulercoord_commit_genesis_ceremony(
        c: &S,
        arg0: &GenesisCeremonyRequest,
    ) -> Result<GenesisCeremonyResult, RpcError> {
        c.schedulercoord_commit_genesis_ceremony(arg0).await
    }
    pub async fn client_schedcoord_lookup_transactions(
        c: &S,
        arg0: &TransactionHashBatch,
    ) -> Result<OptSignedTransactionBatch, RpcError> {
        c.schedcoord_lookup_transactions(arg0).await
    }

    pub async fn trait_schedulercoord_null<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.schedulercoord_null().await
    }
    pub async fn trait_schedulercoord_intro_coord<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: CoordCard,
    ) -> Result<CoordinatorRegistrationOutcome, ProgramError> {
        t.schedulercoord_intro_coord(arg0).await
    }
    pub async fn trait_schedulercoord_submit_unvetted<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: ParallelSignedTransactionBatch,
    ) -> Result<(), ProgramError> {
        t.schedulercoord_submit_unvetted(arg0).await
    }
    pub async fn trait_schedulercoord_submit_vetted<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: ValidatorID,
        arg1: ParallelSignedTransactionBatch,
    ) -> Result<(), ProgramError> {
        t.schedulercoord_submit_vetted(arg0, arg1).await
    }
    pub async fn trait_schedulercoord_propose_block<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: ProposalContext,
    ) -> Result<SchedulerProposalResult, ProgramError> {
        t.schedulercoord_propose_block(arg0, arg1).await
    }
    pub async fn trait_schedulercoord_validate_block<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: Proposal,
    ) -> Result<SchedulerValidationResult, ProgramError> {
        t.schedulercoord_validate_block(arg0, arg1).await
    }
    pub async fn trait_schedulercoord_commit_block<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: ConsensusValue,
    ) -> Result<SchedulerCommitOutcome, ProgramError> {
        t.schedulercoord_commit_block(arg0, arg1).await
    }
    pub async fn trait_schedcoord_get_tip<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
    ) -> Result<SchedulerGetTipResult, ProgramError> {
        t.schedcoord_get_tip().await
    }
    pub async fn trait_schedcoord_load_genesis_configuration<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: GenesisConfiguration,
    ) -> Result<DbSchedResult, ProgramError> {
        t.schedcoord_load_genesis_configuration(arg0).await
    }
    pub async fn trait_schedulercoord_commit_genesis_ceremony<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: GenesisCeremonyRequest,
    ) -> Result<GenesisCeremonyResult, ProgramError> {
        t.schedulercoord_commit_genesis_ceremony(arg0).await
    }
    pub async fn trait_schedcoord_lookup_transactions<
        T: SchedulerCoord_SchedulerCoord1,
    >(
        t: &T,
        arg0: TransactionHashBatch,
    ) -> Result<OptSignedTransactionBatch, ProgramError> {
        t.schedcoord_lookup_transactions(arg0).await
    }
}

pub mod loadgen_interface {
    use super::*;
    type S = LoadgenInterface_LoadgenInterfaceV1_Client;

    pub async fn client_generate_experiment(
        c: &S,
        arg0: &ExperimentConfig,
    ) -> Result<LoadgenResult, RpcError> {
        c.generate_experiment(arg0).await
    }
    pub async fn client_extend_experiment(
        c: &S,
        arg0: &LoadgenContextID,
        arg1: u64,
        arg2: &BlockHeight,
    ) -> Result<LoadgenExtensionResult, RpcError> {
        c.extend_experiment(arg0, arg1, arg2).await
    }

    pub async fn trait_generate_experiment<
        T: LoadgenInterface_LoadgenInterfaceV1,
    >(
        t: &T,
        arg0: ExperimentConfig,
    ) -> Result<LoadgenResult, ProgramError> {
        t.generate_experiment(arg0).await
    }
    pub async fn trait_extend_experiment<
        T: LoadgenInterface_LoadgenInterfaceV1,
    >(
        t: &T,
        arg0: LoadgenContextID,
        arg1: u64,
        arg2: BlockHeight,
    ) -> Result<LoadgenExtensionResult, ProgramError> {
        t.extend_experiment(arg0, arg1, arg2).await
    }
}

pub mod dogwood {
    use super::*;
    type S = Dogwood_Dogwood1_Client;

    pub async fn client_dogwood_null(c: &S) -> Result<(), RpcError> {
        c.dogwood_null().await
    }
    pub async fn client_dogwood_deliver(
        c: &S,
        arg0: &DogwoodDeliveryEnvelope,
    ) -> Result<(), RpcError> {
        c.dogwood_deliver(arg0).await
    }

    pub async fn trait_dogwood_null<T: Dogwood_Dogwood1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.dogwood_null().await
    }
    pub async fn trait_dogwood_deliver<T: Dogwood_Dogwood1>(
        t: &T,
        arg0: DogwoodDeliveryEnvelope,
    ) -> Result<(), ProgramError> {
        t.dogwood_deliver(arg0).await
    }
}
