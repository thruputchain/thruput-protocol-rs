//! Signature pins for `thruput_scp` (generated: src/scp_control.rs, src/scp_handle.rs).

use rpc_api::{ProgramError, RpcError};
use thruput_scp::{
    SCP_Control_SCP_Control1, SCP_Control_SCP_Control1_Client,
    SCP_Handle_SCP_Handle1, SCP_Handle_SCP_Handle1_Client, SCPBallot,
    SCPEnvelope, SCPHash, SCPOptQuorumSet, SCPOptValue, SCPQuorumSet,
    SCPSlotIndex, SCPValidationLevel, SCPValue, SCPValues,
};
use xdrs::EncodesAs;

pub mod scp_control {
    use super::*;
    type S = SCP_Control_SCP_Control1_Client;

    pub async fn client_scp_control_null(c: &S) -> Result<(), RpcError> {
        c.scp_control_null().await
    }
    pub async fn client_scp_control_nominate(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPValue> + ?Sized),
        arg2: &(impl EncodesAs<SCPValue> + ?Sized),
    ) -> Result<bool, RpcError> {
        c.scp_control_nominate(arg0, arg1, arg2).await
    }
    pub async fn client_scp_control_stop_nomination(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_control_stop_nomination(arg0).await
    }
    pub async fn client_scp_control_update_local_quorum_set(
        c: &S,
        arg0: &(impl EncodesAs<SCPQuorumSet> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_control_update_local_quorum_set(arg0).await
    }
    pub async fn client_scp_control_received_envelope(
        c: &S,
        arg0: &(impl EncodesAs<SCPEnvelope> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_control_received_envelope(arg0).await
    }
    pub async fn client_scp_control_set_state_from_envelope(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPEnvelope> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_control_set_state_from_envelope(arg0, arg1).await
    }
    pub async fn client_scp_control_set_qset(
        c: &S,
        arg0: &(impl EncodesAs<SCPQuorumSet> + ?Sized),
    ) -> Result<SCPOptQuorumSet, RpcError> {
        c.scp_control_set_qset(arg0).await
    }
    pub async fn client_scp_control_get_qset(
        c: &S,
        arg0: &(impl EncodesAs<SCPHash> + ?Sized),
    ) -> Result<SCPOptQuorumSet, RpcError> {
        c.scp_control_get_qset(arg0).await
    }
    pub async fn client_scp_control_remove_qset(
        c: &S,
        arg0: &(impl EncodesAs<SCPHash> + ?Sized),
    ) -> Result<SCPOptQuorumSet, RpcError> {
        c.scp_control_remove_qset(arg0).await
    }
    pub async fn client_scp_control_set_lowest_non_externalized_slot(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_control_set_lowest_non_externalized_slot(arg0).await
    }

    pub async fn trait_scp_control_null<T: SCP_Control_SCP_Control1>(
        t: &T,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_control_null().await
    }
    pub async fn trait_scp_control_nominate<T: SCP_Control_SCP_Control1>(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPValue,
        arg2: SCPValue,
    ) -> Result<impl EncodesAs<bool>, ProgramError> {
        t.scp_control_nominate(arg0, arg1, arg2).await
    }
    pub async fn trait_scp_control_stop_nomination<
        T: SCP_Control_SCP_Control1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_control_stop_nomination(arg0).await
    }
    pub async fn trait_scp_control_update_local_quorum_set<
        T: SCP_Control_SCP_Control1,
    >(
        t: &T,
        arg0: SCPQuorumSet,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_control_update_local_quorum_set(arg0).await
    }
    pub async fn trait_scp_control_received_envelope<
        T: SCP_Control_SCP_Control1,
    >(
        t: &T,
        arg0: SCPEnvelope,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_control_received_envelope(arg0).await
    }
    pub async fn trait_scp_control_set_state_from_envelope<
        T: SCP_Control_SCP_Control1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPEnvelope,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_control_set_state_from_envelope(arg0, arg1).await
    }
    pub async fn trait_scp_control_set_qset<T: SCP_Control_SCP_Control1>(
        t: &T,
        arg0: SCPQuorumSet,
    ) -> Result<impl EncodesAs<SCPOptQuorumSet>, ProgramError> {
        t.scp_control_set_qset(arg0).await
    }
    pub async fn trait_scp_control_get_qset<T: SCP_Control_SCP_Control1>(
        t: &T,
        arg0: SCPHash,
    ) -> Result<impl EncodesAs<SCPOptQuorumSet>, ProgramError> {
        t.scp_control_get_qset(arg0).await
    }
    pub async fn trait_scp_control_remove_qset<T: SCP_Control_SCP_Control1>(
        t: &T,
        arg0: SCPHash,
    ) -> Result<impl EncodesAs<SCPOptQuorumSet>, ProgramError> {
        t.scp_control_remove_qset(arg0).await
    }
    pub async fn trait_scp_control_set_lowest_non_externalized_slot<
        T: SCP_Control_SCP_Control1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_control_set_lowest_non_externalized_slot(arg0).await
    }
}

pub mod scp_handle {
    use super::*;
    type S = SCP_Handle_SCP_Handle1_Client;

