//! Properties of the generated syscall API accessors that hold whatever the
//! spec says. Nothing here names a version or a syscall's `since_minor`, so a
//! protocol update needs no edit here.

use thruput_xdr::{SYSCALL_API_MINOR, SyscallNo};

#[test]
fn no_syscall_is_later_than_the_api_minor() {
    let defined = (0..)
        .map_while(|number| SyscallNo(number).since_minor())
        .collect::<Vec<_>>();
    assert!(!defined.is_empty());
    assert!(defined.iter().all(|since| *since <= SYSCALL_API_MINOR));
}

#[test]
fn an_undefined_number_has_no_version_and_is_not_experimental() {
    assert_eq!(SyscallNo(u32::MAX).since_minor(), None);
    assert!(!SyscallNo(u32::MAX).is_experimental());
}

// Usable where a guest embeds the version it was built against.
const _: u32 = match SyscallNo::kReturn.since_minor() {
    Some(since) => since,
    None => panic!("Return is defined"),
};
