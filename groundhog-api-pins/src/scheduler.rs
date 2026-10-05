//! Signature pins for `groundhog_scheduler` (generated: src/catchup.rs,
//! src/coord_sched.rs, src/db_sched.rs, src/event_rpc.rs, src/exec_sched.rs,
//! src/loadgen.rs, src/repair.rs, src/sched_coord.rs, src/sched_db.rs,
//! src/sched_event.rs, src/sched_gateway.rs).

use groundhog_scheduler::{
    BlockExecutionContext, BlockExecutionMode, BlockID, Catchup_Catchup1,
    Catchup_Catchup1_Client, ConsensusValue, ContractAddressBatch, CoordCard,
    CoordScheduler_CoordScheduler1, CoordScheduler_CoordScheduler1_Client,
    CoordinatorRegistrationOutcome, DbAddressAuthDataBatchResult,
    DbBlockResult, DbSched_DbSched1, DbSched_DbSched1_Client, DbSchedResult,
    DbTipResult, EventActivationResult, EventBlockResult, EventCard,
    EventExecutor_EventExecutor1, EventExecutor_EventExecutor1_Client,
    EventPrepareResult, EventRegistrationSnapshot, EventSched_EventSched1,
    EventSched_EventSched1_Client, ExecProposeBatchResult, ExecProposeResult,
    ExecValidateBatchResult, ExecutorSched_ExecutorSched1,
    ExecutorSched_ExecutorSched1_Client, ExpectedDBTip, ExperimentConfig,
    GenesisCeremonyRequest, GenesisCeremonyResult, HogAgeBatch, HogAgeResults,
    HogsTransferred, LoadgenContextID, LoadgenExtensionResult,
    LoadgenInterface_LoadgenInterfaceV1,
    LoadgenInterface_LoadgenInterfaceV1_Client, LoadgenResult,
    MissingTransactionsEnvelope, OptProposal, ParallelSignedTransactionBatch,
    PendingTxCount, Proposal, RPCError, Repair_Repair1, Repair_Repair1_Client,
    RepairedTransactions, SchedDb_SchedDb1, SchedDb_SchedDb1_Client,
    SchedEvent_SchedEvent1, SchedEvent_SchedEvent1_Client,
    SchedGateway_SchedGateway1, SchedGateway_SchedGateway1_Client,
    SchedGatewayIntroOutcome, SchedGatewaySubmitOutcomeBatch,
    SchedulerCommitOutcome, SchedulerCoord_SchedulerCoord1,
    SchedulerCoord_SchedulerCoord1_Client, SchedulerGetTipResult,
    SchedulerProposalResult, SchedulerValidationResult, ShardCard,
    SignedCatchupBlocksRequest, SignedCatchupBlocksResult, TransactionID,
};
use groundhog_scp::scp::{SCPEnvelopes, SCPSlotIndex};
use groundhog_trace::db_conflict::TxFailureDiagnostic;
use groundhog_xdr::event::TxEventLog;
use groundhog_xdr::exec_result::ExecResult;
use groundhog_xdr::genesis::GenesisConfiguration;
use groundhog_xdr::header::ProposalContext;
use groundhog_xdr::network::ProtocolCommit;
use groundhog_xdr::transaction::{
    OptSignedTransactionBatch, ProposedTransaction, ProposedTransactionBatch,
    SignedTransaction, SignedTransactionBatch, TransactionHash,
    TransactionHashBatch,
};
use groundhog_xdr::types::BlockHeight;
use groundhog_xdr::validator::ValidatorID;
use rpc_api::{ProgramError, RpcError};

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

pub mod event_executor {
    use super::*;
    type S = EventExecutor_EventExecutor1_Client;

