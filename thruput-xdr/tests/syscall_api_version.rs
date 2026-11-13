use thruput_xdr::{SYSCALL_API_MAJOR, SYSCALL_API_MINOR, SyscallNo};

#[test]
fn version_matches_the_spec() {
    assert_eq!((SYSCALL_API_MAJOR, SYSCALL_API_MINOR), (1, 1));
}

#[test]
fn since_minor_per_syscall() {
    assert_eq!(SyscallNo::kSHA256.since_minor(), Some(0));
    assert_eq!(SyscallNo::kRistretto255Msm.since_minor(), Some(1));
    assert_eq!(SyscallNo(u32::MAX).since_minor(), None);
}

#[test]
fn no_syscall_is_later_than_the_api_minor() {
    let defined = (0..)
        .map_while(|number| SyscallNo(number).since_minor())
        .collect::<Vec<_>>();
    assert!(!defined.is_empty());
    assert!(defined.iter().all(|since| *since <= SYSCALL_API_MINOR));
}

#[test]
fn experimental_syscalls() {
    assert!(SyscallNo::kRistretto255Msm.is_experimental());
    assert!(!SyscallNo::kSHA256.is_experimental());
    assert!(!SyscallNo(u32::MAX).is_experimental());
}

// Usable where a guest embeds the version it was built against.
const _: u32 = match SyscallNo::kReturn.since_minor() {
    Some(since) => since,
    None => panic!("Return is defined"),
};
