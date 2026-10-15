use super::*;
use std::{format, string::ToString};

#[test]
fn known_vectors_preserve_bytes_and_leading_zeros() {
    let sequence = core::array::from_fn::<_, 64, _>(|i| i as u8);
    let vectors: &[(&[u8], &str)] = &[
        (&[], ""),
        (&[255], "5Q"),
        (&[0, 0, 1, 255], "119p"),
        (b"Hello World", "JxF12TrwUP45BMd"),
        (&[0; 32], "11111111111111111111111111111111"),
        (&[255; 32], "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"),
        (
            &sequence,
            "1GMkH3brNXiNNs1tiFZHu4yZSRrzJwxi5wB9bHFtMinfCXNnR1adh8Vo8NTheK4evneedH4qmvjeqcBBNAefgS",
        ),
    ];
    for &(bytes, text) in vectors {
        let mut encoded = [0; 100];
        let len = encode_into(bytes, &mut encoded).unwrap();
        assert_eq!(&encoded[..len], text.as_bytes());
        let mut decoded = [0; 64];
        let len = decode_into(text, &mut decoded[..bytes.len()]).unwrap();
        assert_eq!(&decoded[..len], bytes);
    }
}

#[test]
fn formatting_handles_empty_single_byte_and_signature_sizes() {
    assert_eq!(base58_encode(&[]).to_string(), "");
    assert_eq!(base58_encode(&[255]).to_string(), "5Q");
    let bytes = core::array::from_fn::<_, 64, _>(|i| i as u8);
    let text = "1GMkH3brNXiNNs1tiFZHu4yZSRrzJwxi5wB9bHFtMinfCXNnR1adh8Vo8NTheK4evneedH4qmvjeqcBBNAefgS";
    assert_eq!(base58_encode(&bytes).to_string(), text);
    assert_eq!(format!("{:?}", base58_encode(&bytes)), text);
}

#[test]
fn size_limits_cover_text_precheck_and_decoded_overflow() {
    assert_eq!(max_encoded_len(0), 0);
    assert_eq!(max_encoded_len(32), 45);
    assert_eq!(max_encoded_len(usize::MAX), usize::MAX);
    assert_eq!(decode_fixed::<0>(""), Ok([]));
    assert_eq!(decode_fixed::<0>("1"), Err(Error::BufferTooSmall));
    for text in ["1".repeat(33), "z".repeat(44), "z".repeat(46)] {
        assert_eq!(decode_fixed::<32>(&text), Err(Error::BufferTooSmall));
    }
    assert_eq!(
        decode_fixed::<32>("2"),
        Err(Error::InvalidLength {
            expected: 32,
            actual: 1
        })
    );
    assert_eq!(encode_into(&[255], &mut [0; 1]), Err(Error::BufferTooSmall));
}

#[test]
fn invalid_input_errors_do_not_echo_text() {
    for text in [
        "0",
        "O",
        "I",
        "l",
        " 1",
        "1 ",
        "1\n",
        "1\t",
        "=",
        "é",
        "１",
        "0x01",
        "secret:do-not-echo",
    ] {
        let error = decode_fixed::<32>(text).unwrap_err();
        assert_eq!(error, Error::InvalidEncoding);
        assert_eq!(error.to_string(), "invalid base58 encoding");
        assert_eq!(format!("{error:?}"), "InvalidEncoding");
    }
}

#[test]
fn alphabet_is_case_sensitive_and_never_guesses_hex() {
    assert_eq!(decode_fixed::<2>("ff"), Ok([8, 194]));
    assert_ne!(decode_fixed::<2>("ab"), decode_fixed::<2>("AB"));
}

#[test]
fn formatting_propagates_writer_errors() {
    struct Reject;
    impl fmt::Write for Reject {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }
    assert!(
        fmt::write(&mut Reject, format_args!("{}", base58_encode(&[1; 32])))
            .is_err()
    );
}
