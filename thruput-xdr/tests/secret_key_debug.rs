//! Debug output of secret keys, alone or inside derived types, never shows
//! the key bytes.

use thruput_xdr::types::{Ed25519SecretKey, SecretKey};

#[test]
fn ed25519_secret_key_debug_is_redacted() {
    assert_eq!(
        format!("{:?}", Ed25519SecretKey::new([0xab; 64])),
        "Ed25519SecretKey(<redacted>)"
    );
}

#[test]
fn derived_debug_of_secret_key_is_redacted() {
    let key = SecretKey::Ed25519 {
        ed25519: Ed25519SecretKey::new([0xab; 64]),
    };
    assert_eq!(
        format!("{key:?}"),
        "Ed25519 { ed25519: Ed25519SecretKey(<redacted>) }"
    );
    assert!(!format!("{key:#?}").contains("171"));
}
