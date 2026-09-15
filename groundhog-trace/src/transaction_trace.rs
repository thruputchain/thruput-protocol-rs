// -*-Rust-*-
// Generated Code: DO NOT CHECK IN

// Generated from xdrc commit: 658b4a40c8451ea28d2bddb17f52ee1d5c485c65

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::unnecessary_cast)]

use crate::*;
use core::option::Option;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "std")]
use std::io::{Read, Write};
use xdrs::*;


#[cfg(feature = "serde")]
use ::serde::{{Serialize, Deserialize}};
#[cfg(feature = "serde")]
use ::serde_arrays;




// === Syscall Trace Unions and Structs ===

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256Inputs {
    pub input: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256Outputs {
    pub hash: SHA256Hash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256Trace {
    pub inputs: SyscallSHA256Inputs,
    pub outputs: Option<SyscallSHA256Outputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetMsgSenderInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetMsgSenderOutputs {
    pub sender: ContractAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetMsgSenderTrace {
    pub inputs: SyscallGetMsgSenderInputs,
    pub outputs: Option<SyscallGetMsgSenderOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCalldataLenInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCalldataLenOutputs {
    pub len: u32,
    pub auth_token_count: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCalldataLenTrace {
    pub inputs: SyscallGetCalldataLenInputs,
    pub outputs: Option<SyscallGetCalldataLenOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCalldataInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCalldataOutputs {
    pub calldata: BytesM,
    pub auth_tokens: VecM<u32>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCalldataTrace {
    pub inputs: SyscallGetCalldataInputs,
    pub outputs: Option<SyscallGetCalldataOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallConsoleLogInputs {
    pub buffer: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallConsoleLogOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallConsoleLogTrace {
    pub inputs: SyscallConsoleLogInputs,
    pub outputs: Option<SyscallConsoleLogOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallInvokeInputs {
    pub address: ContractAddress,
    pub calldata: BytesM,
    pub owned_auth_token_ids_list: BytesM,
    pub inputs: CopyableInvokeInputs,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallInvokeOutputs {
    pub child_trace: Option<InvocationFrameTrace>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallInvokeTrace {
    pub inputs: SyscallInvokeInputs,
    pub outputs: Option<SyscallInvokeOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallReturnInputs {
    pub buffer: BytesM,
    pub revert_frame: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallReturnOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallReturnTrace {
    pub inputs: SyscallReturnInputs,
    pub outputs: Option<SyscallReturnOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetReturnedBufferInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetReturnedBufferOutputs {
    pub buffer: ReturnBuffer,
    pub returned_auth_tokens: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetReturnedBufferTrace {
    pub inputs: SyscallGetReturnedBufferInputs,
    pub outputs: Option<SyscallGetReturnedBufferOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallExitInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallExitOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallExitTrace {
    pub inputs: SyscallExitInputs,
    pub outputs: Option<SyscallExitOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetDeployAddressInputs {
    pub caller_address: Option<ContractAddress>,
    pub nonce: SHA256Hash,
    pub contract_hash: SHA256Hash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetDeployAddressOutputs {
    pub to_be_deployed: ContractAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetDeployAddressTrace {
    pub inputs: SyscallGetDeployAddressInputs,
    pub outputs: Option<SyscallGetDeployAddressOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallDeployContractInputs {
    pub nonce: SHA256Hash,
    pub contract_hash: SHA256Hash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallDeployContractOutputs {
    pub deployed_address: ContractAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallDeployContractTrace {
    pub inputs: SyscallDeployContractInputs,
    pub outputs: Option<SyscallDeployContractOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetSourceTransactionHashInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetSourceTransactionHashOutputs {
    pub source_tx_hash: SHA256Hash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetSourceTransactionHashTrace {
    pub inputs: SyscallGetSourceTransactionHashInputs,
    pub outputs: Option<SyscallGetSourceTransactionHashOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallVerifySignatureInputs {
    pub scheme: i32,
    pub signature: BytesM,
    pub public_key: BytesM,
    pub message: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallVerifySignatureOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallVerifySignatureTrace {
    pub inputs: SyscallVerifySignatureInputs,
    pub outputs: Option<SyscallVerifySignatureOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringGetLenInputs {
    pub key: StorageKeyContractData,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringGetLenOutputs {
    pub length: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringGetLenTrace {
    pub inputs: SyscallBytestringGetLenInputs,
    pub outputs: Option<SyscallBytestringGetLenOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringGetInputs {
    pub key: StorageKeyContractData,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringGetOutputs {
    pub bytestring: Bytestring,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringGetTrace {
    pub inputs: SyscallBytestringGetInputs,
    pub outputs: Option<SyscallBytestringGetOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringSetInputs {
    pub key: StorageKeyContractData,
    pub bytestring: BytesM,
    pub flags: StorageDeltaFlagSet,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringSetOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallBytestringSetTrace {
    pub inputs: SyscallBytestringSetInputs,
    pub outputs: Option<SyscallBytestringSetOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallNonnegativeIntGetInputs {
    pub key: StorageKeyContractData,
    pub force_load: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallNonnegativeIntGetOutputs {
    pub current_set: Option<int128_xdr>,
    pub current_add: int128_xdr,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallNonnegativeIntGetTrace {
    pub inputs: SyscallNonnegativeIntGetInputs,
    pub outputs: Option<SyscallNonnegativeIntGetOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallNonnegativeIntSetAddInputs {
    pub key: StorageKeyContractData,
    pub set_value: Option<int128_xdr>,
    pub add_value: int128_xdr,
    pub flags: StorageDeltaFlagSet,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallNonnegativeIntSetAddOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallNonnegativeIntSetAddTrace {
    pub inputs: SyscallNonnegativeIntSetAddInputs,
    pub outputs: Option<SyscallNonnegativeIntSetAddOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallDeleteObjectInputs {
    pub key: StorageKeyContractData,
    pub flags: i32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallDeleteObjectOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallDeleteObjectTrace {
    pub inputs: SyscallDeleteObjectInputs,
    pub outputs: Option<SyscallDeleteObjectOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCurrentAddressInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCurrentAddressOutputs {
    pub address: ContractAddress,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetCurrentAddressTrace {
    pub inputs: SyscallGetCurrentAddressInputs,
    pub outputs: Option<SyscallGetCurrentAddressOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueueInsertInputs {
    pub key: StorageKeyContractData,
    pub queue_entry: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueueInsertOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueueInsertTrace {
    pub inputs: SyscallRetrocausalQueueInsertInputs,
    pub outputs: Option<SyscallRetrocausalQueueInsertOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueueConsumeInputs {
    pub key: StorageKeyContractData,
    pub peek: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueueConsumeOutputs {
    pub queue_entry: QueueEntry,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueueConsumeTrace {
    pub inputs: SyscallRetrocausalQueueConsumeInputs,
    pub outputs: Option<SyscallRetrocausalQueueConsumeOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetTransactionStatisticsInputs {
    pub statistic: TransactionStatistic,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetTransactionStatisticsOutputs {
    pub result: u64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetTransactionStatisticsTrace {
    pub inputs: SyscallGetTransactionStatisticsInputs,
    pub outputs: Option<SyscallGetTransactionStatisticsOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallChangePublicKeyRegistrationInputs {
    pub key_index: u32,
    pub key_scheme: Option<i32>,
    pub key_bytes: BytesM,
    pub key_weight: Option<u32>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallChangePublicKeyRegistrationOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallChangePublicKeyRegistrationTrace {
    pub inputs: SyscallChangePublicKeyRegistrationInputs,
    pub outputs: Option<SyscallChangePublicKeyRegistrationOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallLogEventInputs {
    pub topic0: Option<Topic>,
    pub topic1: Option<Topic>,
    pub topic2: Option<Topic>,
    pub data: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallLogEventOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallLogEventTrace {
    pub inputs: SyscallLogEventInputs,
    pub outputs: Option<SyscallLogEventOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetAuthClaimInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetAuthClaimOutputs {
    pub auth_claim: AuthClaim,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetAuthClaimTrace {
    pub inputs: SyscallGetAuthClaimInputs,
    pub outputs: Option<SyscallGetAuthClaimOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256UpdateInputs {
    pub index: u32,
    pub buffer: BytesM,
    pub reset: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256UpdateOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256UpdateTrace {
    pub inputs: SyscallSHA256UpdateInputs,
    pub outputs: Option<SyscallSHA256UpdateOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256FinalizeInputs {
    pub index: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256FinalizeOutputs {
    pub hash: SHA256Hash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256FinalizeTrace {
    pub inputs: SyscallSHA256FinalizeInputs,
    pub outputs: Option<SyscallSHA256FinalizeOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallWriteAddressRequiredWeightInputs {
    pub required_weight: Weight,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallWriteAddressRequiredWeightOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallWriteAddressRequiredWeightTrace {
    pub inputs: SyscallWriteAddressRequiredWeightInputs,
    pub outputs: Option<SyscallWriteAddressRequiredWeightOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetBlockHeightInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetBlockHeightOutputs {
    pub result: BlockHeight,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetBlockHeightTrace {
    pub inputs: SyscallGetBlockHeightInputs,
    pub outputs: Option<SyscallGetBlockHeightOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueuePeekLenInputs {
    pub key: StorageKeyContractData,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueuePeekLenOutputs {
    pub len: RuntimeMemoryLength,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallRetrocausalQueuePeekLenTrace {
    pub inputs: SyscallRetrocausalQueuePeekLenInputs,
    pub outputs: Option<SyscallRetrocausalQueuePeekLenOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallMintAuthTokenInputs {
    pub expiry_height: BlockHeight,
    pub serialized_auth_token: BytesM,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallMintAuthTokenOutputs {
    pub token_id: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallMintAuthTokenTrace {
    pub inputs: SyscallMintAuthTokenInputs,
    pub outputs: Option<SyscallMintAuthTokenOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallConsumeAuthTokenInputs {
    pub token_id: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallConsumeAuthTokenOutputs {
    pub payload: AuthTokenPayload,
    pub token_minter: ContractAddress,
    pub auth_payload_written_len: RuntimeMemoryLength,
    pub unwrapped_token_id_start: u32,
    pub unwrapped_token_id_count: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallConsumeAuthTokenTrace {
    pub inputs: SyscallConsumeAuthTokenInputs,
    pub outputs: Option<SyscallConsumeAuthTokenOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256CopyInputs {
    pub src_index: u32,
    pub dst_index: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256CopyOutputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallSHA256CopyTrace {
    pub inputs: SyscallSHA256CopyInputs,
    pub outputs: Option<SyscallSHA256CopyOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetNetworkHashInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetNetworkHashOutputs {
    pub network_hash: NetworkHash,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetNetworkHashTrace {
    pub inputs: SyscallGetNetworkHashInputs,
    pub outputs: Option<SyscallGetNetworkHashOutputs>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetBlockTimepointInputs (
);

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetBlockTimepointOutputs {
    pub block_timepoint: TimePoint,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallGetBlockTimepointTrace {
    pub inputs: SyscallGetBlockTimepointInputs,
    pub outputs: Option<SyscallGetBlockTimepointOutputs>,
}

// === Main Trace Body ===

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum SyscallTraceBody {
    // case kSHA256:
    //   SyscallSHA256Trace sha256;
    kSHA256 { sha256: SyscallSHA256Trace } = SyscallNo::kSHA256.0 as i32,
    // case kGetMsgSender:
    //   SyscallGetMsgSenderTrace get_msg_sender;
    kGetMsgSender { get_msg_sender: SyscallGetMsgSenderTrace } = SyscallNo::kGetMsgSender.0 as i32,
    // case kGetCalldataLen:
    //   SyscallGetCalldataLenTrace get_calldata_len;
    kGetCalldataLen { get_calldata_len: SyscallGetCalldataLenTrace } = SyscallNo::kGetCalldataLen.0 as i32,
    // case kGetCalldata:
    //   SyscallGetCalldataTrace get_calldata;
    kGetCalldata { get_calldata: SyscallGetCalldataTrace } = SyscallNo::kGetCalldata.0 as i32,
    // case kConsoleLog:
    //   SyscallConsoleLogTrace console_log;
    kConsoleLog { console_log: SyscallConsoleLogTrace } = SyscallNo::kConsoleLog.0 as i32,
    // case kInvoke:
    //   SyscallInvokeTrace invoke;
    kInvoke { invoke: SyscallInvokeTrace } = SyscallNo::kInvoke.0 as i32,
    // case kReturn:
    //   SyscallReturnTrace return_trace;
    kReturn { return_trace: SyscallReturnTrace } = SyscallNo::kReturn.0 as i32,
    // case kGetReturnedBuffer:
    //   SyscallGetReturnedBufferTrace get_returned_buffer;
    kGetReturnedBuffer { get_returned_buffer: SyscallGetReturnedBufferTrace } = SyscallNo::kGetReturnedBuffer.0 as i32,
    // case kExit:
    //   SyscallExitTrace exit_trace;
    kExit { exit_trace: SyscallExitTrace } = SyscallNo::kExit.0 as i32,
    // case kGetDeployAddress:
    //   SyscallGetDeployAddressTrace get_deploy_address;
    kGetDeployAddress { get_deploy_address: SyscallGetDeployAddressTrace } = SyscallNo::kGetDeployAddress.0 as i32,
    // case kDeployContract:
    //   SyscallDeployContractTrace deploy_contract;
    kDeployContract { deploy_contract: SyscallDeployContractTrace } = SyscallNo::kDeployContract.0 as i32,
    // case kGetSourceTransactionHash:
    //   SyscallGetSourceTransactionHashTrace get_source_transaction_hash;
    kGetSourceTransactionHash { get_source_transaction_hash: SyscallGetSourceTransactionHashTrace } = SyscallNo::kGetSourceTransactionHash.0 as i32,
    // case kVerifySignature:
    //   SyscallVerifySignatureTrace verify_signature;
    kVerifySignature { verify_signature: SyscallVerifySignatureTrace } = SyscallNo::kVerifySignature.0 as i32,
    // case kBytestringGetLen:
    //   SyscallBytestringGetLenTrace bytestring_get_len;
    kBytestringGetLen { bytestring_get_len: SyscallBytestringGetLenTrace } = SyscallNo::kBytestringGetLen.0 as i32,
    // case kBytestringGet:
    //   SyscallBytestringGetTrace bytestring_get;
    kBytestringGet { bytestring_get: SyscallBytestringGetTrace } = SyscallNo::kBytestringGet.0 as i32,
    // case kBytestringSet:
    //   SyscallBytestringSetTrace bytestring_set;
    kBytestringSet { bytestring_set: SyscallBytestringSetTrace } = SyscallNo::kBytestringSet.0 as i32,
    // case kNonnegativeIntGet:
    //   SyscallNonnegativeIntGetTrace nonnegative_int_get;
    kNonnegativeIntGet { nonnegative_int_get: SyscallNonnegativeIntGetTrace } = SyscallNo::kNonnegativeIntGet.0 as i32,
    // case kNonnegativeIntSetAdd:
    //   SyscallNonnegativeIntSetAddTrace nonnegative_int_set_add;
    kNonnegativeIntSetAdd { nonnegative_int_set_add: SyscallNonnegativeIntSetAddTrace } = SyscallNo::kNonnegativeIntSetAdd.0 as i32,
    // case kDeleteObject:
    //   SyscallDeleteObjectTrace delete_object;
    kDeleteObject { delete_object: SyscallDeleteObjectTrace } = SyscallNo::kDeleteObject.0 as i32,
    // case kGetCurrentAddress:
    //   SyscallGetCurrentAddressTrace get_current_address;
    kGetCurrentAddress { get_current_address: SyscallGetCurrentAddressTrace } = SyscallNo::kGetCurrentAddress.0 as i32,
    // case kRetrocausalQueueInsert:
    //   SyscallRetrocausalQueueInsertTrace retrocausal_queue_insert;
    kRetrocausalQueueInsert { retrocausal_queue_insert: SyscallRetrocausalQueueInsertTrace } = SyscallNo::kRetrocausalQueueInsert.0 as i32,
    // case kRetrocausalQueueConsume:
    //   SyscallRetrocausalQueueConsumeTrace retrocausal_queue_consume;
    kRetrocausalQueueConsume { retrocausal_queue_consume: SyscallRetrocausalQueueConsumeTrace } = SyscallNo::kRetrocausalQueueConsume.0 as i32,
    // case kGetTransactionStatistics:
    //   SyscallGetTransactionStatisticsTrace get_transaction_statistics;
    kGetTransactionStatistics { get_transaction_statistics: SyscallGetTransactionStatisticsTrace } = SyscallNo::kGetTransactionStatistics.0 as i32,
    // case kChangePublicKeyRegistration:
    //   SyscallChangePublicKeyRegistrationTrace change_public_key_registration;
    kChangePublicKeyRegistration { change_public_key_registration: SyscallChangePublicKeyRegistrationTrace } = SyscallNo::kChangePublicKeyRegistration.0 as i32,
    // case kLogEvent:
    //   SyscallLogEventTrace log_event;
    kLogEvent { log_event: SyscallLogEventTrace } = SyscallNo::kLogEvent.0 as i32,
    // case kGetAuthClaim:
    //   SyscallGetAuthClaimTrace get_auth_claim;
    kGetAuthClaim { get_auth_claim: SyscallGetAuthClaimTrace } = SyscallNo::kGetAuthClaim.0 as i32,
    // case kSHA256Update:
    //   SyscallSHA256UpdateTrace sha256_update;
    kSHA256Update { sha256_update: SyscallSHA256UpdateTrace } = SyscallNo::kSHA256Update.0 as i32,
    // case kSHA256Finalize:
    //   SyscallSHA256FinalizeTrace sha256_finalize;
    kSHA256Finalize { sha256_finalize: SyscallSHA256FinalizeTrace } = SyscallNo::kSHA256Finalize.0 as i32,
    // case kWriteAddressRequiredWeight:
    //   SyscallWriteAddressRequiredWeightTrace write_address_required_weight;
    kWriteAddressRequiredWeight { write_address_required_weight: SyscallWriteAddressRequiredWeightTrace } = SyscallNo::kWriteAddressRequiredWeight.0 as i32,
    // case kGetBlockHeight:
    //   SyscallGetBlockHeightTrace get_block_height;
    kGetBlockHeight { get_block_height: SyscallGetBlockHeightTrace } = SyscallNo::kGetBlockHeight.0 as i32,
    // case kRetrocausalQueuePeekLen:
    //   SyscallRetrocausalQueuePeekLenTrace retrocausal_queue_peek_len;
    kRetrocausalQueuePeekLen { retrocausal_queue_peek_len: SyscallRetrocausalQueuePeekLenTrace } = SyscallNo::kRetrocausalQueuePeekLen.0 as i32,
    // case kMintAuthToken:
    //   SyscallMintAuthTokenTrace mint_auth_token;
    kMintAuthToken { mint_auth_token: SyscallMintAuthTokenTrace } = SyscallNo::kMintAuthToken.0 as i32,
    // case kConsumeAuthToken:
    //   SyscallConsumeAuthTokenTrace consume_auth_token;
    kConsumeAuthToken { consume_auth_token: SyscallConsumeAuthTokenTrace } = SyscallNo::kConsumeAuthToken.0 as i32,
    // case kSHA256Copy:
    //   SyscallSHA256CopyTrace sha256_copy;
    kSHA256Copy { sha256_copy: SyscallSHA256CopyTrace } = SyscallNo::kSHA256Copy.0 as i32,
    // case kGetNetworkHash:
    //   SyscallGetNetworkHashTrace get_network_hash;
    kGetNetworkHash { get_network_hash: SyscallGetNetworkHashTrace } = SyscallNo::kGetNetworkHash.0 as i32,
    // case kGetBlockTimepoint:
    //   SyscallGetBlockTimepointTrace get_block_timepoint;
    kGetBlockTimepoint { get_block_timepoint: SyscallGetBlockTimepointTrace } = SyscallNo::kGetBlockTimepoint.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { syscall_number: i32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[repr(i32)]
#[non_exhaustive]
pub enum HostFunctionStatusXdr {
    // case NoneOrRecoverable:
    //   SyscallErrorCode syscall_return_code;
    NoneOrRecoverable { syscall_return_code: SyscallErrorCode } = HostFnErrorCode::NoneOrRecoverable.0 as i32,
    // default:
    //   void;
    #[xdr(default_variant)] Default { error: i32 },
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct SyscallTrace {
    pub gas_remaining_at_start: GasAmount,
    pub gas_remaining_at_end: GasAmount,
    pub body: SyscallTraceBody,
    pub status: HostFunctionStatusXdr,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(ReadXDR, WriteXDR, ConstXDRSize)]
#[derive(schemars::JsonSchema)]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(C)]
pub struct InvocationFrameTrace {
    pub invocation: Invocation,
    /// The ordered list of system calls made by this frame.
    pub system_calls: VecM<SyscallTrace>,
    /// The set of storage writes made by this frame,
    /// when the frame terminates.
    /// Note that this field may be nonempty even if the frame Faults or Aborts;
    /// In such a case, these writes will not be applied to the parent frame,
    /// but are still reported in the trace for testing purposes.
    pub storage_writes: VecM<FlaggedStorageDelta>,
    /// Status of the frame when it terminates.
    pub status: InvocationFrameOutput,
    /// Two-phase-commit conflict diagnostic for a try_invoke frame whose
    /// subtransaction push failed (proposal mode). The kDbDiagnosticNone arm on
    /// every other frame. Best-effort SDK-testing diagnostic; see
    /// SystemCallHandler::finish_frame.
    pub commit_conflict: DbConflictResult,
}


//
// XDR internals below this point
//

impl SyscallTraceBody {
    pub fn syscall_number(&self) -> SyscallNo {
        match self {
            SyscallTraceBody::Default{ syscall_number } => (*syscall_number).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}

impl HostFunctionStatusXdr {
    pub fn error(&self) -> HostFnErrorCode {
        match self {
            HostFunctionStatusXdr::Default{ error } => (*error).into(),

            _ => (unsafe { *(self as *const Self as *const i32) }).into(),
        }
    }
}


