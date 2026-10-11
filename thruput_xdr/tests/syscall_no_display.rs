use thruput_xdr::SyscallNo;

#[test]
fn show_syscall() {
    let s = SyscallNo::kReturn;
    let displayed = s.to_string();

    assert_eq!(displayed.as_str(), "Return");
}
