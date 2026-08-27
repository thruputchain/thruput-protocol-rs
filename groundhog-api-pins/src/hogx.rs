//! Signature pins for `groundhog_hogx` (generated: src/hogx_agent.rs).

use groundhog_hogx::{
    HogxAgent_HogxAgent1, HogxAgent_HogxAgent1_Client,
    HogxHostConfigureResult, HogxHostQueryResult, HogxTail_HogxTail1,
    HogxTail_HogxTail1_Client, HogxTailChunk, HogxTailKick,
    HogxUnitClearResult, HogxUnitListResult, HogxUnitQueryResult,
    HogxUnitStartResult, HogxUnitStopResult, HogxUnitTailResult, HostConfig,
    UnitDef, UnitID,
};
use rpc_api::{ProgramError, RpcError};

pub mod hogx_agent {
    use super::*;
    type S = HogxAgent_HogxAgent1_Client;

    pub async fn client_hogxagent_null(c: &S) -> Result<(), RpcError> {
        c.hogxagent_null().await
    }
    pub async fn client_hogxagent_host_configure(
        c: &S,
        arg0: &HostConfig,
    ) -> Result<HogxHostConfigureResult, RpcError> {
        c.hogxagent_host_configure(arg0).await
    }
    pub async fn client_hogxagent_host_query(
        c: &S,
    ) -> Result<HogxHostQueryResult, RpcError> {
        c.hogxagent_host_query().await
    }
    pub async fn client_hogxagent_unit_start(
        c: &S,
        arg0: &UnitDef,
    ) -> Result<HogxUnitStartResult, RpcError> {
        c.hogxagent_unit_start(arg0).await
    }
    pub async fn client_hogxagent_unit_stop(
        c: &S,
        arg0: &UnitID,
    ) -> Result<HogxUnitStopResult, RpcError> {
        c.hogxagent_unit_stop(arg0).await
    }
    pub async fn client_hogxagent_unit_query(
        c: &S,
        arg0: &UnitID,
    ) -> Result<HogxUnitQueryResult, RpcError> {
        c.hogxagent_unit_query(arg0).await
    }
    pub async fn client_hogxagent_unit_list(
        c: &S,
    ) -> Result<HogxUnitListResult, RpcError> {
        c.hogxagent_unit_list().await
    }
    pub async fn client_hogxagent_unit_clear(
        c: &S,
        arg0: &UnitID,
    ) -> Result<HogxUnitClearResult, RpcError> {
        c.hogxagent_unit_clear(arg0).await
    }
    pub async fn client_hogxagent_unit_tail(
        c: &S,
        arg0: &HogxTailKick,
    ) -> Result<HogxUnitTailResult, RpcError> {
        c.hogxagent_unit_tail(arg0).await
    }

    pub async fn trait_hogxagent_null<T: HogxAgent_HogxAgent1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.hogxagent_null().await
    }
    pub async fn trait_hogxagent_host_configure<T: HogxAgent_HogxAgent1>(
        t: &T,
        arg0: HostConfig,
    ) -> Result<HogxHostConfigureResult, ProgramError> {
        t.hogxagent_host_configure(arg0).await
    }
    pub async fn trait_hogxagent_host_query<T: HogxAgent_HogxAgent1>(
        t: &T,
    ) -> Result<HogxHostQueryResult, ProgramError> {
        t.hogxagent_host_query().await
    }
    pub async fn trait_hogxagent_unit_start<T: HogxAgent_HogxAgent1>(
        t: &T,
        arg0: UnitDef,
    ) -> Result<HogxUnitStartResult, ProgramError> {
        t.hogxagent_unit_start(arg0).await
    }
    pub async fn trait_hogxagent_unit_stop<T: HogxAgent_HogxAgent1>(
        t: &T,
        arg0: UnitID,
    ) -> Result<HogxUnitStopResult, ProgramError> {
        t.hogxagent_unit_stop(arg0).await
    }
    pub async fn trait_hogxagent_unit_query<T: HogxAgent_HogxAgent1>(
        t: &T,
        arg0: UnitID,
    ) -> Result<HogxUnitQueryResult, ProgramError> {
        t.hogxagent_unit_query(arg0).await
    }
    pub async fn trait_hogxagent_unit_list<T: HogxAgent_HogxAgent1>(
        t: &T,
    ) -> Result<HogxUnitListResult, ProgramError> {
        t.hogxagent_unit_list().await
    }
    pub async fn trait_hogxagent_unit_clear<T: HogxAgent_HogxAgent1>(
        t: &T,
        arg0: UnitID,
    ) -> Result<HogxUnitClearResult, ProgramError> {
        t.hogxagent_unit_clear(arg0).await
    }
    pub async fn trait_hogxagent_unit_tail<T: HogxAgent_HogxAgent1>(
        t: &T,
        arg0: HogxTailKick,
    ) -> Result<HogxUnitTailResult, ProgramError> {
        t.hogxagent_unit_tail(arg0).await
    }
}

pub mod hogx_tail {
    use super::*;
    type S = HogxTail_HogxTail1_Client;

    pub async fn client_hogxtail_null(c: &S) -> Result<(), RpcError> {
        c.hogxtail_null().await
    }
    pub async fn client_hogxtail_deliver(
        c: &S,
        arg0: &HogxTailChunk,
    ) -> Result<bool, RpcError> {
        c.hogxtail_deliver(arg0).await
    }

    pub async fn trait_hogxtail_null<T: HogxTail_HogxTail1>(
        t: &T,
    ) -> Result<(), ProgramError> {
        t.hogxtail_null().await
    }
    pub async fn trait_hogxtail_deliver<T: HogxTail_HogxTail1>(
        t: &T,
        arg0: HogxTailChunk,
    ) -> Result<bool, ProgramError> {
        t.hogxtail_deliver(arg0).await
    }
}
