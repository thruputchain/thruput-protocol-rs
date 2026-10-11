#![allow(non_snake_case)]
#![allow(unused_imports)]

use thruput_scp::*;
use thruput_xdr::types::*;
pub use thruput_xdr::validator::*;

#[rustfmt::skip]
pub mod discovery;
pub use discovery::*;

#[rustfmt::skip]
pub mod config;
pub use config::*;

#[rustfmt::skip]
pub mod dogwood_kit;
pub use dogwood_kit::*;

#[rustfmt::skip]
pub mod dogwood;
pub use dogwood::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dogwood_delivery_envelope_round_trip() {
        let envelope = DogwoodDeliveryEnvelope {
            signature: Signature::Ed25519 {
                ed25519: Ed25519Signature::from([0u8; 64]),
            },
            delivery: DogwoodDelivery {
                stream: DogwoodStream {
                    source_id: ValidatorID::from([0xAB; 32]),
                    stream_id: 42,
                },
                route: DogwoodRoute {
                    route_hash: Some([0xCD; 32].into()),
                    route_nodes: None,
                },
                pkg: DogwoodPackage {
                    transmission: DogwoodTransmission {
                        transmission_id: 7,
                        r#type:
                            DogwoodTransmissionType::DogwoodTransmissionPayload,
                        num_payload_bytes: 1024,
                        num_payload_sheaves: 4,
                        num_parity_sheaves: 2,
                        max_bytes_per_chunk: 256,
                        shuffle_seed: 99,
                    },
                    chunk: DogwoodChunk {
                        sheaf_index: 1,
                        chunk_index: 3,
                        chunk: vec![0xDEu8, 0xAD, 0xBE, 0xEF]
                            .try_into()
                            .unwrap(),
                    },
                },
            },
        };

        let encoded = xdrs::txs::encode(&envelope).expect("encode failed");
        let decoded: DogwoodDeliveryEnvelope =
            xdrs::txs::decode(&encoded).expect("decode failed");
        assert_eq!(envelope, decoded);
    }
}
