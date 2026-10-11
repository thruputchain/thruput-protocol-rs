//! JSON form of the fixed-size opaque newtypes under the `serde` feature.
//!
//! Signatures, keys, hashes, addresses, topics, entrypoint IDs, and validator
//! IDs serialize through `xdr_common::serde_hex`: a lowercase hex string
//! with no prefix and exactly two digits per byte. Clients parse these
//! strings, so the tests pin the emitted text and the accepted inputs
//! rather than only checking that a value survives a round trip.

use serde::Serialize;
use serde::de::DeserializeOwned;
use thruput_xdr::contract::ContractEntrypointID;
use thruput_xdr::event::Topic;
use thruput_xdr::transaction::DecoratedSignature;
use thruput_xdr::types::{
    ContractAddress, Ed25519PublicKey, Ed25519SecretKey, Ed25519Signature,
    SHA256Hash, Signature, StorageKeyContractData, StorageLongHash,
};
use thruput_xdr::validator::ValidatorID;

/// Fills an `N`-byte array with distinct ends (0xab, 0xcd, ..., 0xff) so a
/// reversed or truncated rendering fails rather than reading as a
/// palindrome.
fn sample<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    bytes[0] = 0xab;
    bytes[1] = 0xcd;
    bytes[N - 1] = 0xff;
    bytes
}

/// Lowercase hex of `bytes`, the form clients receive.
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Asserts `value` serializes to exactly `"<hex of bytes>"` and parses back
/// from that string, from its uppercase form, and only at the right length.
fn assert_hex_string<T>(value: &T, bytes: &[u8])
where
    T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let expected = hex(bytes);
    let json = serde_json::to_string(value).expect("serialize");
    assert_eq!(json, format!("\"{expected}\""));
    assert_eq!(json.len(), 2 + 2 * bytes.len(), "two digits per byte");

    let decoded: T =
        serde_json::from_str(&json).expect("deserialize lowercase");
    assert_eq!(&decoded, value, "json roundtrip mismatch");

    let upper = format!("\"{}\"", expected.to_uppercase());
    let decoded: T =
        serde_json::from_str(&upper).expect("deserialize uppercase");
    assert_eq!(&decoded, value, "uppercase input mismatch");

    let prefixed = format!("\"0x{expected}\"");
    assert!(
        serde_json::from_str::<T>(&prefixed).is_err(),
        "0x prefix unexpectedly accepted"
    );
    let short = format!("\"{}\"", &expected[..expected.len() - 2]);
    assert!(
        serde_json::from_str::<T>(&short).is_err(),
        "one byte short unexpectedly accepted"
    );
    let long = format!("\"{expected}00\"");
    assert!(
        serde_json::from_str::<T>(&long).is_err(),
        "one byte long unexpectedly accepted"
    );
    let odd = format!("\"{}\"", &expected[..expected.len() - 1]);
    assert!(
        serde_json::from_str::<T>(&odd).is_err(),
        "odd digit count unexpectedly accepted"
    );
    let mut invalid = expected.clone();
    invalid.replace_range(0..2, "zz");
    assert!(
        serde_json::from_str::<T>(&format!("\"{invalid}\"")).is_err(),
        "non-hex digits unexpectedly accepted"
    );
    // Right byte length, but the three-byte '€' straddles the first digit
    // pair: a decoder that slices the str rather than its bytes would panic.
    let mut multibyte = expected.clone();
    multibyte.replace_range(0..3, "€");
    assert_eq!(multibyte.len(), expected.len(), "same byte length");
    assert!(
        serde_json::from_str::<T>(&format!("\"{multibyte}\"")).is_err(),
        "multi-byte UTF-8 unexpectedly accepted"
    );
    let array = serde_json::to_string(bytes).expect("serialize array");
    assert!(
        serde_json::from_str::<T>(&array).is_err(),
        "JSON byte array unexpectedly accepted"
    );
}

#[test]
fn ed25519_signature_is_128_hex_digits() {
    let bytes = sample::<64>();
    assert_hex_string(&Ed25519Signature::new(bytes), &bytes);
}

#[test]
fn ed25519_public_key_is_64_hex_digits() {
    let bytes = sample::<32>();
    assert_hex_string(&Ed25519PublicKey::new(bytes), &bytes);
}

#[test]
fn ed25519_secret_key_is_128_hex_digits() {
    let bytes = sample::<64>();
    assert_hex_string(&Ed25519SecretKey::new(bytes), &bytes);
}

#[test]
fn sha256_hash_is_64_hex_digits() {
    let bytes = sample::<32>();
    assert_hex_string(&SHA256Hash::new(bytes), &bytes);
}

#[test]
fn contract_address_is_64_hex_digits() {
    let bytes = sample::<32>();
    assert_hex_string(&ContractAddress::new(bytes), &bytes);
}

#[test]
fn storage_key_contract_data_is_64_hex_digits() {
    let bytes = sample::<32>();
    assert_hex_string(&StorageKeyContractData::new(bytes), &bytes);
}

#[test]
fn storage_long_hash_is_4096_hex_digits() {
    let bytes = sample::<2048>();
    assert_hex_string(&StorageLongHash::new(bytes), &bytes);
}

#[test]
fn topic_is_64_hex_digits() {
    let bytes = sample::<32>();
    assert_hex_string(&Topic::new(bytes), &bytes);
}

#[test]
fn contract_entrypoint_id_is_16_hex_digits() {
    let bytes = sample::<8>();
    assert_hex_string(&ContractEntrypointID::new(bytes), &bytes);
}

#[test]
fn validator_id_is_64_hex_digits() {
    let bytes = sample::<32>();
    assert_hex_string(&ValidatorID::new(bytes), &bytes);
}

#[test]
fn zero_and_max_render_full_width() {
    assert_eq!(
        serde_json::to_string(&ContractAddress::zero()).unwrap(),
        format!("\"{}\"", "00".repeat(32))
    );
    assert_eq!(
        serde_json::to_string(&SHA256Hash::new([0xff; 32])).unwrap(),
        format!("\"{}\"", "ff".repeat(32))
    );
    // A naive {:x} per byte drops the leading zero and would render short.
    assert_eq!(
        serde_json::to_string(&Ed25519Signature::new([0x01; 64])).unwrap(),
        format!("\"{}\"", "01".repeat(64))
    );
}

#[test]
fn hex_fields_nest_as_strings_inside_containing_types() {
    let sig = sample::<64>();
    let decorated = DecoratedSignature {
        public_key_id: 3,
        signature: Signature::Ed25519 {
            ed25519: Ed25519Signature::new(sig),
        },
    };

    let json = serde_json::to_value(&decorated).expect("serialize");
    assert_eq!(json["public_key_id"], serde_json::json!(3));
    assert_eq!(
        json["signature"]["Ed25519"]["ed25519"],
        serde_json::Value::String(hex(&sig)),
        "signature is a hex string, not a byte array: {json}"
    );

    let decoded: DecoratedSignature =
        serde_json::from_value(json).expect("deserialize");
    assert_eq!(decoded, decorated);
}