    pub async fn client_eventexecutor_null(c: &S) -> Result<(), RpcError> {
        c.eventexecutor_null().await
    }
    pub async fn client_eventexecutor_open_block(
        c: &S,
        arg0: &BlockID,
    ) -> Result<RPCError, RpcError> {
        c.eventexecutor_open_block(arg0).await
    }
    pub async fn client_eventexecutor_close_block(
        c: &S,
        arg0: &BlockID,
    ) -> Result<RPCError, RpcError> {
        c.eventexecutor_close_block(arg0).await
    }
    pub async fn client_eventexecutor_report_tx_failure(
        c: &S,
        arg0: &TransactionHash,
        arg1: &TxFailureDiagnostic,
    ) -> Result<RPCError, RpcError> {
        c.eventexecutor_report_tx_failure(arg0, arg1).await
    }
    pub async fn client_eventexecutor_emit_event(
        c: &S,
        arg0: &BlockID,
        arg1: &TransactionID,
        arg2: &TxEventLog,
        arg3: &ProposedTransaction,
    ) -> Result<RPCError, RpcError> {
        c.eventexecutor_emit_event(arg0, arg1, arg2, arg3).await
    }

    pub async fn trait_eventexecutor_null<T: EventExecutor_EventExecutor1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.eventexecutor_null().await
    }
    pub async fn trait_eventexecutor_open_block<
        T: EventExecutor_EventExecutor1,
    >(
        t: &T,
        arg0: BlockID,
    ) -> Result<RPCError, ProgramError> {
        t.eventexecutor_open_block(arg0).await
    }
    pub async fn trait_eventexecutor_close_block<
        T: EventExecutor_EventExecutor1,
    >(
        t: &T,
        arg0: BlockID,
    ) -> Result<RPCError, ProgramError> {
        t.eventexecutor_close_block(arg0).await
    }
    pub async fn trait_eventexecutor_report_tx_failure<
        T: EventExecutor_EventExecutor1,
    >(
        t: &T,
        arg0: TransactionHash,
        arg1: TxFailureDiagnostic,
    ) -> Result<RPCError, ProgramError> {
        t.eventexecutor_report_tx_failure(arg0, arg1).await
    }
    pub async fn trait_eventexecutor_emit_event<
        T: EventExecutor_EventExecutor1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: TransactionID,
        arg2: TxEventLog,
        arg3: ProposedTransaction,
    ) -> Result<RPCError, ProgramError> {
        t.eventexecutor_emit_event(arg0, arg1, arg2, arg3).await
    }
}

pub mod event_sched {
    use super::*;
    type S = EventSched_EventSched1_Client;

    pub async fn client_eventsched_null(c: &S) -> Result<(), RpcError> {
        c.eventsched_null().await
    }
    pub async fn client_eventsched_open_block(
        c: &S,
        arg0: &BlockID,
    ) -> Result<RPCError, RpcError> {
        c.eventsched_open_block(arg0).await
    }
    pub async fn client_eventsched_close_block(
        c: &S,
        arg0: &BlockID,
        arg1: &BlockHeight,
    ) -> Result<EventBlockResult, RpcError> {
        c.eventsched_close_block(arg0, arg1).await
    }
    pub async fn client_eventsched_commit_block(
        c: &S,
        arg0: &BlockID,
        arg1: &ConsensusValue,
    ) -> Result<RPCError, RpcError> {
        c.eventsched_commit_block(arg0, arg1).await
    }
    pub async fn client_eventsched_abandon_block(
        c: &S,
        arg0: &BlockID,
    ) -> Result<RPCError, RpcError> {
        c.eventsched_abandon_block(arg0).await
    }

    pub async fn trait_eventsched_null<T: EventSched_EventSched1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.eventsched_null().await
    }
    pub async fn trait_eventsched_open_block<T: EventSched_EventSched1>(
        t: &T,
        arg0: BlockID,
    ) -> Result<RPCError, ProgramError> {
        t.eventsched_open_block(arg0).await
    }
    pub async fn trait_eventsched_close_block<T: EventSched_EventSched1>(
        t: &T,
        arg0: BlockID,
        arg1: BlockHeight,
    ) -> Result<EventBlockResult, ProgramError> {
        t.eventsched_close_block(arg0, arg1).await
    }
    pub async fn trait_eventsched_commit_block<T: EventSched_EventSched1>(
        t: &T,
        arg0: BlockID,
        arg1: ConsensusValue,
    ) -> Result<RPCError, ProgramError> {
        t.eventsched_commit_block(arg0, arg1).await
    }
    pub async fn trait_eventsched_abandon_block<T: EventSched_EventSched1>(
        t: &T,
        arg0: BlockID,
    ) -> Result<RPCError, ProgramError> {
        t.eventsched_abandon_block(arg0).await
    }
}

