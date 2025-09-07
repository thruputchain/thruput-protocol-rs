use groundhog_rpc::{blank_rpc_blank_rpc_v1, Data};
use async_trait::async_trait;

pub struct BlankRpcServer;

impl BlankRpcServer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl blank_rpc_blank_rpc_v1 for BlankRpcServer {
    async fn null(&self) -> Result<(), ::rpc_driver::ProgramError> {
        // Simple null operation - just return success
        Ok(())
    }
    
    async fn echo(&self, data: Data) -> Result<Data, ::rpc_driver::ProgramError> {
        // Echo the data back
        Ok(data)
    }
}
