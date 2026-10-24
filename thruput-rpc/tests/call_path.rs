//! The generated `blank_rpc` client and dispatcher, end to end.
//!
//! Over a `LoopbackLink` a call takes the same path it does on a connection:
//! the handler runs on its own task, which decodes the reply into the
//! generated client's `out` slot while the client's future waits. There are
//! no sockets, so these run under Miri (see `scripts/ci-check.sh`). The
//! tests at the end repeat the round trips over real TCP connections and
//! pools, outside Miri.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use thruput_rpc::{
    Data, blank_rpc_blank_rpc_v1, blank_rpc_blank_rpc_v1_Client,
    blank_rpc_blank_rpc_v1_Dispatcher,
};
use thruput_rpc_driver::loopback::LoopbackLink;
use thruput_rpc_driver::{
    ProgVers, ProgramError, RpcError, RpcHandler, ServiceMap,
};
use tokio::sync::{Semaphore, mpsc};

/// Echoes, except that an echo of `HOLD` reports it has started and then
/// waits for a permit, noting if it is dropped before getting one.
struct Echo {
    entered: Option<mpsc::UnboundedSender<()>>,
    permits: Arc<Semaphore>,
    dropped: Arc<AtomicBool>,
}

const HOLD: &[u8] = b"hold";

impl Echo {
    fn new() -> Self {
        Self {
            entered: None,
            permits: Arc::new(Semaphore::new(0)),
            dropped: Arc::new(AtomicBool::new(false)),
        }
    }
}

struct SetOnDrop(Arc<AtomicBool>);

impl Drop for SetOnDrop {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

#[async_trait::async_trait]
impl blank_rpc_blank_rpc_v1 for Echo {
    async fn null(&self) -> Result<(), ProgramError> {
        Ok(())
    }

