// Tests for optional parameters

use groundhog_xdr::SyscallNo;
use groundhog_xdr::syscalls::*;
use groundhog_xdr::types::*;
use groundhog_xdr_syscall_tests::{MockSyscallHandler, set_mock_handler};
use std::sync::Arc;

#[test]
fn test_get_deploy_address_optional_input() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // GetDeployAddress has:
    // - caller_address: ContractAddress (optional input)
    // - nonce: SHA256Hash (input, fixed-size)
    // - contract_hash: SHA256Hash (input, fixed-size)
    // - to_be_deployed: ContractAddress (output, fixed-size)

    let caller_addr = ContractAddress::from([0x11; 32]);
    let caller_address = Some(&caller_addr);
    let nonce = SHA256Hash::from([0x22; 32]);
    let contract_hash = SHA256Hash::from([0x33; 32]);
    let mut to_be_deployed = ContractAddress::from([0u8; 32]);

    let result = raw_GetDeployAddress(
        caller_address,
        &nonce,
        &contract_hash,
        &mut to_be_deployed,
    );

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // first invocation should be syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kGetDeployAddress.0);

    // arg1: caller_address pointer (Some value)
    assert_eq!(invocations[0].args[0], caller_addr.as_ptr() as u64);
    // arg2: nonce pointer
    assert_eq!(invocations[0].args[1], nonce.as_ptr() as u64);
    // arg3: contract_hash pointer
    assert_eq!(invocations[0].args[2], contract_hash.as_ptr() as u64);
    // arg4: to_be_deployed pointer
    assert_eq!(invocations[0].args[3], to_be_deployed.as_mut_ptr() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[4..], [0; 3]);
}

#[test]
fn test_get_deploy_address_optional_none() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // Test with None for optional parameter
    let caller_address: Option<&ContractAddress> = None;
    let nonce = SHA256Hash::from([0x22; 32]);
    let contract_hash = SHA256Hash::from([0x33; 32]);
    let mut to_be_deployed = ContractAddress::from([0u8; 32]);

    let result = raw_GetDeployAddress(
        caller_address,
        &nonce,
        &contract_hash,
        &mut to_be_deployed,
    );

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // first invocation should be syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kGetDeployAddress.0);

    // arg1: caller_address should be u64::MAX when None
    assert_eq!(invocations[0].args[0], u64::MAX);
    // arg2: nonce pointer
    assert_eq!(invocations[0].args[1], nonce.as_ptr() as u64);
    // arg3: contract_hash pointer
    assert_eq!(invocations[0].args[2], contract_hash.as_ptr() as u64);
    // arg4: to_be_deployed pointer
    assert_eq!(invocations[0].args[3], to_be_deployed.as_mut_ptr() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[4..], [0; 3]);
}

#[test]
fn test_log_event_optional_topics() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // LogEvent has:
    // - topic0: Topic ([u8; 32], optional)
    // - topic1: Topic ([u8; 32], optional)
    // - topic2: Topic ([u8; 32], optional)
    // - data: span_argument

    let topic0_arr = [0xAA; 32];
    let topic1_arr = [0xBB; 32];
    let topic0 = Some(&topic0_arr);
    let topic1 = Some(&topic1_arr);
    let topic2: Option<&[u8; 32]> = None;
    let data = b"event data";

    let result = raw_LogEvent(topic0, topic1, topic2, data);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // LogEvent syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kLogEvent.0);

    // arg1: topic0 pointer (Some value)
    assert_eq!(invocations[0].args[0], topic0_arr.as_ptr() as u64);
    // arg2: topic1 pointer (Some value)
    assert_eq!(invocations[0].args[1], topic1_arr.as_ptr() as u64);
    // arg3: topic2 should be u64::MAX when None
    assert_eq!(invocations[0].args[2], u64::MAX);
    // arg4: data pointer
    assert_eq!(invocations[0].args[3], data.as_ptr() as u64);
    // arg5: data length
    assert_eq!(invocations[0].args[4], data.len() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[5..], [0; 2]);
}

#[test]
fn test_change_public_key_registration_optional_params() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // ChangePublicKeyRegistration has:
    // - key_index: uint32_t (primitive)
    // - key_scheme: int32_t (optional primitive)
    // - key_bytes: span_argument
    // - key_weight: uint32_t (optional primitive)

    let key_index: u32 = 5;
    let key_scheme_val: i32 = 1;
    let key_scheme: Option<&i32> = Some(&key_scheme_val);
    let key_bytes = b"public key bytes";
    let key_weight_val: u32 = 100;
    let key_weight: Option<&u32> = Some(&key_weight_val);

    let result = raw_ChangePublicKeyRegistration(
        key_index, key_scheme, key_bytes, key_weight,
    );

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // ChangePublicKeyRegistration syscall number
    assert_eq!(
        invocations[0].syscall_no,
        SyscallNo::kChangePublicKeyRegistration.0
    );

    // arg1: key_index (u32 primitive, passed by value)
    assert_eq!(invocations[0].args[0], key_index as u64);
    // arg2: key_scheme (Some value, passed as pointer to the value)
    assert_eq!(invocations[0].args[1], &key_scheme_val as *const i32 as u64);
    // arg3: key_bytes pointer
    assert_eq!(invocations[0].args[2], key_bytes.as_ptr() as u64);
    // arg4: key_bytes length
    assert_eq!(invocations[0].args[3], key_bytes.len() as u64);
    // arg5: key_weight (Some value, passed as pointer to the value)
    assert_eq!(invocations[0].args[4], &key_weight_val as *const u32 as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[5..], [0; 2]);
}
