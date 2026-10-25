//! Debug output of an ML-DSA-65 public key, alone or inside PublicKey, is a
//! short prefix rather than all 1952 bytes.

use thruput_xdr::types::{MlDsa65PublicKey, PublicKey};

fn sample() -> MlDsa65PublicKey {
    let mut bytes = [0xffu8; 1952];
    for (i, b) in bytes.iter_mut().take(8).enumerate() {
        *b = i as u8 * 0x11;
    }
    MlDsa65PublicKey::new(bytes)
}

#[test]
fn mldsa65_public_key_debug_is_a_prefix() {
    assert_eq!(
        format!("{:?}", sample()),
        "MlDsa65PublicKey(0011223344556677..)"
    );
}

#[test]
fn derived_debug_of_public_key_uses_the_prefix() {
    let key = PublicKey::MlDsa65 {
        mldsa65: Box::new(sample()),
    };
    assert_eq!(
        format!("{key:?}"),
        "MlDsa65 { mldsa65: MlDsa65PublicKey(0011223344556677..) }"
    );
}
