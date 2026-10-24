//! Signature pins for `thruput_view` (generated: src/view_gateway.rs).

use thruput_rpc_api::{ProgramError, RpcError};
use thruput_view::{
    ViewGateway_ViewGateway1, ViewGateway_ViewGateway1_Client,
    ViewGatewayBatchQuery, ViewGatewayBatchResult, ViewGatewayIntroOutcome,
    ViewGatewayResult,
};
use thruput_xdr::network::ProtocolCommit;
use thruput_xdr::view::TransactionViewQuery;
use xdrs::EncodesAs;

pub mod view_gateway {
    use super::*;
    type S = ViewGateway_ViewGateway1_Client;

    pub async fn client_viewgateway_intro(
        c: &S,
        arg0: &(impl EncodesAs<ProtocolCommit> + ?Sized),
    ) -> Result<ViewGatewayIntroOutcome, RpcError> {
        c.viewgateway_intro(arg0).await
    }
    pub async fn client_viewgateway_view(
        c: &S,
        arg0: &(impl EncodesAs<TransactionViewQuery> + ?Sized),
    ) -> Result<ViewGatewayResult, RpcError> {
        c.viewgateway_view(arg0).await
    }
    pub async fn client_viewgateway_view_batch(
        c: &S,
        arg0: &(impl EncodesAs<ViewGatewayBatchQuery> + ?Sized),
    ) -> Result<ViewGatewayBatchResult, RpcError> {
        c.viewgateway_view_batch(arg0).await
    }

    pub async fn trait_viewgateway_intro<T: ViewGateway_ViewGateway1>(
        t: &T,
        arg0: ProtocolCommit,
    ) -> Result<impl EncodesAs<ViewGatewayIntroOutcome>, ProgramError> {
        t.viewgateway_intro(arg0).await
    }
    pub async fn trait_viewgateway_view<T: ViewGateway_ViewGateway1>(
        t: &T,
        arg0: TransactionViewQuery,
    ) -> Result<impl EncodesAs<ViewGatewayResult>, ProgramError> {
        t.viewgateway_view(arg0).await
    }
    pub async fn trait_viewgateway_view_batch<T: ViewGateway_ViewGateway1>(
        t: &T,
        arg0: ViewGatewayBatchQuery,
    ) -> Result<impl EncodesAs<ViewGatewayBatchResult>, ProgramError> {
        t.viewgateway_view_batch(arg0).await
    }
}
