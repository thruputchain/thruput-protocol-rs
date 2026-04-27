// Tests for span arguments (input and output buffers)

use groundhog_xdr::SyscallNo;
use groundhog_xdr::syscalls::*;
use groundhog_xdr::types::*;
use groundhog_xdr_syscall_tests::{MockSyscallHandler, set_mock_handler};
use std::sync::Arc;

#[test]
fn test_sha256_span_input_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // SHA256 has:
    // - buffer: span_argument (input)
    // - hash: SHA256Hash (output, fixed-size)

    let input_buffer = b"test input data";
    let mut hash = SHA256Hash::from([0u8; 32]);

    let result = raw_SHA256(input_buffer, &mut hash);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // SHA256 syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kSHA256.0);

    // arg1: input buffer pointer
    assert_eq!(invocations[0].args[0], input_buffer.as_ptr() as u64);
    // arg2: input buffer length
    assert_eq!(invocations[0].args[1], input_buffer.len() as u64);
    // arg3: output hash pointer
    assert_eq!(invocations[0].args[2], hash.as_mut_ptr() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[3..], [0; 7]);
}

#[test]
fn test_get_calldata_span_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // GetCalldata has:
    // - buffer: span_argument (output)
    // - written_len: RuntimeMemoryLength (u32, primitive output)

    let mut buffer = [0u8; 1024];
    let mut written_len: RuntimeMemoryLength = 0;
    let mut auth_token_list = [0u8; 1024];
    let mut auth_token_written_len: RuntimeMemoryLength = 0;

    let result = raw_GetCalldata(
        &mut buffer,
        &mut written_len,
        &mut auth_token_list,
        &mut auth_token_written_len,
    );

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // GetCalldata syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kGetCalldata.0);

    // arg1: output buffer pointer
    assert_eq!(invocations[0].args[0], buffer.as_mut_ptr() as u64);
    // arg2: output buffer length (capacity)
    assert_eq!(invocations[0].args[1], buffer.len() as u64);
    // arg3: written_len pointer
    assert_eq!(invocations[0].args[2], &written_len as *const _ as u64);
    // arg4: auth_token_list pointer
    assert_eq!(invocations[0].args[3], auth_token_list.as_mut_ptr() as u64);
    // arg5: auth_token_list length
    assert_eq!(invocations[0].args[4], auth_token_list.len() as u64);
    // arg6: auth_token_written_len pointer
    assert_eq!(
        invocations[0].args[5],
        &auth_token_written_len as *const _ as u64
    );
    // Remaining args should be 0
    assert_eq!(invocations[0].args[6..], [0; 4]);
}

#[test]
fn test_get_returned_buffer_span_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // GetReturnedBuffer has:
    // - buffer: span_argument (output)
    // - written_len: RuntimeMemoryLength (u32, primitive output)

    let mut buffer = [0u8; 512];
    let mut written_len: RuntimeMemoryLength = 0;

    let mut restored_auth_token_ids = [0u8; 512];
    let mut restored_auth_token_ids_written_len: RuntimeMemoryLength = 0;

    let result = raw_GetReturnedBuffer(
        &mut buffer,
        &mut written_len,
        &mut restored_auth_token_ids,
        &mut restored_auth_token_ids_written_len,
    );

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // GetReturnedBuffer syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kGetReturnedBuffer.0);

    // arg1: output buffer pointer
    assert_eq!(invocations[0].args[0], buffer.as_mut_ptr() as u64);
    // arg2: output buffer length (capacity)
    assert_eq!(invocations[0].args[1], buffer.len() as u64);
    // arg3: written_len pointer
    assert_eq!(invocations[0].args[2], &written_len as *const _ as u64);

    // arg4: restored_auth_token_ids pointer
    assert_eq!(
        invocations[0].args[3],
        restored_auth_token_ids.as_mut_ptr() as u64
    );
    // arg5: restored_auth_token_ids length
    assert_eq!(invocations[0].args[4], restored_auth_token_ids.len() as u64);
    // arg6: restored_auth_token_ids_written_len pointer
    assert_eq!(
        invocations[0].args[5],
        &restored_auth_token_ids_written_len as *const _ as u64
    );
    // Remaining args should be 0
    assert_eq!(invocations[0].args[6..], [0; 4]);
}

#[test]
fn test_bytestring_get_span_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // BytestringGet has:
    // - key: StorageKeyContractData (input, fixed-size)
    // - bytestring: span_argument (output)
    // - written_len: RuntimeMemoryLength (u32, primitive output)

    let key = StorageKeyContractData::from([0xAA; 32]);
    let mut bytestring = [0u8; 256];
    let mut written_len: RuntimeMemoryLength = 0;

    let result = raw_BytestringGet(&key, &mut bytestring, &mut written_len);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // BytestringGet syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kBytestringGet.0);

    // arg1: key pointer
    assert_eq!(invocations[0].args[0], key.as_ptr() as u64);
    // arg2: output bytestring pointer
    assert_eq!(invocations[0].args[1], bytestring.as_mut_ptr() as u64);
    // arg3: output bytestring length (capacity)
    assert_eq!(invocations[0].args[2], bytestring.len() as u64);
    // arg4: written_len pointer
    assert_eq!(invocations[0].args[3], &written_len as *const _ as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[4..], [0; 6]);
}
