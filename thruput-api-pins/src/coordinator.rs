//! Signature pins for `thruput_coordinator` (generated: src/discovery.rs, src/dogwood.rs).

use thruput_coordinator::{
    Discovery_Discovery1, Discovery_Discovery1_Client, Dogwood_Dogwood1,
    Dogwood_Dogwood1_Client, DogwoodDeliveryEnvelope, KnownQuorumSets,
};
use thruput_rpc_api::{ProgramError, RpcError};
use xdrs::EncodesAs;

pub mod discovery {
    use super::*;
    type S = Discovery_Discovery1_Client;

    pub async fn client_discovery_null(c: &S) -> Result<(), RpcError> {
        c.discovery_null().await
    }
    pub async fn client_discovery_update_quorums(
        c: &S,
        arg0: &(impl EncodesAs<KnownQuorumSets> + ?Sized),
    ) -> Result<KnownQuorumSets, RpcError> {
        c.discovery_update_quorums(arg0).await
    }

    pub async fn trait_discovery_null<T: Discovery_Discovery1>(
        t: &T,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.discovery_null().await
    }
    pub async fn trait_discovery_update_quorums<T: Discovery_Discovery1>(
        t: &T,
        arg0: KnownQuorumSets,
    ) -> Result<impl EncodesAs<KnownQuorumSets>, ProgramError> {
        t.discovery_update_quorums(arg0).await
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
