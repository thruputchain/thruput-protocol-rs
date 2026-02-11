use async_trait::async_trait;
use groundhog_rpc::{Data, blank_rpc_blank_rpc_v1};

pub struct BlankRpcServer;

impl BlankRpcServer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl blank_rpc_blank_rpc_v1 for BlankRpcServer {
    async fn null(
        &self,
        _client: &::std::sync::Arc<::rpc_driver::ClientHandle>,
    ) -> Result<(), ::rpc_driver::ProgramError> {
        // Simple null operation - just return success
        Ok(())
    }

    async fn echo(
        &self,
        _client: &::std::sync::Arc<::rpc_driver::ClientHandle>,
        data: Data,
    ) -> Result<Data, ::rpc_driver::ProgramError> {
        // Echo the data back
        Ok(data)
    }
}
