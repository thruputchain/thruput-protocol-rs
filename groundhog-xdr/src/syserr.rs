use crate::*;
use core::fmt;

#[cfg_attr(
    feature = "xdrs",
    derive(xdrs::ReadXDR, xdrs::WriteXDR, xdrs::ConstWriteXDR)
)]
#[cfg_attr(feature = "spec", derive(xdrs::XDR))]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[cfg_attr(feature = "std", error("{:?}", self))]
#[repr(transparent)]
pub struct SysErr(pub i32);

impl SysErr {
    pub fn is_code(&self, code: SyscallErrorCode) -> bool {
        self.0 == code.0
    }

    pub fn is_none(&self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeNone)
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    pub fn is_storage_key_not_exist(self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist)
    }

    pub fn is_invalid_input(self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput)
    }
}

impl fmt::Debug for SysErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = SyscallErrorCode(self.0);
        match code {
            SyscallErrorCode::kSyscallErrorCodeNone => {
                write!(f, "SysErr<None>")
            }
            SyscallErrorCode::kSyscallErrorCodeFault => {
                write!(f, "SysErr<Fault>")
            }
            SyscallErrorCode::kSyscallErrorCodeNoSyscall => {
                write!(f, "SysErr<NoSyscall>")
            }
            SyscallErrorCode::kSyscallErrorCodeIsRootTransaction => {
                write!(f, "SysErr<IsRootTransaction>")
            }
            SyscallErrorCode::kSyscallErrorCodeContractAlreadyDeployed => {
                write!(f, "SysErr<ContractAlreadyDeployed>")
            }
            SyscallErrorCode::kSyscallErrorCodeNoReturnBufferAvailable => {
                write!(f, "SysErr<NoReturnBufferAvailable>")
            }
            SyscallErrorCode::kSyscallErrorCodeInvalidSyscallInput => {
                write!(f, "SysErr<InvalidSyscallInput>")
            }
            SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist => {
                write!(f, "SysErr<StorageKeyNotExist>")
            }
            SyscallErrorCode::kSyscallErrorCodeNoRetrocausalQueueEntry => {
                write!(f, "SysErr<NoRetrocausalQueueEntry>")
            }
            _ => write!(f, "SysErr({})", self.0),
        }
    }
}

impl From<SyscallErrorCode> for SysErr {
    fn from(code: SyscallErrorCode) -> Self {
        SysErr(code.0)
    }
}

impl From<SysErr> for i32 {
    fn from(err: SysErr) -> Self {
        err.0
    }
}