pub mod executor_sched {
    use super::*;
    type S = ExecutorSched_ExecutorSched1_Client;

    pub async fn client_executorsched_null(c: &S) -> Result<(), RpcError> {
        c.executorsched_null().await
    }
    pub async fn client_executorsched_ready(
        c: &S,
    ) -> Result<ExecResult, RpcError> {
        c.executorsched_ready().await
    }
    pub async fn client_executorsched_propose_transaction(
        c: &S,
        arg0: &BlockID,
        arg1: &SignedTransaction,
    ) -> Result<ExecProposeResult, RpcError> {
        c.executorsched_propose_transaction(arg0, arg1).await
    }
    pub async fn client_executorsched_validate_transaction(
        c: &S,
        arg0: &BlockID,
        arg1: &ProposedTransaction,
    ) -> Result<ExecResult, RpcError> {
        c.executorsched_validate_transaction(arg0, arg1).await
    }
    pub async fn client_executorsched_propose_batch(
        c: &S,
        arg0: &BlockID,
        arg1: &SignedTransactionBatch,
    ) -> Result<ExecProposeBatchResult, RpcError> {
        c.executorsched_propose_batch(arg0, arg1).await
    }
    pub async fn client_executorsched_validate_batch(
        c: &S,
        arg0: &BlockID,
        arg1: &ProposedTransactionBatch,
    ) -> Result<ExecValidateBatchResult, RpcError> {
        c.executorsched_validate_batch(arg0, arg1).await
    }
    pub async fn client_executorsched_open_proposal_block(
        c: &S,
        arg0: &BlockID,
        arg1: &BlockExecutionContext,
    ) -> Result<ExecResult, RpcError> {
        c.executorsched_open_proposal_block(arg0, arg1).await
    }
    pub async fn client_executorsched_open_validation_block(
        c: &S,
        arg0: &BlockID,
        arg1: &BlockExecutionContext,
    ) -> Result<ExecResult, RpcError> {
        c.executorsched_open_validation_block(arg0, arg1).await
    }
    pub async fn client_executorsched_finish_block(
        c: &S,
        arg0: &BlockID,
    ) -> Result<ExecResult, RpcError> {
        c.executorsched_finish_block(arg0).await
    }
    pub async fn client_executorsched_shutdown(c: &S) -> Result<(), RpcError> {
        c.executorsched_shutdown().await
    }

