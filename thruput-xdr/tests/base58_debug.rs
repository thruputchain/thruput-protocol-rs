//! Typed identifier diagnostics use base58 independently of raw Serde/XDR.

use thruput_xdr::types::{ContractAddress, NetworkHash, SHA256Hash};

/// 0xab, 0xcd, 28 zero bytes, then 0xff -- distinct ends, so a reversed or
/// truncated rendering fails rather than reading as a palindrome.
fn sample() -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes[0] = 0xab;
    bytes[1] = 0xcd;
    bytes[31] = 0xff;
    bytes
}

#[test]
fn contract_address_debug_is_unprefixed_base58() {
    assert_eq!(
        format!("{:?}", ContractAddress::new(sample())),
        "CZe287fajMxBpe8q4t5XBZZPSZ3gwQR1ETdKx93Qd3JE"
    );
}

#[test]
fn sha256_hash_debug_is_unprefixed_base58() {
    assert_eq!(
        format!("{:?}", SHA256Hash::new(sample())),
        "CZe287fajMxBpe8q4t5XBZZPSZ3gwQR1ETdKx93Qd3JE"
    );
}

#[test]
fn wrappers_inherit_identifier_formatting() {
    assert_eq!(
        format!("{:?}", NetworkHash::new(SHA256Hash::new(sample()))),
        "NetworkHash { inner: CZe287fajMxBpe8q4t5XBZZPSZ3gwQR1ETdKx93Qd3JE }"
    );
}

#[test]
fn zero_and_max_preserve_all_bytes() {
    assert_eq!(
        format!("{:?}", ContractAddress::zero()),
        "11111111111111111111111111111111"
    );
    assert_eq!(
        format!("{:?}", SHA256Hash::new([0xff; 32])),
        "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
    );
}

#[test]
#[cfg(feature = "xdrs")]
fn identifier_diagnostics_do_not_change_xdr_bytes() {
    let address = ContractAddress::new(sample());
    let hash = SHA256Hash::new(sample());
    assert_eq!(xdrs::txs::encode(&address).unwrap(), sample());
    assert_eq!(xdrs::txs::encode(&hash).unwrap(), sample());
}
