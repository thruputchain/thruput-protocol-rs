// Tests for the convenience wrappers, which read their outputs only after a
// successful syscall. Each success test uses a responder that writes the
// outputs, as the runtime does.

use std::sync::Arc;
use thruput_xdr::SyscallNo;
use thruput_xdr::syscall_error::SyscallErrorCode;
use thruput_xdr::syscalls::*;
use thruput_xdr::syserr::SysErr;
use thruput_xdr::types::*;
use thruput_xdr_syscall_tests::{
    MockSyscallHandler, set_mock_handler, write_output,
};

#[test]
fn test_wrapper_returns_written_outputs() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_responder(|syscall_no, args| {
        assert_eq!(syscall_no, SyscallNo::kGetCalldataLen.0);
        // SAFETY: GetCalldataLen's two outputs are a RuntimeMemoryLength and
        // a u32, passed by pointer in its first two arguments.
        unsafe {
            write_output::<RuntimeMemoryLength>(args[0], 42);
            write_output::<u32>(args[1], 3);
        }
        SysErr(0)
    });

    assert_eq!(GetCalldataLen().unwrap(), (42, 3));
    assert_eq!(handler.get_invocations().len(), 1);
}

#[test]
fn test_wrapper_returns_written_fixed_size_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    let sender = ContractAddress::from([7u8; 32]);
    handler.set_responder(move |_, args| {
        // SAFETY: GetMsgSender's output is a ContractAddress passed by
        // pointer in its first argument.
        unsafe { write_output(args[0], sender) };
        SysErr(0)
    });

    assert_eq!(GetMsgSender().unwrap(), sender);
}

#[test]
fn test_wrapper_returns_configured_error() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    // No responder: the syscall fails without writing its outputs, so the
    // wrapper must return the error rather than read them.
    handler.set_return_value(SysErr::from(
        SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput,
    ));

    let err = GetCalldataLen().unwrap_err();
    assert!(err.is_invalid_input());
}

#[test]
fn test_wrapper_returns_responder_error() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_responder(|_, _| {
        SysErr::from(
            SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable,
        )
    });

    let err = GetMsgSender().unwrap_err();
    assert!(
        err.is_code(
            SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable
        )
    );
}

#[test]
#[should_panic(expected = "no mock handler")]
fn test_syscall_without_handler_panics() {
    // Each test runs on its own thread, and this one installs no handler.
    let _ = GetCalldataLen();
}