    pub async fn trait_executorsched_null<T: ExecutorSched_ExecutorSched1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.executorsched_null().await
    }
    pub async fn trait_executorsched_ready<T: ExecutorSched_ExecutorSched1>(
        t: &T,
    ) -> Result<ExecResult, ProgramError> {
        t.executorsched_ready().await
    }
    pub async fn trait_executorsched_propose_transaction<
        T: ExecutorSched_ExecutorSched1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: SignedTransaction,
    ) -> Result<ExecProposeResult, ProgramError> {
        t.executorsched_propose_transaction(arg0, arg1).await
    }
    pub async fn trait_executorsched_validate_transaction<
        T: ExecutorSched_ExecutorSched1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: ProposedTransaction,
    ) -> Result<ExecResult, ProgramError> {
        t.executorsched_validate_transaction(arg0, arg1).await
    }
    pub async fn trait_executorsched_propose_batch<
        T: ExecutorSched_ExecutorSched1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: SignedTransactionBatch,
    ) -> Result<ExecProposeBatchResult, ProgramError> {
        t.executorsched_propose_batch(arg0, arg1).await
    }
    pub async fn trait_executorsched_validate_batch<
        T: ExecutorSched_ExecutorSched1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: ProposedTransactionBatch,
    ) -> Result<ExecValidateBatchResult, ProgramError> {
        t.executorsched_validate_batch(arg0, arg1).await
    }
    pub async fn trait_executorsched_open_proposal_block<
        T: ExecutorSched_ExecutorSched1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: BlockExecutionContext,
    ) -> Result<ExecResult, ProgramError> {
        t.executorsched_open_proposal_block(arg0, arg1).await
    }
    pub async fn trait_executorsched_open_validation_block<
        T: ExecutorSched_ExecutorSched1,
    >(
        t: &T,
        arg0: BlockID,
        arg1: BlockExecutionContext,
    ) -> Result<ExecResult, ProgramError> {
        t.executorsched_open_validation_block(arg0, arg1).await
    }
    pub async fn trait_executorsched_finish_block<
        T: ExecutorSched_ExecutorSched1,
    >(
        t: &T,
        arg0: BlockID,
    ) -> Result<ExecResult, ProgramError> {
        t.executorsched_finish_block(arg0).await
    }
    pub async fn trait_executorsched_shutdown<
        T: ExecutorSched_ExecutorSched1,
    >(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.executorsched_shutdown().await
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

pub mod repair {
    use super::*;
    type S = Repair_Repair1_Client;

    pub async fn client_repair_null(c: &S) -> Result<(), RpcError> {
        c.repair_null().await
    }
    pub async fn client_repair_missing_txs(
        c: &S,
        arg0: &MissingTransactionsEnvelope,
    ) -> Result<RepairedTransactions, RpcError> {
        c.repair_missing_txs(arg0).await
    }
    pub async fn client_repair_missing_proposal(
        c: &S,
        arg0: &ConsensusValue,
    ) -> Result<OptProposal, RpcError> {
        c.repair_missing_proposal(arg0).await
    }
    pub async fn client_repair_missing_scp_envelopes(
        c: &S,
        arg0: &SCPSlotIndex,
    ) -> Result<SCPEnvelopes, RpcError> {
        c.repair_missing_scp_envelopes(arg0).await
    }

    pub async fn trait_repair_null<T: Repair_Repair1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.repair_null().await
    }
    pub async fn trait_repair_missing_txs<T: Repair_Repair1>(
        t: &T,
        arg0: MissingTransactionsEnvelope,
    ) -> Result<RepairedTransactions, ProgramError> {
        t.repair_missing_txs(arg0).await
    }
    pub async fn trait_repair_missing_proposal<T: Repair_Repair1>(
        t: &T,
        arg0: ConsensusValue,
    ) -> Result<OptProposal, ProgramError> {
        t.repair_missing_proposal(arg0).await
    }
    pub async fn trait_repair_missing_scp_envelopes<T: Repair_Repair1>(
        t: &T,
        arg0: SCPSlotIndex,
    ) -> Result<SCPEnvelopes, ProgramError> {
        t.repair_missing_scp_envelopes(arg0).await
    }
}

pub mod sched_db {
    use super::*;
    type S = SchedDb_SchedDb1_Client;

    pub async fn client_scheddb_intro_shard(
        c: &S,
        arg0: &ShardCard,
    ) -> Result<(), RpcError> {
        c.scheddb_intro_shard(arg0).await
    }
    pub async fn client_scheddb_hogs_transferred(
        c: &S,
        arg0: &HogsTransferred,
    ) -> Result<(), RpcError> {
        c.scheddb_hogs_transferred(arg0).await
    }

    pub async fn trait_scheddb_intro_shard<T: SchedDb_SchedDb1>(
        t: &T,
        arg0: ShardCard,
    ) -> Result<(), ProgramError> {
        t.scheddb_intro_shard(arg0).await
    }
    pub async fn trait_scheddb_hogs_transferred<T: SchedDb_SchedDb1>(
        t: &T,
        arg0: HogsTransferred,
    ) -> Result<(), ProgramError> {
        t.scheddb_hogs_transferred(arg0).await
    }
}

