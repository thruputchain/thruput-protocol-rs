//! `Debug` for the 32-byte opaque newtypes renders hex rather than the
//! `[1, 2, 3, ...]` a derive would produce.

use thruput_xdr::types::{ContractAddress, SHA256Hash};

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
fn contract_address_debug_is_prefixed_lowercase_hex() {
    assert_eq!(
        format!("{:?}", ContractAddress::new(sample())),
        "0xabcd0000000000000000000000000000000000000000000000000000000000ff"
    );
}

#[test]
fn sha256_hash_debug_is_prefixed_lowercase_hex() {
    assert_eq!(
        format!("{:?}", SHA256Hash::new(sample())),
        "0xabcd0000000000000000000000000000000000000000000000000000000000ff"
    );
}

#[test]
fn every_byte_is_rendered_as_two_digits() {
    // A naive {:x} per byte drops the leading zero, so an address with small
    // bytes would render short. Pin the width at 2 chars per byte.
    let rendered = format!("{:?}", ContractAddress::new([0x01; 32]));
    assert_eq!(rendered, format!("0x{}", "01".repeat(32)));
    assert_eq!(rendered.len(), 2 + 64);
}

#[test]
fn zero_and_max_render_full_width() {
    assert_eq!(
        format!("{:?}", ContractAddress::zero()),
        format!("0x{}", "00".repeat(32))
    );
    assert_eq!(
        format!("{:?}", SHA256Hash::new([0xff; 32])),
        format!("0x{}", "ff".repeat(32))
    );
}
