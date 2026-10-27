#![allow(non_snake_case)]
#![allow(unused_imports)]

use thruput_xdr::auth_claim::*;
use thruput_xdr::auth_token::*;
use thruput_xdr::contract::*;
use thruput_xdr::event::*;
use thruput_xdr::invoke_flag::*;
use thruput_xdr::network::*;
use thruput_xdr::syscall_error::*;
use thruput_xdr::transaction::*;
use thruput_xdr::types::*;
use thruput_xdr::view::*;

#[rustfmt::skip]
pub mod view_gateway;
pub use view_gateway::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use thruput_rpc_api::{
        ProgVers, ProgramError, RpcError, RpcResponse, RpcSender,
    };
    use xdr_common::{BytesM, StringM, VecM};

    const VIEW_GATEWAY_PROG_VERS: ProgVers = ProgVers::new(0x56475759, 1);

    /// Stand-in protocol token. Fixed width, so it has to fill the array.
    const TEST_PROTOCOL: ProtocolCommit = *b"hogp/0123456789abcde";

    struct ScriptedViewGateway;

    #[allow(refining_impl_trait)]
    impl ViewGateway_ViewGateway1 for ScriptedViewGateway {
        async fn viewgateway_view_batch(
            &self,
            request: ViewGatewayBatchQuery,
        ) -> Result<ViewGatewayBatchResult, ProgramError> {
            assert_eq!(request.calls.0, vec![query(), query()]);
            assert_eq!(request.min_height, Some(BlockHeight::from(42)));
            assert_eq!(request.budget_millis, 1000);
            assert_eq!(request.max_response_bytes, 1_048_576);
            Ok(ViewGatewayBatchResult::ViewGatewayBatchOk {
                payload: ViewGatewayBatchPayload {
                    height: BlockHeight::from(42),
                    results: VecM(vec![
                        TransactionViewResult {
                            gas_consumed: 17,
                            output: InvocationFrameOutput::Return {
                                return_buffer: BytesM(vec![1]),
                            },
                            events: VecM(vec![]),
                        },
                        TransactionViewResult {
                            gas_consumed: 23,
                            output: InvocationFrameOutput::Fault,
                            events: VecM(vec![]),
                        },
                    ]),
                },
            })
        }

        async fn viewgateway_intro(
            &self,
            protocol: ProtocolCommit,
        ) -> Result<ViewGatewayIntroOutcome, ProgramError> {
            assert_eq!(protocol, TEST_PROTOCOL);
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
            assert_eq!(
                query.invoke_flags,
                InvokeFlag::ReadOnly.0.cast_unsigned()
            );

            Ok(ok(InvocationFrameOutput::Return {
                return_buffer: BytesM(vec![1, 2, 3]),
            }))
        }
    }

    struct LocalSender {
        handler: Arc<dyn thruput_rpc_api::RpcHandler>,
        calls: Arc<Mutex<Vec<(ProgVers, u32)>>>,
    }

    #[async_trait::async_trait]
    impl RpcSender for LocalSender {
        fn call_buf(&self) -> thruput_rpc_api::CallBuf {
            thruput_rpc_api::CallBuf::new()
        }

        async fn send_call(
            &self,
            progvers: ProgVers,
            proc: u32,
            call: thruput_rpc_api::CallBuf,
        ) -> Result<RpcResponse, RpcError> {
            self.calls
                .lock()
                .expect("calls lock poisoned")
                .push((progvers, proc));
            if progvers != self.handler.prog_vers() {
                return Err(RpcError::ProgUnavail);
            }
            let payload = self.handler.dispatch(proc, call.payload()).await?;
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
                gas_consumed: 77,
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
            invoke_flags: InvokeFlag::ReadOnly.0.cast_unsigned(),
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
        let _ = schemars::schema_for!(ViewGatewayBatchQuery);
        let _ = schemars::schema_for!(ViewGatewayBatchResult);
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
    async fn generated_client_dispatches_intro_view_and_batch() {
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
            .viewgateway_intro(&TEST_PROTOCOL)
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

        let batch = ViewGatewayBatchQuery {
            calls: VecM(vec![query(), query()]),
            min_height: Some(BlockHeight::from(42)),
            budget_millis: 1000,
            max_response_bytes: 1_048_576,
        };
        let result = client
            .viewgateway_view_batch(&batch)
            .await
            .expect("batch should round-trip through dispatcher");
        assert_eq!(
            result,
            ScriptedViewGateway
                .viewgateway_view_batch(batch)
                .await
                .unwrap()
        );
        assert_eq!(
            *calls.lock().expect("calls lock poisoned"),
            vec![
                (VIEW_GATEWAY_PROG_VERS, 0),
                (VIEW_GATEWAY_PROG_VERS, 1),
                (VIEW_GATEWAY_PROG_VERS, 2)
            ]
        );
    }

    #[test]
    fn batch_failures_preserve_height_details_and_unknown_outcomes() {
        let mut failures =
            vec![ViewGatewayBatchResult::ViewGatewayBatchHeightUnavailable {
                height_unavailable: ViewGatewayHeightUnavailable {
                    requested: BlockHeight::from(50),
                    observed: BlockHeight::from(42),
                },
            }];
        failures.extend(
            [1, 3, 4, 5, 6, 99]
                .map(|outcome| ViewGatewayBatchResult::Default { outcome }),
        );
        for result in failures {
            let encoded = xdrs::txs::encode(&result).unwrap();
            let decoded: ViewGatewayBatchResult =
                xdrs::txs::decode(&encoded).unwrap();
            assert_eq!(decoded, result);
        }
    }
}
