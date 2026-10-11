use crate::*;
use core::fmt;

#[cfg_attr(
    feature = "xdrs",
    derive(
        xdrs::ReadXDR,
        xdrs::WriteXDR,
        xdrs::ConstWriteXDR,
        xdrs::ConstXDRSize
    )
)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[cfg_attr(feature = "std", error("{:?}", self))]
#[repr(transparent)]
/// Raw error code returned across the syscall boundary.
///
/// Wraps the `i32` produced by a syscall; the known values are the
/// [`SyscallErrorCode`] constants, with
/// `kSyscallErrorCodeNone` (0) meaning success. Unknown codes are
/// preserved as-is, so `Debug` falls back to the numeric value.
pub struct SysErr(pub i32);

impl SysErr {
    /// Returns whether this error carries exactly `code`.
    pub fn is_code(&self, code: SyscallErrorCode) -> bool {
        self.0 == code.0
    }

    /// Returns whether this is the success code (`kSyscallErrorCodeNone`).
    pub fn is_none(&self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeNone)
    }

    /// Returns whether this is any code other than success.
    pub fn is_some(&self) -> bool {
        !self.is_none()
    }

    /// Returns whether the syscall failed because the storage key does
    /// not exist (`kSyscallErrorCodeStorageKeyNotExist`).
    pub fn is_storage_key_not_exist(self) -> bool {
        self.is_code(SyscallErrorCode::kSyscallErrorCodeStorageKeyNotExist)
    }

    /// Returns whether the syscall rejected its input
    /// (`kSyscallErrorCodeInvalidSyscallInput`).
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