pub mod sched_event {
    use super::*;
    type S = SchedEvent_SchedEvent1_Client;

    pub async fn client_schedevent_prepare_event(
        c: &S,
        arg0: &EventCard,
    ) -> Result<EventPrepareResult, RpcError> {
        c.schedevent_prepare_event(arg0).await
    }
    pub async fn client_schedevent_activate_event(
        c: &S,
        arg0: &EventRegistrationSnapshot,
    ) -> Result<EventActivationResult, RpcError> {
        c.schedevent_activate_event(arg0).await
    }

    pub async fn trait_schedevent_prepare_event<T: SchedEvent_SchedEvent1>(
        t: &T,
        arg0: EventCard,
    ) -> Result<EventPrepareResult, ProgramError> {
        t.schedevent_prepare_event(arg0).await
    }
    pub async fn trait_schedevent_activate_event<T: SchedEvent_SchedEvent1>(
        t: &T,
        arg0: EventRegistrationSnapshot,
    ) -> Result<EventActivationResult, ProgramError> {
        t.schedevent_activate_event(arg0).await
    }
}

pub mod sched_gateway {
    use super::*;
    type S = SchedGateway_SchedGateway1_Client;

    pub async fn client_schedgateway_intro(
        c: &S,
        arg0: &ProtocolCommit,
    ) -> Result<SchedGatewayIntroOutcome, RpcError> {
        c.schedgateway_intro(arg0).await
    }
    pub async fn client_schedgateway_submit_tx_batch(
        c: &S,
        arg0: &ParallelSignedTransactionBatch,
    ) -> Result<SchedGatewaySubmitOutcomeBatch, RpcError> {
        c.schedgateway_submit_tx_batch(arg0).await
    }
    pub async fn client_schedgateway_get_tip(
        c: &S,
    ) -> Result<ExpectedDBTip, RpcError> {
        c.schedgateway_get_tip().await
    }
    pub async fn client_schedgateway_lookup_tx(
        c: &S,
        arg0: &TransactionHashBatch,
    ) -> Result<OptSignedTransactionBatch, RpcError> {
        c.schedgateway_lookup_tx(arg0).await
    }
    pub async fn client_schedgateway_pending_tx_count(
        c: &S,
    ) -> Result<PendingTxCount, RpcError> {
        c.schedgateway_pending_tx_count().await
    }

    pub async fn trait_schedgateway_intro<T: SchedGateway_SchedGateway1>(
        t: &T,
        arg0: ProtocolCommit,
    ) -> Result<SchedGatewayIntroOutcome, ProgramError> {
        t.schedgateway_intro(arg0).await
    }
    pub async fn trait_schedgateway_submit_tx_batch<
        T: SchedGateway_SchedGateway1,
    >(
        t: &T,
        arg0: ParallelSignedTransactionBatch,
    ) -> Result<SchedGatewaySubmitOutcomeBatch, ProgramError> {
        t.schedgateway_submit_tx_batch(arg0).await
    }
    pub async fn trait_schedgateway_get_tip<T: SchedGateway_SchedGateway1>(
        t: &T,
    ) -> Result<ExpectedDBTip, ProgramError> {
        t.schedgateway_get_tip().await
    }
    pub async fn trait_schedgateway_lookup_tx<
        T: SchedGateway_SchedGateway1,
    >(
        t: &T,
        arg0: TransactionHashBatch,
    ) -> Result<OptSignedTransactionBatch, ProgramError> {
        t.schedgateway_lookup_tx(arg0).await
    }
    pub async fn trait_schedgateway_pending_tx_count<
        T: SchedGateway_SchedGateway1,
    >(
        t: &T,
    ) -> Result<PendingTxCount, ProgramError> {
        t.schedgateway_pending_tx_count().await
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
