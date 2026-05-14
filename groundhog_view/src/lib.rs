#![allow(non_snake_case)]
#![allow(unused_imports)]

pub use groundhog_scp::network::*;
pub use groundhog_xdr::auth_claim::*;
pub use groundhog_xdr::auth_token::*;
pub use groundhog_xdr::event::*;
pub use groundhog_xdr::invoke_flag::*;
pub use groundhog_xdr::transaction::*;
pub use groundhog_xdr::types::*;
pub use groundhog_xdr::view::*;

#[rustfmt::skip]
pub mod view_gateway;
pub use view_gateway::*;

#[cfg(test)]
mod tests {
    use super::*;
    use xdr_common::{BytesM, VecM};

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
}
