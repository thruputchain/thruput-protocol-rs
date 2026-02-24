// Tests for primitive parameter types (u32, u64, bool, i32, etc.)

use groundhog_xdr::syscalls::*;
use groundhog_xdr::SyscallNo;
use groundhog_xdr::transaction::*;
use groundhog_xdr::types::*;
use groundhog_xdr_syscall_tests::{MockSyscallHandler, set_mock_handler};
use std::sync::Arc;

#[test]
fn test_get_calldata_len_primitive_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0)); // Success

    // GetCalldataLen returns a u32 (RuntimeMemoryLength)
    // It has no inputs, one output: len: RuntimeMemoryLength
    let mut len: RuntimeMemoryLength = 0;

    // GetCalldataLen returns a u32 (RuntimeMemoryLength)
    // It has no inputs, one output: len: RuntimeMemoryLength
    let result = raw_GetCalldataLen(&mut len);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // GetCalldataLen syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kGetCalldataLen as u64);
    // Output parameter should be passed as a pointer
    // The first arg should be the pointer to len
    assert_eq!(invocations[0].args[0], &len as *const _ as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[1..], [0; 6]);
}

#[test]
fn test_console_log_span_input() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    let buffer = b"Hello, world!";

    let result = raw_ConsoleLog(buffer);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // ConsoleLog syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kConsoleLog as u64);
    // Span argument: first arg is pointer, second is length
    assert_eq!(invocations[0].args[0], buffer.as_ptr() as u64);
    assert_eq!(invocations[0].args[1], buffer.len() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[2..], [0; 5]);
}

#[test]
fn test_invoke_primitive_inputs() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // Invoke has:
    // - address: ContractAddress (fixed-size, 32 bytes)
    // - method: MethodID (u64, primitive)
    // - gas_limit: GasAmount (u64, primitive)
    // - calldata: span_argument
    // - flags: InvokeFlagSet (i32, primitive)
    // Output: return_buffer_len: RuntimeMemoryLength (u32, primitive)

    let address = ContractAddress::from([0x42; 32]);
    let method: u64 = 0x1234567890ABCDEF;
    let gas_limit: GasAmount = 1000000;
    let calldata = b"test calldata";
    let flags: InvokeFlagSet = 0x01;
    let mut return_buffer_len: RuntimeMemoryLength = 0;

    let result = raw_Invoke(
        &address,
        method,
        gas_limit,
        calldata,
        flags,
        &mut return_buffer_len,
    );

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // Invoke syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kInvoke as u64);

    // Check arguments:
    // arg1: address pointer
    assert_eq!(invocations[0].args[0], address.as_ptr() as u64);
    // arg2: method (u64 primitive, passed by value)
    assert_eq!(invocations[0].args[1], method);
    // arg3: gas_limit (u64 primitive, passed by value)
    assert_eq!(invocations[0].args[2], gas_limit);
    // arg4: calldata pointer
    assert_eq!(invocations[0].args[3], calldata.as_ptr() as u64);
    // arg5: calldata length
    assert_eq!(invocations[0].args[4], calldata.len() as u64);
    // arg6: flags (i32 primitive, passed by value as u64)
    assert_eq!(invocations[0].args[5], flags as u64);
    // arg7: return_buffer_len pointer
    assert_eq!(
        invocations[0].args[6],
        &return_buffer_len as *const _ as u64
    );
}

#[test]
fn test_sha256_update_bool_primitive() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // SHA256Update has:
    // - index: u32 (primitive)
    // - buffer: span_argument
    // - reset: bool (primitive)
    //
    // This test specifically verifies that the bool is passed by value (as bool)
    // and not by reference (as &bool). If it were passed by reference, we'd see
    // a pointer address instead of 0 or 1.

    let index: u32 = 5;
    let buffer = b"test data";
    let reset = true;

    let result = raw_SHA256Update(index, buffer, reset);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // SHA256Update syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kSHA256Update as u64);

    // arg1: index (u32 primitive, passed by value)
    assert_eq!(invocations[0].args[0], index as u64);
    // arg2: buffer pointer
    assert_eq!(invocations[0].args[1], buffer.as_ptr() as u64);
    // arg3: buffer length
    assert_eq!(invocations[0].args[2], buffer.len() as u64);
    // arg4: reset (bool primitive, passed by value as u64)
    // If it were passed as &bool, this would be a pointer address (large number)
    // Since it's passed by value, true becomes 1 and false becomes 0
    assert_eq!(invocations[0].args[3], reset as u64);
    assert_eq!(invocations[0].args[3], 1u64); // true should be 1
    // Remaining args should be 0
    assert_eq!(invocations[0].args[4..], [0; 3]);
}
