//! Signature pins for `thruput_db_gateway` (generated: src/db_gateway.rs).

use thruput_db_gateway::{
    AddressAndKeyBatch, DbGateway_DbGateway1, DbGateway_DbGateway1_Client,
    DbGatewayConfigResult, DbGatewayContractResult, DbGatewayIntroOutcome,
    DbGatewayResult,
};
use thruput_rpc_api::{ProgramError, RpcError};
use thruput_xdr::network::ProtocolCommit;
use thruput_xdr::types::SHA256Hash;
use xdrs::EncodesAs;

pub mod db_gateway {
    use super::*;
    type S = DbGateway_DbGateway1_Client;

    pub async fn client_dbgateway_intro(
        c: &S,
        arg0: &(impl EncodesAs<ProtocolCommit> + ?Sized),
    ) -> Result<DbGatewayIntroOutcome, RpcError> {
        c.dbgateway_intro(arg0).await
    }
    pub async fn client_dbgateway_get_items(
        c: &S,
        arg0: &(impl EncodesAs<AddressAndKeyBatch> + ?Sized),
    ) -> Result<DbGatewayResult, RpcError> {
        c.dbgateway_get_items(arg0).await
    }
    pub async fn client_dbgateway_get_config(
        c: &S,
    ) -> Result<DbGatewayConfigResult, RpcError> {
        c.dbgateway_get_config().await
    }

    pub async fn client_dbgateway_get_contract(
        c: &S,
        arg0: &(impl EncodesAs<SHA256Hash> + ?Sized),
    ) -> Result<DbGatewayContractResult, RpcError> {
        c.dbgateway_get_contract(arg0).await
    }

    pub async fn trait_dbgateway_intro<T: DbGateway_DbGateway1>(
        t: &T,
        arg0: ProtocolCommit,
    ) -> Result<impl EncodesAs<DbGatewayIntroOutcome>, ProgramError> {
        t.dbgateway_intro(arg0).await
    }
    pub async fn trait_dbgateway_get_items<T: DbGateway_DbGateway1>(
        t: &T,
        arg0: AddressAndKeyBatch,
    ) -> Result<impl EncodesAs<DbGatewayResult>, ProgramError> {
        t.dbgateway_get_items(arg0).await
    }
    pub async fn trait_dbgateway_get_config<T: DbGateway_DbGateway1>(
        t: &T,
    ) -> Result<impl EncodesAs<DbGatewayConfigResult>, ProgramError> {
        t.dbgateway_get_config().await
    }

    pub async fn trait_dbgateway_get_contract<T: DbGateway_DbGateway1>(
        t: &T,
        arg0: SHA256Hash,
    ) -> Result<impl EncodesAs<DbGatewayContractResult>, ProgramError> {
        t.dbgateway_get_contract(arg0).await
    }
}
