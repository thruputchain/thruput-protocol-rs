mod server_impl;

use std::error::Error;
use groundhog_rpc::blank_rpc_blank_rpc_v1_Dispatcher;
use rpc_driver::ServiceMap;
use server_impl::BlankRpcServer;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("RPC Throughput Server starting...");
    
    // Create service map
    let mut service_map = ServiceMap::new();
    service_map.add_service(Box::new(blank_rpc_blank_rpc_v1_Dispatcher::new(Box::new(BlankRpcServer::new()))));

    let listener = TcpListener::bind("[::]:12345").await?;
    let addr = listener.local_addr().unwrap();
    println!("RPC Throughput Server listening on {}", addr);
    
    let server = rpc_driver::RpcServer::new(service_map, listener);

    tokio::spawn(async move {
        server.run().await.map_err(|e| eprintln!("Error running RPC server: {}", e)).unwrap();
    });

    println!("RPC Throughput Server running with blank_rpc implementation. Press Ctrl+C to stop.");
    
    // Keep the server running
    tokio::signal::ctrl_c().await?;
    println!("Shutting down RPC Throughput Server...");
    
    Ok(())
}