    pub async fn client_scp_handle_null(c: &S) -> Result<(), RpcError> {
        c.scp_handle_null().await
    }
    pub async fn client_scp_handle_emit_envelope(
        c: &S,
        arg0: &(impl EncodesAs<SCPEnvelope> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_emit_envelope(arg0).await
    }
    pub async fn client_scp_handle_validate_ballot_value(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPValue> + ?Sized),
    ) -> Result<SCPValidationLevel, RpcError> {
        c.scp_handle_validate_ballot_value(arg0, arg1).await
    }
    pub async fn client_scp_handle_validate_nomination_value(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPValue> + ?Sized),
    ) -> Result<SCPValidationLevel, RpcError> {
        c.scp_handle_validate_nomination_value(arg0, arg1).await
    }
    pub async fn client_scp_handle_extract_valid_value(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPValue> + ?Sized),
    ) -> Result<SCPOptValue, RpcError> {
        c.scp_handle_extract_valid_value(arg0, arg1).await
    }
    pub async fn client_scp_handle_combine_candidates(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPValues> + ?Sized),
    ) -> Result<SCPOptValue, RpcError> {
        c.scp_handle_combine_candidates(arg0, arg1).await
    }
    pub async fn client_scp_handle_value_externalized(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPValue> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_value_externalized(arg0, arg1).await
    }
    pub async fn client_scp_handle_nominating_value(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPValue> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_nominating_value(arg0, arg1).await
    }
    pub async fn client_scp_handle_updated_candidate_value(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPValue> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_updated_candidate_value(arg0, arg1).await
    }
    pub async fn client_scp_handle_started_ballot_proto(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPBallot> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_started_ballot_proto(arg0, arg1).await
    }
    pub async fn client_scp_handle_accepted_ballot_prepared(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPBallot> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_accepted_ballot_prepared(arg0, arg1).await
    }
    pub async fn client_scp_handle_confirmed_ballot_prepared(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPBallot> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_confirmed_ballot_prepared(arg0, arg1).await
    }
    pub async fn client_scp_handle_accepted_commit(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPBallot> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_accepted_commit(arg0, arg1).await
    }
    pub async fn client_scp_handle_ballot_did_hear_from_quorum(
        c: &S,
        arg0: &(impl EncodesAs<SCPSlotIndex> + ?Sized),
        arg1: &(impl EncodesAs<SCPBallot> + ?Sized),
    ) -> Result<(), RpcError> {
        c.scp_handle_ballot_did_hear_from_quorum(arg0, arg1).await
    }
    pub async fn client_scp_handle_get_quorum_set(
        c: &S,
        arg0: &(impl EncodesAs<SCPHash> + ?Sized),
    ) -> Result<SCPOptQuorumSet, RpcError> {
        c.scp_handle_get_quorum_set(arg0).await
    }

    pub async fn trait_scp_handle_null<T: SCP_Handle_SCP_Handle1>(
        t: &T,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_null().await
    }
    pub async fn trait_scp_handle_emit_envelope<T: SCP_Handle_SCP_Handle1>(
        t: &T,
        arg0: SCPEnvelope,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_emit_envelope(arg0).await
    }
    pub async fn trait_scp_handle_validate_ballot_value<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPValue,
    ) -> Result<impl EncodesAs<SCPValidationLevel>, ProgramError> {
        t.scp_handle_validate_ballot_value(arg0, arg1).await
    }
    pub async fn trait_scp_handle_validate_nomination_value<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPValue,
    ) -> Result<impl EncodesAs<SCPValidationLevel>, ProgramError> {
        t.scp_handle_validate_nomination_value(arg0, arg1).await
    }
    pub async fn trait_scp_handle_extract_valid_value<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPValue,
    ) -> Result<impl EncodesAs<SCPOptValue>, ProgramError> {
        t.scp_handle_extract_valid_value(arg0, arg1).await
    }
    pub async fn trait_scp_handle_combine_candidates<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPValues,
    ) -> Result<impl EncodesAs<SCPOptValue>, ProgramError> {
        t.scp_handle_combine_candidates(arg0, arg1).await
    }
    pub async fn trait_scp_handle_value_externalized<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPValue,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_value_externalized(arg0, arg1).await
    }
    pub async fn trait_scp_handle_nominating_value<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPValue,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_nominating_value(arg0, arg1).await
    }
    pub async fn trait_scp_handle_updated_candidate_value<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPValue,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_updated_candidate_value(arg0, arg1).await
    }
    pub async fn trait_scp_handle_started_ballot_proto<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPBallot,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_started_ballot_proto(arg0, arg1).await
    }
    pub async fn trait_scp_handle_accepted_ballot_prepared<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPBallot,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_accepted_ballot_prepared(arg0, arg1).await
    }
    pub async fn trait_scp_handle_confirmed_ballot_prepared<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPBallot,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_confirmed_ballot_prepared(arg0, arg1).await
    }
    pub async fn trait_scp_handle_accepted_commit<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPBallot,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_accepted_commit(arg0, arg1).await
    }
    pub async fn trait_scp_handle_ballot_did_hear_from_quorum<
        T: SCP_Handle_SCP_Handle1,
    >(
        t: &T,
        arg0: SCPSlotIndex,
        arg1: SCPBallot,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.scp_handle_ballot_did_hear_from_quorum(arg0, arg1).await
    }
    pub async fn trait_scp_handle_get_quorum_set<T: SCP_Handle_SCP_Handle1>(
        t: &T,
        arg0: SCPHash,
    ) -> Result<impl EncodesAs<SCPOptQuorumSet>, ProgramError> {
        t.scp_handle_get_quorum_set(arg0).await
    }
}
