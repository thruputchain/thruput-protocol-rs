//! XDR encode/decode roundtrip tests for the central protocol types.
//!
//! These tests exercise the `xdrs` code path (both the traditional TXS
//! and compact CXS serializations) end to end: encode a hand-built
//! value, decode it, and compare. They also cover the decode-side
//! hardening contract: bounded containers reject over-limit input, and
//! truncated or length-corrupted input returns `Err` instead of
//! panicking or over-allocating.

use thruput_xdr::address_auth::{
    AddressAuthData, PublicKeyConfig, PublicKeyConfigOpt,
    kDefaultNewAddressRequiredWeight,
};
use thruput_xdr::auth_claim::{
    AuthClaim, AuthClaim_source_address_claim, PubkeyAccountPreimage,
    kMaxSigners,
};
use thruput_xdr::contract::{Contract, ContractBlob};
use thruput_xdr::event::{TxEmittedEvent, TxEventLog};
use thruput_xdr::exec_result::ExecResult;
use thruput_xdr::storage_item::{
    AuthDataChangeArgs, AuthDataChangeSigner, Bytestring, StorageDelta,
    StorageItem, kBytestringCeilingLen,
};
use thruput_xdr::transaction::{
    DecoratedSignature, Invocation, InvocationFrameOutput, SignedTransaction,
    Transaction, Uniquifier,
};
use thruput_xdr::types::{
    ContractAddress, Ed25519PublicKey, Ed25519Signature, MlDsa65PublicKey,
    MlDsa65Signature, PublicKey, PublicKeyHash, SHA256Hash, Signature,
    int128_xdr,
};
use xdrs::{BytesM, VecM};

/// Encodes `value` in both XDR serializations and asserts that decoding
/// reproduces it exactly. Returns the TXS encoding for reuse in
/// corruption tests.
fn roundtrip<T>(value: &T) -> Vec<u8>
where
    T: xdrs::WriteXDR
        + for<'r> xdrs::ReadXDR<'r>
        + PartialEq
        + std::fmt::Debug,
{
    let txs_bytes = xdrs::txs::encode(value).expect("txs encode");
    let decoded: T = xdrs::txs::decode(&txs_bytes).expect("txs decode");
    assert_eq!(&decoded, value, "txs roundtrip mismatch");

    let cxs_bytes = xdrs::cxs::encode(value).expect("cxs encode");
    let decoded: T = xdrs::cxs::decode(&cxs_bytes).expect("cxs decode");
    assert_eq!(&decoded, value, "cxs roundtrip mismatch");

    txs_bytes
}

/// Asserts that every strict prefix of a TXS encoding fails to decode
/// (and does not panic). Also checks that one trailing byte is
/// rejected, since `decode` requires the buffer be fully consumed.
fn assert_prefixes_and_suffix_err<T>(bytes: &[u8])
where
    T: for<'r> xdrs::ReadXDR<'r> + std::fmt::Debug,
{
    for len in 0..bytes.len() {
        assert!(
            xdrs::txs::decode::<T>(&bytes[..len]).is_err(),
            "truncation to {len} of {} bytes unexpectedly decoded",
            bytes.len()
        );
    }
    let mut extended = bytes.to_vec();
    extended.push(0);
    assert!(
        xdrs::txs::decode::<T>(&extended).is_err(),
        "trailing garbage unexpectedly accepted"
    );
}

fn sample_pubkey(fill: u8) -> PublicKey {
    PublicKey::Ed25519 {
        ed25519: Ed25519PublicKey::new([fill; 32]),
    }
}

fn sample_invocation() -> Invocation {
    Invocation {
        invoked_address: ContractAddress::new([0xAB; 32]),
        entrypoint: thruput_xdr::contract::ContractEntrypointID::from_u64(
            0x0123_4567_89AB_CDEF,
        ),
        gas_limit: u64::MAX,
        calldata: b"hello groundhog".as_slice().try_into().unwrap(),
    }
}

fn sample_transaction() -> Transaction {
    Transaction {
        invocation: sample_invocation(),
        uniquifier: Uniquifier {
            block_expiry_height: 1234.into(),
            nonce: 0xDEAD_BEEF,
        },
        contracts_to_deploy: vec![
            Contract::kRv32 {
                rv32_contract: ContractBlob {
                    data: b"\x7fELF-ish".as_slice().try_into().unwrap(),
                },
            },
            Contract::kMockTest {
                mock_contract_id: -7,
            },
        ]
        .try_into()
        .unwrap(),
        auth_claim: AuthClaim::kSourceAddressAuthData {
            source_address_claim: AuthClaim_source_address_claim {
                allowed_keys: 0b101,
                required_weight: 2,
            },
        },
    }
}

fn sample_signed_transaction() -> SignedTransaction {
    SignedTransaction {
        tx: sample_transaction(),
        signatures: vec![
            DecoratedSignature {
                public_key_id: 0,
                signature: Signature::Ed25519 {
                    ed25519: Ed25519Signature::from([0x11; 64]),
                },
            },
            DecoratedSignature {
                public_key_id: 3,
                signature: Signature::Ed25519 {
                    ed25519: Ed25519Signature::from([0x22; 64]),
                },
            },
        ]
        .try_into()
        .unwrap(),
        pubkey_claim_key: None,
    }
}

