// Tests for fixed-size types (SHA256Hash, ContractAddress, StorageKeyContractData, etc.)

use std::sync::Arc;
use thruput_xdr::SyscallNo;
use thruput_xdr::syscalls::*;
use thruput_xdr::syserr::SysErr;
use thruput_xdr::types::*;
use thruput_xdr_syscall_tests::{MockSyscallHandler, set_mock_handler};

#[test]
fn test_get_msg_sender_fixed_size_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // GetMsgSender has:
    // - sender: ContractAddress (output, fixed-size)

    let mut sender = ContractAddress::from([0u8; 32]);

    let result = raw_GetMsgSender(&mut sender);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // GetMsgSender syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kGetMsgSender.0);

    // arg1: sender pointer
    assert_eq!(invocations[0].args[0], sender.as_mut_ptr() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[1..], [0; 6]);
}

#[test]
fn test_get_source_transaction_hash_fixed_size_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // GetSourceTransactionHash has:
    // - source_tx_hash: SHA256Hash (output, fixed-size)

    let mut source_tx_hash = SHA256Hash::from([0u8; 32]);

    let result = raw_GetSourceTransactionHash(&mut source_tx_hash);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // GetSourceTransactionHash syscall number
    assert_eq!(
        invocations[0].syscall_no,
        SyscallNo::kGetSourceTransactionHash.0
    );

    // arg1: source_tx_hash pointer
    assert_eq!(invocations[0].args[0], source_tx_hash.as_mut_ptr() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[1..], [0; 6]);
}

#[test]
fn test_deploy_contract_fixed_size_input_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // DeployContract has:
    // - nonce: SHA256Hash (input, fixed-size)
    // - contract_hash: SHA256Hash (input, fixed-size)
    // - deployed_address: ContractAddress (output, fixed-size)

    let nonce = SHA256Hash::from([0x11; 32]);
    let contract_hash = SHA256Hash::from([0x22; 32]);
    let mut deployed_address = ContractAddress::from([0u8; 32]);

    let result =
        raw_DeployContract(&nonce, &contract_hash, &mut deployed_address);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // DeployContract syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kDeployContract.0);

    // arg1: nonce pointer
    assert_eq!(invocations[0].args[0], nonce.as_ptr() as u64);
    // arg2: contract_hash pointer
    assert_eq!(invocations[0].args[1], contract_hash.as_ptr() as u64);
    // arg3: deployed_address pointer
    assert_eq!(invocations[0].args[2], deployed_address.as_mut_ptr() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[3..], [0; 4]);
}

#[test]
fn test_bytestring_get_len_fixed_size_input() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // BytestringGetLen has:
    // - key: StorageKeyContractData (input, fixed-size)
    // - length: RuntimeMemoryLength (u32, primitive output)

    let key = StorageKeyContractData::from([0xAA; 32]);
    let mut length: RuntimeMemoryLength = 0;

    let result = raw_BytestringGetLen(&key, &mut length);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // BytestringGetLen syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kBytestringGetLen.0);

    // arg1: key pointer
    assert_eq!(invocations[0].args[0], key.as_ptr() as u64);
    // arg2: length pointer
    assert_eq!(invocations[0].args[1], &length as *const _ as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[2..], [0; 5]);
}

#[test]
fn test_bytestring_set_fixed_size_input() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // BytestringSet has:
    // - key: StorageKeyContractData (input, fixed-size)
    // - bytestring: span_argument (input)

    let key = StorageKeyContractData::from([0xBB; 32]);
    let bytestring = b"storage value";

    let result = raw_BytestringSet(&key, bytestring);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // BytestringSet syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kBytestringSet.0);

    // arg1: key pointer
    assert_eq!(invocations[0].args[0], key.as_ptr() as u64);
    // arg2: bytestring pointer
    assert_eq!(invocations[0].args[1], bytestring.as_ptr() as u64);
    // arg3: bytestring length
    assert_eq!(invocations[0].args[2], bytestring.len() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[3..], [0; 4]);
}

#[test]
fn test_get_current_address_fixed_size_output() {
    let handler = Arc::new(MockSyscallHandler::new());
    set_mock_handler(handler.clone());
    handler.set_return_value(SysErr(0));

    // GetCurrentAddress has:
    // - address: ContractAddress (output, fixed-size)

    let mut address = ContractAddress::from([0u8; 32]);

    let result = raw_GetCurrentAddress(&mut address);

    assert!(result.is_ok());

    let invocations = handler.get_invocations();
    assert_eq!(invocations.len(), 1);

    // GetCurrentAddress syscall number
    assert_eq!(invocations[0].syscall_no, SyscallNo::kGetCurrentAddress.0);

    // arg1: address pointer
    assert_eq!(invocations[0].args[0], address.as_mut_ptr() as u64);
    // Remaining args should be 0
    assert_eq!(invocations[0].args[1..], [0; 6]);
}
