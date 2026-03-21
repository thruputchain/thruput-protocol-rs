// Compile-time tests to ensure generated types have required trait implementations.

use groundhog_xdr::types::uint128_xdr;

#[test]
fn test_uint128_xdr_has_partial_ord() {
    fn assert_partial_ord<T: PartialOrd>(_: &T) {}

    let val = uint128_xdr {
        highbits: 1,
        lowbits: 2,
    };
    assert_partial_ord(&val);

    // Also test that comparison operations work
    let val2 = uint128_xdr {
        highbits: 2,
        lowbits: 1,
    };
    let _ = val < val2; // This will fail to compile if PartialOrd is not implemented
}

#[cfg(feature = "std")]
mod format_tests {

    #[test]
    fn test_display() {
        let x: groundhog_xdr::storage_item::StorageItemType = 0.into();
        std::println!("value: {}", x);
    }

    #[test]
    fn to_string_test() {
        let x: groundhog_xdr::transaction::InvocationFrameStatusCode =
            0.into();
        assert_eq!(x.to_string(), "InvocationFrameStatusCode::Return");

        let y: groundhog_xdr::transaction::InvocationFrameStatusCode =
            100.into();
        assert_eq!(y.to_string(), "InvocationFrameStatusCode(100)");
    }
}
