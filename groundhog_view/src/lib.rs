#![allow(non_snake_case)]
#![allow(unused_imports)]

use groundhog_scp::network::*;
use groundhog_xdr::auth_claim::*;
use groundhog_xdr::auth_token::*;
use groundhog_xdr::contract::*;
use groundhog_xdr::event::*;
use groundhog_xdr::invoke_flag::*;
use groundhog_xdr::transaction::*;
use groundhog_xdr::types::*;
use groundhog_xdr::view::*;

#[rustfmt::skip]
pub mod view_gateway;
pub use view_gateway::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rpc_api::{ProgVers, ProgramError, RpcError, RpcResponse, RpcSender};
    use std::sync::{Arc, Mutex};
    use xdr_common::{BytesM, StringM, VecM};

    const VIEW_GATEWAY_PROG_VERS: ProgVers = ProgVers::new(0x56475759, 1);

    struct ScriptedViewGateway;

    #[async_trait::async_trait]
    impl ViewGateway_ViewGateway1 for ScriptedViewGateway {
        async fn viewgateway_intro(
            &self,
            protocol: ProtocolCommit,
        ) -> Result<ViewGatewayIntroOutcome, ProgramError> {
            assert_eq!(protocol.0, b"hogp/test".to_vec());
            Ok(ViewGatewayIntroOutcome::ViewGatewayIntroSuccess)
        }

        async fn viewgateway_view(
            &self,
            query: TransactionViewQuery,
        ) -> Result<ViewGatewayResult, ProgramError> {
            assert_eq!(
                query.invocation.invoked_address,
                ContractAddress::from([7u8; 32])
            );
            assert_eq!(*query.invocation.entrypoint, [8u8; 8]);
            assert_eq!(query.invocation.gas_limit, 123);
            assert_eq!(query.invocation.calldata.0, vec![9, 10]);
            assert_eq!(
                query.parent_address,
                Some(ContractAddress::from([6u8; 32]))
            );
            assert_eq!(query.forwarded_tokens.0.len(), 0);
            assert_eq!(query.invoke_flags, InvokeFlag::ReadOnly.0);

            Ok(ok(InvocationFrameOutput::Return {
                return_buffer: BytesM(vec![1, 2, 3]),
            }))
        }
    }

    struct LocalSender {
        handler: Arc<dyn rpc_api::RpcHandler>,
        calls: Arc<Mutex<Vec<(ProgVers, u32)>>>,
    }

    #[async_trait::async_trait]
    impl RpcSender for LocalSender {
        async fn send_call(
            &self,
            progvers: ProgVers,
            proc: u32,
            data: &[u8],
        ) -> Result<RpcResponse, RpcError> {
            self.calls
                .lock()
                .expect("calls lock poisoned")
                .push((progvers, proc));
            if progvers != self.handler.prog_vers() {
                return Err(RpcError::ProgUnavail);
            }
            let payload = self.handler.dispatch(proc, data).await?;
            Ok(RpcResponse::from_payload(payload))
        }
    }

    fn round_trip(result: ViewGatewayResult) {
        let encoded =
            xdrs::txs::encode(&result).expect("encode ViewGatewayResult");
        let decoded: ViewGatewayResult =
            xdrs::txs::decode(&encoded).expect("decode ViewGatewayResult");
        assert_eq!(decoded, result);
    }

    fn ok(output: InvocationFrameOutput) -> ViewGatewayResult {
        ViewGatewayResult::ViewGatewayOk {
            payload: ViewGatewayPayload {
                height: BlockHeight::from(42),
                output,
                events: VecM(vec![]),
            },
        }
    }

    fn query() -> TransactionViewQuery {
        TransactionViewQuery {
            invocation: Invocation {
                invoked_address: ContractAddress::from([7u8; 32]),
                entrypoint: ContractEntrypointID::from([8u8; 8]),
                gas_limit: 123,
                calldata: BytesM(vec![9, 10]),
            },
            parent_address: Some(ContractAddress::from([6u8; 32])),
            auth_claim: AuthClaim::kSourceAddressAuthData {
                source_address_claim: AuthClaim_source_address_claim {
                    allowed_keys: 0,
                    required_weight: 0,
                },
            },
            forwarded_tokens: VecM(vec![]),
            invoke_flags: InvokeFlag::ReadOnly.0,
        }
    }

    #[test]
    fn view_gateway_result_round_trips_return_abort_and_fault() {
        round_trip(ok(InvocationFrameOutput::Return {
            return_buffer: BytesM(vec![1, 2, 3]),
        }));
        round_trip(ok(InvocationFrameOutput::Abort {
            abort_buffer: BytesM(vec![4, 5, 6]),
        }));
        round_trip(ok(InvocationFrameOutput::Fault));
    }

    #[test]
    fn view_gateway_result_round_trips_service_failures() {
        round_trip(ViewGatewayResult::ViewGatewayAbandoned);
        round_trip(ViewGatewayResult::ViewGatewayInternalError);
    }

    #[test]
    fn view_gateway_result_round_trips_unknown_default_outcome() {
        round_trip(ViewGatewayResult::Default { outcome: 99 });
    }

    #[test]
    fn view_gateway_types_have_json_schema() {
        let _ = schemars::schema_for!(ViewGatewayPayload);
        let _ = schemars::schema_for!(ViewGatewayOutcome);
        let _ = schemars::schema_for!(ViewGatewayResult);
        let _ = schemars::schema_for!(ViewGatewayIntroOutcome);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn view_gateway_result_serde_round_trips() {
        let result = ok(InvocationFrameOutput::Return {
            return_buffer: BytesM(vec![1, 2, 3]),
        });
        let json = serde_json::to_value(&result)
            .expect("serialize ViewGatewayResult");
        let decoded: ViewGatewayResult = serde_json::from_value(json)
            .expect("deserialize ViewGatewayResult");
        assert_eq!(decoded, result);
    }

    #[tokio::test]
    async fn generated_client_dispatches_intro_and_view() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let handler = Arc::new(ViewGateway_ViewGateway1_Dispatcher::new(
            Box::new(ScriptedViewGateway),
        ));
        let client =
            ViewGateway_ViewGateway1_Client::new(Arc::new(LocalSender {
                handler,
                calls: calls.clone(),
            }));

        let intro = client
            .viewgateway_intro(&StringM(b"hogp/test".to_vec()))
            .await
            .expect("intro should round-trip through dispatcher");
        assert_eq!(intro, ViewGatewayIntroOutcome::ViewGatewayIntroSuccess);

        let result = client
            .viewgateway_view(&query())
            .await
            .expect("view should round-trip through dispatcher");

        match result {
            ViewGatewayResult::ViewGatewayOk { payload } => {
                assert_eq!(*payload.height, 42);
                assert_eq!(
                    payload.output,
                    InvocationFrameOutput::Return {
                        return_buffer: BytesM(vec![1, 2, 3]),
                    }
                );
            }
            other => panic!("unexpected view result: {other:?}"),
        }

        assert_eq!(
            *calls.lock().expect("calls lock poisoned"),
            vec![(VIEW_GATEWAY_PROG_VERS, 0), (VIEW_GATEWAY_PROG_VERS, 1)]
        );
    }
}