fn sample_auth_data() -> AddressAuthData {
    // Slots 0 and 2 occupied, the rest empty.
    let public_keys = std::array::from_fn(|i| {
        PublicKeyConfigOpt::new(match i {
            0 => Some(PublicKeyConfig {
                key: sample_pubkey(0x42),
                max_weight: 3,
            }),
            2 => Some(PublicKeyConfig {
                key: sample_pubkey(0x43),
                max_weight: 1,
            }),
            _ => None,
        })
    });
    AddressAuthData {
        public_keys,
        required_weight: kDefaultNewAddressRequiredWeight,
        last_written_height: 99.into(),
    }
}

fn sample_signer(fill: u8) -> AuthDataChangeSigner {
    AuthDataChangeSigner {
        public_key_id: fill as u32,
        public_key: PublicKeyConfigOpt::new(Some(PublicKeyConfig {
            key: sample_pubkey(fill),
            max_weight: 2,
        })),
    }
}

#[test]
fn transaction_roundtrip() {
    let bytes = roundtrip(&sample_transaction());
    assert_prefixes_and_suffix_err::<Transaction>(&bytes);
}

#[test]
fn signed_transaction_roundtrip() {
    let bytes = roundtrip(&sample_signed_transaction());
    assert_prefixes_and_suffix_err::<SignedTransaction>(&bytes);
}

#[test]
fn mldsa_pubkey_claim_roundtrip() {
    // A pubkey claim names its key by hash; the key itself, here an ML-DSA-65
    // one, rides alongside the signatures.
    let mut tx = sample_transaction();
    tx.auth_claim = AuthClaim::kPubkeySignature {
        pubkey_preimage: PubkeyAccountPreimage {
            pubkey_hash: PublicKeyHash::new([0x5C; 32]),
            nonce: 7,
        },
    };
    let stx = SignedTransaction {
        tx,
        signatures: vec![DecoratedSignature {
            public_key_id: 0,
            signature: Signature::MlDsa65 {
                mldsa65: MlDsa65Signature::new([0x33; 3309]),
            },
        }]
        .try_into()
        .unwrap(),
        pubkey_claim_key: Some(PublicKey::MlDsa65 {
            mldsa65: MlDsa65PublicKey::new([0x44; 1952]),
        }),
    };
    let bytes = roundtrip(&stx);
    assert_prefixes_and_suffix_err::<SignedTransaction>(&bytes);
}

#[test]
fn contract_variants_roundtrip() {
    roundtrip(&Contract::kRv32 {
        rv32_contract: ContractBlob {
            data: b"rv32 code".as_slice().try_into().unwrap(),
        },
    });
    roundtrip(&Contract::kMockTest {
        mock_contract_id: 42,
    });
    roundtrip(&Contract::kNative {
        native_contract_id: 0.into(),
    });
}

/// Discriminant 0 selected the retired Wasmi arm. Encodings that still
/// carry it must fail to decode rather than resolve to a live contract
/// type.
#[test]
fn retired_contract_discriminant_rejected() {
    let contract = Contract::kRv32 {
        rv32_contract: ContractBlob {
            data: b"rv32 code".as_slice().try_into().unwrap(),
        },
    };

    let mut txs_bytes = xdrs::txs::encode(&contract).expect("txs encode");
    txs_bytes[..4].copy_from_slice(&0i32.to_be_bytes());
    assert!(
        xdrs::txs::decode::<Contract>(&txs_bytes).is_err(),
        "retired discriminant unexpectedly decoded from txs"
    );

    let mut cxs_bytes = xdrs::cxs::encode(&contract).expect("cxs encode");
    cxs_bytes[0] = 0;
    assert!(
        xdrs::cxs::decode::<Contract>(&cxs_bytes).is_err(),
        "retired discriminant unexpectedly decoded from cxs"
    );
}

#[test]
fn storage_delta_variants_roundtrip() {
    roundtrip(&StorageDelta::kDeltaTypeBytestringSet {
        bytestring: b"some value".as_slice().try_into().unwrap(),
    });
    roundtrip(&StorageDelta::kDeltaTypeNonnegativeIntAdd {
        nnint_add: (-3i128).into(),
    });
    roundtrip(&StorageDelta::kDeltaTypeDelete);
    let bytes = roundtrip(&StorageDelta::kDeltaTypeAuthDataChange {
        auth_data_change: AuthDataChangeArgs {
            auth_data_change_signers: vec![sample_signer(1), sample_signer(2)]
                .try_into()
                .unwrap(),
            new_required_weight: Some(4),
        },
    });
    assert_prefixes_and_suffix_err::<StorageDelta>(&bytes);
}

#[test]
fn storage_item_roundtrip() {
    roundtrip(&StorageItem::kItemTypeBytestring {
        bytestring: b"stored bytes".as_slice().try_into().unwrap(),
    });
    let value: int128_xdr = 12_345_678_901_234_567_890_i128.into();
    roundtrip(&StorageItem::kItemTypeNonnegativeInt { nnint: value });
}

