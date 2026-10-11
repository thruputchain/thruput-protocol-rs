//! Signature pins for `thruput_db_gateway` (generated: src/db_gateway.rs).

use rpc_api::{ProgramError, RpcError};
use thruput_db_gateway::{
    AddressAndKeyBatch, DbGateway_DbGateway1, DbGateway_DbGateway1_Client,
    DbGatewayIntroOutcome, DbGatewayResult,
};
use thruput_xdr::network::ProtocolCommit;
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

    pub async fn trait_dbgateway_intro<T: DbGateway_DbGateway1>(
        t: &T,
        arg0: ProtocolCommit,
    ) -> Result<DbGatewayIntroOutcome, ProgramError> {
        t.dbgateway_intro(arg0).await
    }
    pub async fn trait_dbgateway_get_items<T: DbGateway_DbGateway1>(
        t: &T,
        arg0: AddressAndKeyBatch,
    ) -> Result<DbGatewayResult, ProgramError> {
        t.dbgateway_get_items(arg0).await
    }
}
