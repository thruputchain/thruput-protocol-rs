//! Signature pins for `thruput_coordinator` (generated: src/catchup.rs,
//! src/discovery.rs, src/dogwood.rs, src/loadgen.rs, src/repair.rs).

use thruput_coordinator::{
    BlockHeight, BristleconeKnownQuorumSets, Catchup_Catchup1,
    Catchup_Catchup1_Client, Discovery_Discovery2,
    Discovery_Discovery2_Client, Dogwood_Dogwood1, Dogwood_Dogwood1_Client,
    DogwoodDeliveryEnvelope, ExperimentConfig, LoadgenContextID,
    LoadgenExtensionResult, LoadgenInterface_LoadgenInterfaceV1,
    LoadgenInterface_LoadgenInterfaceV1_Client, LoadgenResult, Repair_Repair2,
    Repair_Repair2_Client, RepairedBlocks, RepairedProposal, RepairedSlot,
    RepairedTxs, SignedCatchupBlocksRequest, SignedCatchupBlocksResult,
    SignedMissingBlocks, SignedMissingProposal, SignedMissingSlot,
    SignedMissingTxs,
};
use thruput_rpc_api::{ProgramError, RpcError};
use xdrs::EncodesAs;

pub mod discovery {
    use super::*;
    type S = Discovery_Discovery2_Client;

    pub async fn client_discovery_null(c: &S) -> Result<(), RpcError> {
        c.discovery_null().await
    }
    pub async fn client_discovery_update_quorums(
        c: &S,
        arg0: &(impl EncodesAs<BristleconeKnownQuorumSets> + ?Sized),
    ) -> Result<BristleconeKnownQuorumSets, RpcError> {
        c.discovery_update_quorums(arg0).await
    }

    pub async fn trait_discovery_null<T: Discovery_Discovery2>(
        t: &T,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.discovery_null().await
    }
    pub async fn trait_discovery_update_quorums<T: Discovery_Discovery2>(
        t: &T,
        arg0: BristleconeKnownQuorumSets,
    ) -> Result<impl EncodesAs<BristleconeKnownQuorumSets>, ProgramError> {
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
        arg0: &(impl EncodesAs<SignedCatchupBlocksRequest> + ?Sized),
    ) -> Result<SignedCatchupBlocksResult, RpcError> {
        c.catchup_solicit(arg0).await
    }

    pub async fn trait_catchup_null<T: Catchup_Catchup1>(
        t: &T,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.catchup_null().await
    }
    pub async fn trait_catchup_solicit<T: Catchup_Catchup1>(
        t: &T,
        arg0: SignedCatchupBlocksRequest,
    ) -> Result<impl EncodesAs<SignedCatchupBlocksResult>, ProgramError> {
        t.catchup_solicit(arg0).await
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
        arg0: &(impl EncodesAs<SignedMissingTxs> + ?Sized),
    ) -> Result<RepairedTxs, RpcError> {
        c.repair_missing_txs(arg0).await
    }
    pub async fn client_repair_missing_proposal(
        c: &S,
        arg0: &(impl EncodesAs<SignedMissingProposal> + ?Sized),
    ) -> Result<RepairedProposal, RpcError> {
        c.repair_missing_proposal(arg0).await
    }
    pub async fn client_repair_missing_slot(
        c: &S,
        arg0: &(impl EncodesAs<SignedMissingSlot> + ?Sized),
    ) -> Result<RepairedSlot, RpcError> {
        c.repair_missing_slot(arg0).await
    }
    pub async fn client_repair_missing_blocks(
        c: &S,
        arg0: &(impl EncodesAs<SignedMissingBlocks> + ?Sized),
    ) -> Result<RepairedBlocks, RpcError> {
        c.repair_missing_blocks(arg0).await
    }

    pub async fn trait_repair_null<T: Repair_Repair2>(
        t: &T,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.repair_null().await
    }
    pub async fn trait_repair_missing_txs<T: Repair_Repair2>(
        t: &T,
        arg0: SignedMissingTxs,
    ) -> Result<impl EncodesAs<RepairedTxs>, ProgramError> {
        t.repair_missing_txs(arg0).await
    }
    pub async fn trait_repair_missing_proposal<T: Repair_Repair2>(
        t: &T,
        arg0: SignedMissingProposal,
    ) -> Result<impl EncodesAs<RepairedProposal>, ProgramError> {
        t.repair_missing_proposal(arg0).await
    }
    pub async fn trait_repair_missing_slot<T: Repair_Repair2>(
        t: &T,
        arg0: SignedMissingSlot,
    ) -> Result<impl EncodesAs<RepairedSlot>, ProgramError> {
        t.repair_missing_slot(arg0).await
    }
    pub async fn trait_repair_missing_blocks<T: Repair_Repair2>(
        t: &T,
        arg0: SignedMissingBlocks,
    ) -> Result<impl EncodesAs<RepairedBlocks>, ProgramError> {
        t.repair_missing_blocks(arg0).await
    }
}

pub mod loadgen_interface {
    use super::*;
    type S = LoadgenInterface_LoadgenInterfaceV1_Client;

    pub async fn client_generate_experiment(
        c: &S,
        arg0: &(impl EncodesAs<ExperimentConfig> + ?Sized),
    ) -> Result<LoadgenResult, RpcError> {
        c.generate_experiment(arg0).await
    }
    pub async fn client_extend_experiment(
        c: &S,
        arg0: &(impl EncodesAs<LoadgenContextID> + ?Sized),
        arg1: u64,
        arg2: &(impl EncodesAs<BlockHeight> + ?Sized),
    ) -> Result<LoadgenExtensionResult, RpcError> {
        c.extend_experiment(arg0, arg1, arg2).await
    }

    pub async fn trait_generate_experiment<
        T: LoadgenInterface_LoadgenInterfaceV1,
    >(
        t: &T,
        arg0: ExperimentConfig,
    ) -> Result<impl EncodesAs<LoadgenResult>, ProgramError> {
        t.generate_experiment(arg0).await
    }
    pub async fn trait_extend_experiment<
        T: LoadgenInterface_LoadgenInterfaceV1,
    >(
        t: &T,
        arg0: LoadgenContextID,
        arg1: u64,
        arg2: BlockHeight,
    ) -> Result<impl EncodesAs<LoadgenExtensionResult>, ProgramError> {
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
        arg0: &(impl EncodesAs<DogwoodDeliveryEnvelope> + ?Sized),
    ) -> Result<(), RpcError> {
        c.dogwood_deliver(arg0).await
    }

    pub async fn trait_dogwood_null<T: Dogwood_Dogwood1>(
        t: &T,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.dogwood_null().await
    }
    pub async fn trait_dogwood_deliver<T: Dogwood_Dogwood1>(
        t: &T,
        arg0: DogwoodDeliveryEnvelope,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.dogwood_deliver(arg0).await
    }
}
