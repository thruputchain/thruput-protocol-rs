//! Signature pins for `thruput_rpc` (generated: src/test_rpc.rs).

use thruput_rpc::{
    Data, blank_rpc_blank_rpc_v1, blank_rpc_blank_rpc_v1_Client,
};
use thruput_rpc_api::{ProgramError, RpcError};
use xdrs::EncodesAs;

pub mod blank_rpc {
    use super::*;
    type S = blank_rpc_blank_rpc_v1_Client;

    pub async fn client_null(c: &S) -> Result<(), RpcError> {
        c.null().await
    }
    pub async fn client_echo(
        c: &S,
        arg0: &(impl EncodesAs<Data> + ?Sized),
    ) -> Result<Data, RpcError> {
        c.echo(arg0).await
    }

    pub async fn trait_null<T: blank_rpc_blank_rpc_v1>(
        t: &T,
    ) -> Result<impl EncodesAs<()>, ProgramError> {
        t.null().await
    }
    pub async fn trait_echo<T: blank_rpc_blank_rpc_v1>(
        t: &T,
        arg0: Data,
    ) -> Result<impl EncodesAs<Data>, ProgramError> {
        t.echo(arg0).await
    }
}
