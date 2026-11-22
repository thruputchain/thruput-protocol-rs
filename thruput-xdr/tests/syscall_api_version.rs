use thruput_xdr::SyscallNo;

#[test]
fn an_undefined_number_has_no_version() {
    assert_eq!(SyscallNo(u32::MAX).since_minor(), None);
    assert!(!SyscallNo(u32::MAX).is_experimental());
}

// Usable where a guest embeds the version it was built against.
const _: u32 = match SyscallNo::kReturn.since_minor() {
    Some(since) => since,
    None => panic!("Return is defined"),
};
