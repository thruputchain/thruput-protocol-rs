//! Debug output of secret keys, alone or inside derived types, never shows
//! the key bytes.

use thruput_xdr::types::{Ed25519SecretKey, MlDsa65SecretKey, SecretKey};

#[test]
fn ed25519_secret_key_debug_is_redacted() {
    assert_eq!(
        format!("{:?}", Ed25519SecretKey::new([0xab; 64])),
        "Ed25519SecretKey(<redacted>)"
    );
}

#[test]
fn mldsa65_secret_key_debug_is_redacted() {
    assert_eq!(
        format!("{:?}", MlDsa65SecretKey::new([0xab; 4032])),
        "MlDsa65SecretKey(<redacted>)"
    );
}

#[test]
fn derived_debug_of_secret_key_is_redacted() {
    let ed25519 = SecretKey::Ed25519 {
        ed25519: Ed25519SecretKey::new([0xab; 64]),
    };
    assert_eq!(
        format!("{ed25519:?}"),
        "Ed25519 { ed25519: Ed25519SecretKey(<redacted>) }"
    );
    let mldsa65 = SecretKey::MlDsa65 {
        mldsa65: Box::new(MlDsa65SecretKey::new([0xab; 4032])),
    };
    assert_eq!(
        format!("{mldsa65:?}"),
        "MlDsa65 { mldsa65: MlDsa65SecretKey(<redacted>) }"
    );
    for key in [&ed25519, &mldsa65] {
        assert!(!format!("{key:#?}").contains("171"));
    }
}