#[test]
fn address_auth_data_roundtrip() {
    let bytes = roundtrip(&sample_auth_data());
    assert_prefixes_and_suffix_err::<AddressAuthData>(&bytes);
}

#[test]
fn tx_event_log_roundtrip() {
    let log = TxEventLog {
        tx_hash: SHA256Hash::new([0x99; 32]),
        exec_result: ExecResult(0),
        root_output: InvocationFrameOutput::Return {
            return_buffer: b"ret".as_slice().try_into().unwrap(),
        },
        gas_consumed: 12_345,
        events: vec![TxEmittedEvent {
            contract_address: ContractAddress::new([0x77; 32]),
            num_topics: 2,
            serialized_event: [0u8; 96].as_slice().try_into().unwrap(),
        }]
        .try_into()
        .unwrap(),
    };
    let bytes = roundtrip(&log);
    assert_prefixes_and_suffix_err::<TxEventLog>(&bytes);
}

#[test]
fn empty_containers_roundtrip() {
    // Empty calldata, no contracts to deploy.
    let mut tx = sample_transaction();
    tx.invocation.calldata = BytesM::default();
    tx.contracts_to_deploy = VecM::default();
    roundtrip(&tx);

    // No signatures.
    let stx = SignedTransaction {
        tx,
        signatures: VecM::default(),
        pubkey_claim_key: None,
    };
    roundtrip(&stx);

    // Empty bounded bytestring and empty bounded vec.
    roundtrip(&StorageDelta::kDeltaTypeBytestringSet {
        bytestring: Bytestring::default(),
    });
    roundtrip(&AuthDataChangeArgs {
        auth_data_change_signers: VecM::default(),
        new_required_weight: None,
    });

    // All-empty key slots.
    let auth = AddressAuthData {
        public_keys: std::array::from_fn(|_| PublicKeyConfigOpt::new(None)),
        required_weight: 0,
        last_written_height: 0.into(),
    };
    roundtrip(&auth);
}

#[test]
fn bytestring_at_limit_accepted_over_limit_rejected() {
    // At the limit: accepted and roundtrips.
    let max: Bytestring = vec![0xA5; kBytestringCeilingLen as usize]
        .try_into()
        .unwrap();
    roundtrip(&max);

    // One past the limit: encode through a wider bound, decode as the
    // bounded type. Must fail cleanly, not panic.
    let over: BytesM<{ kBytestringCeilingLen * 2 }> =
        vec![0xA5; kBytestringCeilingLen as usize + 1]
            .try_into()
            .unwrap();
    let bytes = xdrs::txs::encode(&over).unwrap();
    assert!(xdrs::txs::decode::<Bytestring>(&bytes).is_err());
}

#[test]
fn signer_vec_at_limit_accepted_over_limit_rejected() {
    // Exactly kMaxSigners entries: accepted and roundtrips.
    let signers: Vec<_> = (0..kMaxSigners as u8).map(sample_signer).collect();
    let at_limit: VecM<AuthDataChangeSigner, kMaxSigners> =
        signers.try_into().unwrap();
    roundtrip(&at_limit);

    // One more entry: encode through a wider bound, decode as the
    // bounded type. Must fail cleanly, not panic.
    let signers: Vec<_> = (0..=kMaxSigners as u8).map(sample_signer).collect();
    let over: VecM<AuthDataChangeSigner, { kMaxSigners * 2 }> =
        signers.try_into().unwrap();
    let bytes = xdrs::txs::encode(&over).unwrap();
    assert!(
        xdrs::txs::decode::<VecM<AuthDataChangeSigner, kMaxSigners>>(&bytes)
            .is_err()
    );
}

#[test]
fn oversized_length_prefixes_error_without_panicking() {
    // A bounded bytestring whose length prefix exceeds the bound.
    let mut bytes = (kBytestringCeilingLen + 1).to_be_bytes().to_vec();
    bytes.extend_from_slice(&[0u8; 16]);
    assert!(xdrs::txs::decode::<Bytestring>(&bytes).is_err());

    // A length prefix of u32::MAX with almost no data behind it must
    // error out instead of panicking or attempting a huge allocation.
    let huge = u32::MAX.to_be_bytes().to_vec();
    assert!(xdrs::txs::decode::<Bytestring>(&huge).is_err());
    assert!(xdrs::txs::decode::<VecM<Contract>>(&huge).is_err());
    assert!(
        xdrs::txs::decode::<VecM<AuthDataChangeSigner, kMaxSigners>>(&huge)
            .is_err()
    );

    // The same corruption inside a larger structure: patch the calldata
    // length prefix of a valid transaction encoding.
    let tx = sample_transaction();
    let good = xdrs::txs::encode(&tx).unwrap();
    // calldata length sits after invoked_address (32) + entrypoint (8)
    // + gas_limit (8).
    let len_offset = 32 + 8 + 8;
    let mut corrupted = good.clone();
    corrupted[len_offset..len_offset + 4]
        .copy_from_slice(&u32::MAX.to_be_bytes());
    assert!(xdrs::txs::decode::<Transaction>(&corrupted).is_err());
}