    async fn echo(&self, arg0: Data) -> Result<Data, ProgramError> {
        if *arg0 == *HOLD {
            let unfinished = SetOnDrop(self.dropped.clone());
            if let Some(entered) = &self.entered {
                let _ = entered.send(());
            }
            self.permits.acquire().await.unwrap().forget();
            std::mem::forget(unfinished);
        }
        Ok(arg0)
    }
}

fn services(echo: Echo) -> ServiceMap {
    let mut services = ServiceMap::new();
    services.add_service(Box::new(blank_rpc_blank_rpc_v1_Dispatcher::new(
        Box::new(echo),
    )));
    services
}

/// A client linked to `services`, and the link.
fn loopback(
    services: ServiceMap,
) -> (LoopbackLink, blank_rpc_blank_rpc_v1_Client) {
    let link = LoopbackLink::connect(ServiceMap::new(), services);
    let (client, _server) = link.senders();
    (link, blank_rpc_blank_rpc_v1_Client::new(Arc::new(client)))
}

fn hold() -> Data {
    HOLD.to_vec().try_into().unwrap()
}

fn data(len: usize) -> Data {
    (0..len)
        .map(|i| i as u8)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

// Under Miri every byte is interpreted, so keep the payloads small there.
const SIZES: &[usize] = if cfg!(miri) {
    &[0, 1, 3, 4, 5, 255, 1024]
} else {
    &[0, 1, 3, 4, 5, 255, 1024, 64 * 1024, 1 << 20]
};

async fn round_trips(client: &blank_rpc_blank_rpc_v1_Client) {
    client.null().await.unwrap();
    for &len in SIZES {
        let sent = data(len);
        assert_eq!(client.echo(&sent).await.unwrap(), sent, "{len} bytes");
    }
}

async fn concurrent_echoes(
    client: &Arc<blank_rpc_blank_rpc_v1_Client>,
    calls: usize,
) {
    let tasks: Vec<_> = (0..calls)
        .map(|i| {
            let client = client.clone();
            tokio::spawn(async move {
                let sent = data(i * 37);
                assert_eq!(client.echo(&sent).await.unwrap(), sent);
            })
        })
        .collect();
    for task in tasks {
        task.await.unwrap();
    }
}

#[tokio::test]
async fn loopback_round_trips() {
    let (_link, client) = loopback(services(Echo::new()));
    round_trips(&client).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn loopback_concurrent_calls_each_get_their_own_reply() {
    let (_link, client) = loopback(services(Echo::new()));
    concurrent_echoes(&Arc::new(client), if cfg!(miri) { 8 } else { 64 })
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn dropping_a_waiting_call_drops_its_handler_and_the_link_keeps_working()
{
    let (entered, mut entries) = mpsc::unbounded_channel();
    let echo = Echo {
        entered: Some(entered),
        ..Echo::new()
    };
    let (permits, dropped) = (echo.permits.clone(), echo.dropped.clone());
    let (_link, client) = loopback(services(echo));
    let client = Arc::new(client);

    let held = tokio::spawn({
        let client = client.clone();
        async move { client.echo(&hold()).await }
    });
    entries.recv().await.unwrap();
    held.abort();
    assert!(held.await.unwrap_err().is_cancelled());

    // The handler task is aborted with its call; a permit now finds no one.
    permits.add_permits(1);
    while !dropped.load(Ordering::SeqCst) {
        tokio::task::yield_now().await;
    }
    round_trips(&client).await;
}

#[tokio::test]
async fn poisoning_the_link_fails_a_waiting_call() {
    let (entered, mut entries) = mpsc::unbounded_channel();
    let echo = Echo {
        entered: Some(entered),
        ..Echo::new()
    };
    let (link, client) = loopback(services(echo));
    let client = Arc::new(client);

    let held = tokio::spawn({
        let client = client.clone();
        async move { client.echo(&hold()).await }
    });
    entries.recv().await.unwrap();
    link.poison();
    assert!(matches!(held.await.unwrap(), Err(RpcError::SystemError)));
    assert!(matches!(client.null().await, Err(RpcError::SystemError)));
}

/// Answers every `blank_rpc` call with the same bytes.
struct Fixed(Vec<u8>);

#[async_trait::async_trait]
impl RpcHandler for Fixed {
    async fn dispatch(
        &self,
        _proc: u32,
        _data: &[u8],
    ) -> Result<Vec<u8>, RpcError> {
        Ok(self.0.clone())
    }

    fn prog_vers(&self) -> ProgVers {
        blank_rpc_blank_rpc_v1_Dispatcher::new(Box::new(Echo::new()))
            .prog_vers()
    }
}

#[tokio::test]
async fn a_reply_that_does_not_decode_is_a_system_error() {
    // A length prefix promising more bytes than follow: the decoder fails
    // before filling the client's slot.
    let mut short = ServiceMap::new();
    short.add_service(Box::new(Fixed(vec![0, 0, 0, 9, 1, 2, 3, 4])));
    let (_link, client) = loopback(short);
    assert!(matches!(
        client.echo(&data(4)).await,
        Err(RpcError::SystemError)
    ));
    // A void reply with bytes left over.
    assert!(matches!(client.null().await, Err(RpcError::SystemError)));
}

#[cfg(not(miri))]
mod tcp {
    use super::*;
    use thruput_rpc_driver::{
        DynamicPool, MultiplexPool, RpcSender, RpcServer, connect_client,
    };

    async fn server() -> std::net::SocketAddr {
        let listener =
            tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let _ = RpcServer::new(listener, services(Echo::new()))
            .start()
            .await;
        addr
    }

    async fn senders(
        addr: &std::net::SocketAddr,
    ) -> Vec<(&'static str, Arc<dyn RpcSender>)> {
        vec![
            (
                "connection",
                connect_client(addr).await.unwrap().into_handle(),
            ),
            (
                "dynamic pool",
                Arc::new(DynamicPool::new(
                    *addr,
                    4,
                    std::time::Duration::from_secs(60),
                    None,
                )),
            ),
            (
                "multiplex pool",
                Arc::new(MultiplexPool::connect(addr, 2).await.unwrap()),
            ),
        ]
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn round_trips_over_tcp() {
        let addr = server().await;
        for (name, sender) in senders(&addr).await {
            let client = Arc::new(blank_rpc_blank_rpc_v1_Client::new(sender));
            round_trips(&client).await;
            concurrent_echoes(&client, 64).await;
            eprintln!("{name}: ok");
        }
    }
}
